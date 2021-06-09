#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

#[repr(C)]
pub struct RegisterBlock {
    ctrl: Volatile<u32>,
    load: Volatile<u32>,
    reason: Volatile<u32>,
    clk: [Volatile<u32>; 8],
    tick: Volatile<u32>,
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

    pub const PTR: *mut self::RegisterBlock = super::WATCHDOG_BASE as *mut _;

    #[inline(never)]
    pub fn enable_tick(&mut self, cycles: u32) {
        self.tick.write(cycles | defs::WATCHDOG_TICK_ENABLE_BITS );
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