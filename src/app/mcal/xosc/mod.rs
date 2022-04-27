#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

#[repr(C)]
pub struct RegisterBlock {
    ctrl: Volatile<u32>,
    status: Volatile<u32>,
    dormant: Volatile<u32>,
    startup: Volatile<u32>,
    _reserved: [Volatile<u32>; 3],
    count: Volatile<u32>,
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const PTR: *mut self::RegisterBlock = super::XOSC_BASE as *mut _;

    #[inline(never)]
    pub fn init(&mut self) {
        // Assumes 1-15 MHz input
        self.ctrl.write(defs::XOSC_CTRL_FREQ_RANGE_VALUE_1_15MHZ);
        // Set xosc startup delay
        self.startup.write((((super::XOSC_MHZ * super::MHZ) / 1000) + 128) / 256);
        // Set the enable bit now that we have set freq range and startup delay
        self.ctrl.write(defs::XOSC_CTRL_FREQ_RANGE_VALUE_1_15MHZ |
                        defs::XOSC_CTRL_ENABLE_VALUE_ENABLE << defs::XOSC_CTRL_ENABLE_LSB);
        // Wait for XOSC to be stable
        while 0 != (self.status.read() & defs::XOSC_STATUS_STABLE_BITS){}
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