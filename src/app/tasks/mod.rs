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
  pub body: fn(),
  pub stack: &'a mut [u32],
  pub stacksize: usize,
  pub state: TaskState,
  pub priority: u32, //Higher is more prioritary
}

impl<'a> Task<'a> {
  pub fn loopy() -> ! {
    loop {}
  }
  pub fn new(
    name: &'static str,
    body: fn(),
    stack: &'a mut [u32],
    stacksize: usize,
    priority: u32,
  ) -> Self {
    // Check the alignment
    if stacksize % 4 != 0 {
      panic!("Stack sizes need to be aligned to 4")
    }

    // Init the stack with the initial context
    stack[stacksize - 1] = 0x01000000;               // Initial XPSR: Thumb set, no exception
    stack[stacksize - 2] = body as u32 | 0x1;        // Initial PC: Task body
    stack[stacksize - 3] = Self::loopy as u32 | 0x1; // Initial LR: Function that will be called when the task finish
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
      priority: priority,
    }
  }
}

//TODO: Move to proper unit
pub struct SchedTable<'a> {
  pub macroperiod: u64,                // in us
  pub tasks: &'a mut [Task<'a>],
  pub schedpoints: &'a [(u64, usize)], // Currently only one task is enabled at each sched point
}
//pub struct Scheduler<'a> {
//    pub coresched: [SchedTable<'a>; super::mcal::NUM_CORES as usize],
//}

extern "C" {
  #[cfg(armv6m)]
  fn ctxtswtr();
}

static mut C0ST: Option<(usize, usize)> = None;
static mut C1ST: Option<(usize, usize)> = None;

#[no_mangle]
#[inline(never)]
pub extern "C" fn alarmhandler(coreidx: u32, psp: usize, fromusermode: bool) -> usize {
  unsafe {
    // Clear the alarm first
    let mut timer = super::mcal::timer::Peripheral::new();
    timer.clear_alarm(super::mcal::timer::AlarmId::Alarm0);

    let (corest, alarm) = match coreidx {
      0 => (C0ST, super::mcal::timer::AlarmId::Alarm0),
      1 => (C1ST, super::mcal::timer::AlarmId::Alarm1),
      _ => unreachable!(),
    };

    if let Some((stptr, currentidx)) = corest {
      // Reborrow to deref raw ptr and get mutable reference to the sched table
      let schedtab = &mut *(stptr as *mut SchedTable);
      let schedtablen = schedtab.schedpoints.len();
      let nextidx = (currentidx + 1) % schedtablen;
      C0ST = Some((stptr, nextidx));

      // Configure the next alarm
      {
        let delta = 
        if schedtab.schedpoints[nextidx].0 > schedtab.schedpoints[currentidx].0 {
          schedtab.schedpoints[nextidx].0 - schedtab.schedpoints[currentidx].0
        }
        else {
          schedtab.macroperiod - schedtab.schedpoints[currentidx].0
        };

        timer.set_alarm_relative(
          alarm,
          delta as u32,
          ctxtswtr,
        );
      }
      // Save the context
      {
        schedtab.tasks[schedtab.schedpoints[currentidx].1].regs.sp = psp;
        schedtab.tasks[schedtab.schedpoints[currentidx].1].state = TaskState::Ready;
      }
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

impl<'a> SchedTable<'a> {
  pub fn start(self) -> ! {
    // Initialize task ids
    for (idx, task) in self.tasks.iter_mut().enumerate() {
      task.id = Some(idx);
      task.state = TaskState::Ready;
    }
    // Launch the sched table
    unsafe {
      let mut timer = super::mcal::timer::Peripheral::new();
      let coreid = super::mcal::sio::Peripheral::new().get_core_num();
      let alarm: super::mcal::timer::AlarmId = match coreid {
        0 => super::mcal::timer::AlarmId::Alarm0,
        1 => super::mcal::timer::AlarmId::Alarm1,
        _ => panic!("Only two cores are supported"),
      };

      if coreid == 0 {
        C0ST = Some(((&self as *const Self) as usize, 0));
      } else {
        loop {} /*TODO: Enable the second core only if the 1st core is working properly*/
      }

      // Reset the time just before starting the scheduler
      timer.reset_time();
      // Trigger the first alarm by hand!
      timer.set_alarm_relative(alarm, self.schedpoints[1].0 as u32, ctxtswtr);

      // Setup process stack and user mode right before switching to task
      super::mcal::intrinsics::launch(self.tasks[0].body, self.tasks[0].regs.sp + 64);
      
      loop {} // This wont ever be used but the compiler needs to know its divergent
    }
  }
}
