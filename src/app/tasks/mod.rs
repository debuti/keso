#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum TaskState {
  Inactive,
  Waiting,
  Ready,
  Running,
  Halted
}

pub struct Registers {
  pub sp: usize,
  pub r4: u32,
}

pub struct Task<'a> {
  pub regs: Registers,
  pub id: u32,
  pub name: &'static str,
  pub body: fn(),
  pub stack: &'a mut [u32],
  pub stacksize: usize,
  pub state: TaskState,
  pub priority: u32, //Higher is more prioritary
}

impl<'a> Task<'a> {
  pub fn new(name: &'static str, body: fn(), stack: &'a mut [u32], stacksize: usize, priority: u32) -> Self {
    // Check the alignment
    if stacksize % 4 != 0 {panic!("Stack sizes need to be aligned to 4")}

    // Init the stack with the initial context
    stack[stacksize-1] = 0x01000000;        // Initial XPSR: Thumb set, no exception
    stack[stacksize-2] = body as u32;       // Initial PC: Task body  //TODO: Set LSB to 1 so thumb
    stack[stacksize-3] = 0x0;               // Initial LR: Function that will be called when the task finish //TODO: Set this
    stack[stacksize-4] = 0x0;               // Initial R12
    stack[stacksize-5] = 0x0;               // Initial R3
    stack[stacksize-6] = 0x0;               // Initial R2
    stack[stacksize-7] = 0x0;               // Initial R1
    stack[stacksize-8] = 0x0;               // Initial R0

    // Return the handle
    Self {
      regs: Registers {
        sp: stack[stacksize-8] as *mut usize as usize,
        r4: 0,
      },
      id: 0, //TODO: Take this value automatically
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
  pub macroperiod: u64,                       // in us
  pub schedpoints: &'a mut [(u64, Task<'a>)], // Currently only one task is enabled at each sched point
}
//pub struct Scheduler<'a> {
//    pub coresched: [SchedTable<'a>; super::mcal::NUM_CORES as usize],
//}

  
static mut C0ST: Option<usize> = None;


#[no_mangle]
#[inline(never)]
pub fn alarm0handler() {
  //TODO: Implement this once. Maybe reading XPSR to see the actual exc being handled
  unsafe {
    let mut timer = super::mcal::timer::Peripheral::new();
    let time = timer.get_time();
    timer.clear_alarm(super::mcal::timer::AlarmId::Alarm0);

    if let Some(stptr) = C0ST {
      // Reborrow to deref raw ptr and get mutable reference to the sched table
      let schedtab = &mut *(stptr as *mut SchedTable);
      
      // Guess which was the fired schedule point
      let t = time % schedtab.macroperiod;

      for idx in (0..schedtab.schedpoints.len()).rev() {
        if t >= schedtab.schedpoints[idx].0 {
          // Configure the next alarm
          {
            timer.set_alarm_relative(super::mcal::timer::AlarmId::Alarm0, schedtab.schedpoints[idx+1].0 as u32, alarm0handler); 
          }
          // Save the context
          {
            schedtab.schedpoints[idx-1].1.regs.sp = super::mcal::intrinsics::getpsp();
            schedtab.schedpoints[idx-1].1.regs.r4 = super::mcal::intrinsics::getrx(4);
            //TODO: Retrieve and save the rest of the registers
          }
          // Run the appropiate task          
          {
            super::mcal::intrinsics::setpsp(schedtab.schedpoints[idx].1.regs.sp);
            super::mcal::intrinsics::setrx(4, schedtab.schedpoints[idx].1.regs.r4);
            
            //TODO: Restore the rest of the registers
          }
        }
      }
        
      // TEST THIS
      // CHECK THAT PSP IS PROPERLY SET AFTER THIS!!
    } 
    else {
      panic!("Timer shouldn't be firing without schedule table"); // How the fuck the timer is running without 
    }
  }
}

#[no_mangle]
#[inline(never)]
pub fn alarm1handler() {
  unsafe {
    let mut timer = super::mcal::timer::Peripheral::new();
    timer.clear_alarm(super::mcal::timer::AlarmId::Alarm0);
        // Run the appropiate task
 }
}


impl<'a> SchedTable<'a> {
  pub fn start(self) -> ! {

    unsafe {
      let mut timer = super::mcal::timer::Peripheral::new();
      let coreid = super::mcal::sio::Peripheral::new().get_core_num();
      let (_alarm, _handler): (super::mcal::timer::AlarmId, fn()) = 
        match coreid {
          0 => (super::mcal::timer::AlarmId::Alarm0, alarm0handler),
          1 => (super::mcal::timer::AlarmId::Alarm1, alarm1handler),
          _ => panic!("Only two cores are supported")
        };
      if coreid == 0 {C0ST = Some((&self as *const Self) as usize);}
      // Reset the time just before starting the scheduler
      timer.reset_time();
      //TODO: Trigger the first alarm by hand!
      //timer.set_alarm_relative(alarm, self.schedpoints[0].0, handler); 
    }

    // Setup process stack and user mode right after switching to task
    super::mcal::intrinsics::setcontrol(false, true); 

    loop {}
  }
}