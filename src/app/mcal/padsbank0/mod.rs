#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

#[repr(C)]
pub struct RegisterBlock {
    voltage_select: Volatile<u32>,
    io: [Volatile<u32>; 30],
    swclk: Volatile<u32>,
    swd: Volatile<u32>,
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

    pub const PTR: *mut self::RegisterBlock = super::PADS_BANK0_BASE as *mut _;

    #[inline(never)]
    pub fn writeio(&mut self, idx: usize, value: u32) {
        self.io[idx].write(value)
    }

    #[inline(never)]
    pub fn readio(&mut self, idx: usize) -> u32 {
        self.io[idx].read()
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