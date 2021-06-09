#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

#[derive(Clone, Copy)]
pub enum ResetDevice {
    Adc = 0, //(0)
    BusCtrl, //(1)
    Dma, //(2)
    I2c0, //(3)
    I2c1, //(4)
    IoBank0, //(5)
    IoQspi, //(6)
    Jtag, //(7)
    PadsBank0, //(8)
    PadsQspi, //(9)
    Pio0, //(10)
    Pio1, //(11)
    PllSys, //(12)
    PllUsb, //(13)
    Pwm, //(14)
    Rtc, //(15)
    Spi0, //(16)
    Spi1, //(17)
    SysCfg, //(18)
    Sysinfo, //(19)
    Tbman, //(20)
    Timer, //(21)
    Uart0, //(22)
    Uart1, //(23)
    UsbCtrl, //(24)
}

#[repr(C)]
pub struct RegisterBlock {
    reset: Volatile<u32>,
    wdsel: Volatile<u32>,
    reset_done: Volatile<u32>,
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

    pub const PTR: *mut self::RegisterBlock = super::RESETS_BASE as *mut _;

    #[inline(never)]
    pub fn disable(&mut self, rd: ResetDevice) {
        let t: u32 = self.reset.read();
        self.reset.write(t | 1 << (rd as u32));
    }

    #[inline(never)]
    pub fn enable(&mut self, rd: ResetDevice) {
        let t: u32 = self.reset.read();
        self.reset.write(t & !(1 << (rd as u32)));
    }

    #[inline(never)]
    pub fn enable_wait(&mut self, rd: ResetDevice) {
        let rd_ = rd as u32;
        self.enable(rd);
        while !self.reset_done.read() & 1 << rd_ != 0 {/*NOP*/}
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