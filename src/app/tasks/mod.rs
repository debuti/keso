#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum TaskState {
  Inactive,
  Waiting,
  Ready,
  Running,
  Halted,
}

type TaskId = usize;
pub struct Registers {
  pub sp: usize,
}

pub struct Task<'a> {
  pub regs: Registers,
  pub id: Option<TaskId>,
  pub name: &'static str,
  pub body: fn() -> !,
  pub stack: &'a mut [u32],
  pub stacksize: usize,
  pub state: TaskState,
}

impl<'a> Task<'a> {
  pub const STACK_CANARY: u32 = 0xCACAC0C0;
  pub fn finishjob() {
    // Tell the kernel that we finished this iteration
    unsafe {core::arch::asm!("svc #1");}
    
    // Wait for the scheduler here. It has to be here on userland, and it has to be exactly one instruction
    // By using inline assembly we make sure the loop {} is not optimized out
    unsafe {core::arch::asm!("b .");} 
  }
  pub fn loopy() -> ! {
    loop {}
  }
  pub fn new(
    name: &'static str,
    body: fn() -> !,
    stack: &'a mut [u32],
    stacksize: usize,
  ) -> Self {
    // Check the alignment and minimum size
    if stacksize % 4 != 0 || stacksize < 32 {
      panic!("Stack sizes need to be aligned to 4")
    }

    // Set up the stack canary
    stack[0] = Self::STACK_CANARY;

    // Init the stack with the initial context
    stack[stacksize - 1] = 0x01000000;               // Initial XPSR: Thumb set, no exception
    stack[stacksize - 2] = body as u32 | 0x1;        // Initial PC: Task body
    stack[stacksize - 3] = Self::loopy as u32 | 0x1; // Initial LR: Function that will be called when the task finishes (it shouldnt)
    stack[stacksize - 4] = 0x12;                     // Initial R12
    stack[stacksize - 5] = 0x3;                      // Initial R3
    stack[stacksize - 6] = 0x2;                      // Initial R2
    stack[stacksize - 7] = 0x1;                      // Initial R1
    stack[stacksize - 8] = 0x0;                      // Initial R0
    stack[stacksize - 9] = 0x11;                     // Initial R11
    stack[stacksize - 10] = 0x10;                    // Initial R10
    stack[stacksize - 11] = 0x9;                     // Initial R9
    stack[stacksize - 12] = 0x8;                     // Initial R8
    stack[stacksize - 13] = 0x7;                     // Initial R7
    stack[stacksize - 14] = 0x6;                     // Initial R6
    stack[stacksize - 15] = 0x5;                     // Initial R5
    stack[stacksize - 16] = 0x4;                     // Initial R4

    // Return the handle
    Self {
      regs: Registers {
        sp: &stack[stacksize - 16] as *const u32 as usize,
      },
      id: None,
      name: name,
      body: body,
      stack: stack,
      stacksize: stacksize,
      state: TaskState::Waiting,
    }
  }
}

//TODO: Move to proper unit
pub struct SchedTable<'a> {
  pub macroperiod: u64,                // in us
  pub tasks: &'a mut [Task<'a>],
  pub schedpoints: &'a [(u64, TaskId)], // Currently only one task is enabled at each sched point
}


extern "C" {
  #[cfg(armv6m)]
  fn ctxtswtr();
  #[cfg(armv6m)]
  fn svclanding();
}

#[derive(Clone, Copy)]
pub struct CoreObj {
  stptr: usize,
  currentidx: usize,
  finish: bool,
}

static mut KERNEL_OBJ: [Option<CoreObj>; super::mcal::NUM_CORES as usize] = [None; super::mcal::NUM_CORES as usize];

#[no_mangle]
#[inline(never)]
pub extern "C" fn alarmhandler(sp: usize, fromusermode: bool) -> usize {
    let coreid = super::mcal::sio::Peripheral::new().get_core_num();
    let mut timer = super::mcal::timer::Peripheral::new();

    let alarm = match coreid {
      0 => super::mcal::timer::AlarmId::Alarm0,
      1 => super::mcal::timer::AlarmId::Alarm1,
      _ => unreachable!(),
    };

    // Clear the alarm first
    timer.clear_alarm(alarm);

  unsafe {
    if let Some(cobj) = &mut KERNEL_OBJ[coreid as usize] {
      // Reborrow to deref raw ptr and get mutable reference to the sched table
      let schedtab = &mut *(cobj.stptr as *mut SchedTable);
      let nextidx = (cobj.currentidx + 1) % schedtab.schedpoints.len();

      // Configure the next alarm
      {
        let delta = 
          if schedtab.schedpoints[nextidx].0 > schedtab.schedpoints[cobj.currentidx].0 {
            schedtab.schedpoints[nextidx].0 - schedtab.schedpoints[cobj.currentidx].0
          }
          else {
            schedtab.macroperiod - schedtab.schedpoints[cobj.currentidx].0
          };

        timer.set_alarm_relative(
          alarm,
          delta as u32,
          ctxtswtr,
        );
      }
      // Save the context
      if fromusermode {
      
        // Check the stack canary
        {
          if schedtab.tasks[schedtab.schedpoints[cobj.currentidx].1].stack[0] != Task::STACK_CANARY 
          {
            super::mcal::uart::Peripheral::new(super::mcal::uart::Uart::Uart0).puts("Task stack overflow!\n\r");

            //TODO: Use a callback to user code to signal this FatalError
            //TODO: Also, the other cores should be notified and stopped
            loop {}
          }
        }

        schedtab.tasks[schedtab.schedpoints[cobj.currentidx].1].regs.sp = sp;
        schedtab.tasks[schedtab.schedpoints[cobj.currentidx].1].state = TaskState::Ready;
      
        if !cobj.finish {
          // Oh oh the task wasnt finished on time :(
  
          let mut uart = super::mcal::uart::Peripheral::new(super::mcal::uart::Uart::Uart0);
          match coreid {
            0 => uart.puts("C0 finished abruptly!\n\r"),
            1 => uart.puts("C1 finished abruptly!\n\r"),
            _ => unreachable!(),
          };

          //TODO: Use a callback to user code to signal this FatalError
          //TODO: Also, the other cores should be notified and stopped
          loop {}
        }
      }

      cobj.finish = false;
      cobj.currentidx = nextidx;

      // Run the appropiate task
      {
        schedtab.tasks[schedtab.schedpoints[nextidx].1].state = TaskState::Running;
        return schedtab.tasks[schedtab.schedpoints[nextidx].1].regs.sp;
      }
    } else {
      panic!("Timer shouldn't be firing without schedule table"); // How the fuck the timer is running without
    }
  }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn svchandler(syscall: u32) {
  fn iterationend(coreid: u32) {
    // Signal iteration end
    unsafe {
      if let Some(cobj) = &mut KERNEL_OBJ[coreid as usize] {
        let schedtab = &mut *(cobj.stptr as *mut SchedTable);
        schedtab.tasks[schedtab.schedpoints[cobj.currentidx].1].state = TaskState::Waiting;
        cobj.finish = true;
      }
    }
  }

  let coreid = super::mcal::sio::Peripheral::new().get_core_num();
  match syscall {
    1 => {iterationend(coreid)},
    _ => {panic!("Unknown syscall")},
  }
}

impl<'a> SchedTable<'a> {
  pub fn start(self) -> ! {
    // Initialize task ids
    for (idx, task) in self.tasks.iter_mut().enumerate() {
      task.id = Some(idx);
      task.state = TaskState::Ready;
    }
    // Setup the svc call
    {
      let mut cm0p = super::mcal::cm0p::Peripheral::new();
      let irqid = super::mcal::cm0p::IrqId::Svc;
      cm0p.irq_set_exclusive_handler(irqid, svclanding);
    }
    // Launch the sched table
    unsafe {
      let mut timer = super::mcal::timer::Peripheral::new();
      let coreid = super::mcal::sio::Peripheral::new().get_core_num();
      let alarm = match coreid {
        0 => super::mcal::timer::AlarmId::Alarm0,
        1 => super::mcal::timer::AlarmId::Alarm1,
        _ => unreachable!(),
      };

      // Provide the last item so the 1st task is the schedtable entry zero
      KERNEL_OBJ[coreid as usize] = Some(CoreObj{stptr:      (&self as *const Self) as usize,
                                                currentidx: self.schedpoints.len() - 1,
                                                finish:     false});

      // Reset the time just before starting the scheduler
      if coreid == 0 { timer.reset_time(); }
      // Trigger the first alarm by hand!
      timer.set_alarm_relative(alarm, self.schedpoints[1].0 as u32, ctxtswtr);

      loop {} // This wont ever be used but the compiler needs to know its divergent
    }
  }
}
