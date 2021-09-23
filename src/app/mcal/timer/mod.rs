#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

pub const NUM_TIMERS: usize = 4;


#[derive(Copy, Clone)]
pub enum AlarmId {
    Alarm0 = 0,
    Alarm1 = 1,
    Alarm2 = 2,
    Alarm3 = 3,
}

#[repr(C)]
pub struct RegisterBlock {
    timehw: Volatile<u32>,
    timelw: Volatile<u32>,
    timehr: Volatile<u32>,
    timelr: Volatile<u32>,
    alarm: [Volatile<u32>; NUM_TIMERS],
    armed: Volatile<u32>,
    timerawh: Volatile<u32>,
    timerawl: Volatile<u32>,
    dbgpause: Volatile<u32>,
    pause: Volatile<u32>,
    intr: Volatile<u32>,
    inte: Volatile<u32>,
    intf: Volatile<u32>,
    ints: Volatile<u32>,
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const unsafe fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const PTR: *mut self::RegisterBlock = super::TIMER_BASE as *mut _;

    #[inline(never)]
    pub fn get_time(&self) -> u64 {
        (self.timelr.read() as u64)|((self.timehr.read() as u64)<<32)
    }

    #[inline(never)]
    pub fn reset_time(&mut self) {
        self.timelw.write(0);
        self.timehw.write(0);
    }

    #[inline(never)]
    /**
     * The timer has 4 alarms, and outputs a separate interrupt for each alarm. 
     * The alarms match on the lower 32 bits of the 64-bit counter which means they
     * can be fired at a maximum of 2 32 microseconds into the future. This is equivalent to:
     *    2^32 ÷ 10 = ~4295 seconds = ~72 minutes
     */
    pub fn set_alarm_relative(&mut self, idx: AlarmId, delayus: u32, handler: fn()) {
        // Enable the interrupt for our alarm (the timer outputs 4 alarm irqs)
        let t = self.inte.read();
        self.inte.write(t | 1<<idx as usize);
        // Set irq handler for alarm irq
        let irqid = match idx {
            AlarmId::Alarm0 => super::cm0p::IrqId::Timer0,
            AlarmId::Alarm1 => super::cm0p::IrqId::Timer1,
            AlarmId::Alarm2 => super::cm0p::IrqId::Timer2,
            AlarmId::Alarm3 => super::cm0p::IrqId::Timer3,
        };
        unsafe {
            let mut cm0p = super::cm0p::Peripheral::new();
            cm0p.irq_set_exclusive_handler(irqid, handler);
            // Enable the alarm irq
            cm0p.irq_set_enabled(irqid, true);
        }
        // Alarm is only 32 bits so if trying to delay more
        // than that need to be careful and keep track of the upper
        // bits
        let target = self.timerawl.read() + delayus;
        // Write the lower 32 bits of the target time to the alarm which
        // will arm it
        self.alarm[idx as usize].write(target);
    }

    #[inline(never)]
    pub fn clear_alarm(&mut self, idx: AlarmId){
        // Raw interrupts (W1C)
        let t = self.intr.read();
        self.intr.write(t | 1<<idx as usize);
        //
        unsafe {
            let irqid = match idx {
                AlarmId::Alarm0 => super::cm0p::IrqId::Timer0,
                AlarmId::Alarm1 => super::cm0p::IrqId::Timer1,
                AlarmId::Alarm2 => super::cm0p::IrqId::Timer2,
                AlarmId::Alarm3 => super::cm0p::IrqId::Timer3,
            };
            let mut cm0p = super::cm0p::Peripheral::new();
            // Disable the alarm irq
            cm0p.irq_set_enabled(irqid, false);
        }
        // Disable the interrupt for our alarm
        let t = self.inte.read();
        self.inte.write(t & !(1<<idx as usize));
    }

    #[inline(never)]
    pub fn delay(&self, delayus: u32) {
        let target = self.get_time() + delayus as u64;
        loop {
            if self.get_time() > target {return;}
        }
    }

    #[inline(never)]
    pub fn delay_nops(nops: u32) {
        for _ in 0..nops {
            crate::app::mcal::intrinsics::nop();
        }
    }
}

impl ops::Deref for Peripheral {
    type Target = self::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl ops::DerefMut for Peripheral {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *Self::PTR }
    }
}