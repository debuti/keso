#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

pub enum Pll {
    Sys,
    Usb,
}

#[repr(C)]
pub struct RegisterBlock {
    cs: Volatile<u32>,
    pwr: Volatile<u32>,
    fbdiv_int: Volatile<u32>,
    prim: Volatile<u32>,
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
    sel : Pll,
}

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const fn new(sel:Pll) -> Self {
        Self {
            _marker: PhantomData,
            sel: sel,
        }
    }

    pub const PTR_SYS: *mut self::RegisterBlock = super::PLL_SYS_BASE as *mut _;
    pub const PTR_USB: *mut self::RegisterBlock = super::PLL_USB_BASE as *mut _;

    #[inline(never)]
    pub fn init(&mut self, refdiv:u32, vco_freq:u32, post_div1:u32, post_div2:u32) {
        // Turn off PLL in case it is already running
        self.pwr.write(0xffffffff);
        self.fbdiv_int.write(0x0);
        let ref_mhz = super::XOSC_MHZ / refdiv;
        self.cs.write(refdiv);
        // What are we multiplying the reference clock by to get the vco freq
        // (The regs are called div, because you divide the vco output and compare it to the refclk)
        let fbdiv = vco_freq / (ref_mhz * super::MHZ);
        // Put calculated value into feedback divider
        self.fbdiv_int.write(fbdiv);
        // Turn on PLL
        let power = defs::PLL_PWR_PD_BITS | // Main power
                    defs::PLL_PWR_VCOPD_BITS; // VCO Power
        {
            let t = self.pwr.read();
            self.pwr.write(t & !power);
        }
        // Wait for PLL to lock
        while self.cs.read() & defs::PLL_CS_LOCK_BITS == 0 {/*NOP*/}
        // Set up post dividers - div1 feeds into div2 so if div1 is 5 and div2 is 2 then you get a divide by 10
        let pdiv = (post_div1 << defs::PLL_PRIM_POSTDIV1_LSB) | (post_div2 << defs::PLL_PRIM_POSTDIV2_LSB);
        self.prim.write(pdiv);
        // Turn on post divider
        {
            let t = self.pwr.read();
            self.pwr.write(t & !defs::PLL_PWR_POSTDIVPD_BITS);
        }
    }
}

impl ops::Deref for Peripheral {
    type Target = self::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            match self.sel {
                Pll::Sys => &*Self::PTR_SYS,
                Pll::Usb => &*Self::PTR_USB
            }
        }
    }
}

impl ops::DerefMut for Peripheral {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            match self.sel {
                Pll::Sys => &mut *Self::PTR_SYS,
                Pll::Usb => &mut *Self::PTR_USB
            }
        }
    }
}