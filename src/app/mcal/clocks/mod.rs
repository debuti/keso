#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

const CLOCK_INDEX_LENGTH: usize = 10;

#[derive(PartialEq, Clone, Copy)]
pub enum ClockIndex {
    Gpout0 = 0,     //< GPIO Muxing 0
    Gpout1,         //< GPIO Muxing 1
    Gpout2,         //< GPIO Muxing 2
    Gpout3,         //< GPIO Muxing 3
    Ref,            //< Watchdog and timers reference clock
    Sys,            //< Processors, bus fabric, memory, memory mapped registers
    Peri,           //< Peripheral clock for UART and SPI
    Usb,            //< USB clock
    Adc,            //< ADC clock
    Rtc,            //< Real time clock
}

impl ClockIndex {
    // Clock muxing consists of two components:
    // - A glitchless mux, which can be switched freely, but whose inputs must be
    //   free-running
    // - An auxiliary (glitchy) mux, whose output glitches when switched, but has
    //   no constraints on its inputs
    // Not all clocks have both types of mux.
    #[inline(always)]
    pub fn has_glitchless_mux(clkidx: ClockIndex) -> bool {
        clkidx == Self::Sys || clkidx == Self::Ref
    }
}

#[repr(C)]
pub struct Clock {
    ctrl: Volatile<u32>,
    div: Volatile<u32>,
    selected: Volatile<u32>,
}

#[repr(C)]
pub struct Fc {
    ref_khz: Volatile<u32>,
    min_khz: Volatile<u32>,
    max_khz: Volatile<u32>,
    delay: Volatile<u32>,
    interval: Volatile<u32>,
    src: Volatile<u32>,
    status: Volatile<u32>,
    result: Volatile<u32>,
}

#[repr(C)]
pub struct Resus {
    ctrl: Volatile<u32>,
    status: Volatile<u32>,
}

#[repr(C)]
pub struct RegisterBlock {
    clk: [Clock; CLOCK_INDEX_LENGTH],
    resus: Resus,
    fc0: Fc,
    wake_en0: Volatile<u32>,
    wake_en1: Volatile<u32>,
    sleep_en0: Volatile<u32>,
    sleep_en1: Volatile<u32>,
    enabled0: Volatile<u32>,
    enabled1: Volatile<u32>,
    intr: Volatile<u32>,
    inte: Volatile<u32>,
    intf: Volatile<u32>,
    ints: Volatile<u32>,
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
}

static mut CONFIGURED_FREQ: [u32; CLOCK_INDEX_LENGTH] = [0; CLOCK_INDEX_LENGTH];

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub const PTR: *mut self::RegisterBlock = super::CLOCKS_BASE as *mut _;

    #[inline(never)]
    pub fn disable_resus(&mut self) {
        self.resus.ctrl.write(0);
    }

    #[inline(never)]
    pub fn reset_source(&mut self, clkidx: ClockIndex) {
        let clkidx = clkidx as usize;
        let t = self.clk[clkidx].ctrl.read();
        //TODO: SRC field length is different between clocks, do it automatically, doing it this way is gross
        self.clk[clkidx].ctrl.write(t & !(0x3));
        while self.clk[clkidx].selected.read() != 0x1 {/*NOP*/}
    }


    #[inline(never)]
    pub fn clock_configure(&mut self, clkidx: ClockIndex, src: u32, auxsrc: u32, src_freq: u32, freq: u32) -> Result<(),()> {
        assert_eq!(src_freq >= freq, true);
        let clkidx = &clkidx;
        let clkidx_ = *clkidx as usize;

        if freq > src_freq {return Err(());}
    
        // Div register is 24.8 int.frac divider so multiply by 2^8 (left shift by 8)
        let div: u32 = (((src_freq as u64) << 8) / (freq as u64)) as u32;
            
        // If increasing divisor, set divisor before source. Otherwise set source
        // before divisor. This avoids a momentary overspeed when e.g. switching
        // to a faster source and increasing divisor to compensate.
        if div > self.clk[clkidx_].div.read() {
            self.clk[clkidx_].div.write(div);
        }
    
        // If switching a glitchless slice (ref or sys) to an aux source, switch
        // away from aux *first* to avoid passing glitches when changing aux mux.
        // Assume (!!!) glitchless source 0 is no faster than the aux source.
        if ClockIndex::has_glitchless_mux(*clkidx) && src == defs::CLOCKS_CLK_SYS_CTRL_SRC_VALUE_CLKSRC_CLK_SYS_AUX {
            let t = self.clk[clkidx_].ctrl.read();
            self.clk[clkidx_].ctrl.write(t & !defs::CLOCKS_CLK_REF_CTRL_SRC_BITS);
            while self.clk[clkidx_].selected.read() & 1 == 0 {/*NOP*/}
        }

        // If no glitchless mux, cleanly stop the clock to avoid glitches
        // propagating when changing aux mux. Note it would be a really bad idea
        // to do this on one of the glitchless clocks (clk_sys, clk_ref).
        else {
            let t = self.clk[clkidx_].ctrl.read();
            self.clk[clkidx_].ctrl.write(t & !defs::CLOCKS_CLK_GPOUT0_CTRL_ENABLE_BITS);
            unsafe {
                if CONFIGURED_FREQ[clkidx_] > 0 {
                    // Delay for 3 cycles of the target clock, for ENABLE propagation.
                    // Note XOSC_COUNT is not helpful here because XOSC is not
                    // necessarily running, nor is timer... so, 3 cycles per loop:
                    let _delay_cyc = CONFIGURED_FREQ[ClockIndex::Sys as usize] / CONFIGURED_FREQ[clkidx_] + 1;
                    //TODO:
                    /*asm volatile (
                        ".syntax unified \n\t"
                        "1: \n\t"
                        "subs %0, #1 \n\t"
                        "bne 1b"
                        : "+r" (delay_cyc)
                    );*/
                }
            }
        }
    
        // Set aux mux first, and then glitchless mux if this clock has one
        {
            let t = self.clk[clkidx_].ctrl.read() & !defs::CLOCKS_CLK_SYS_CTRL_AUXSRC_BITS;
            self.clk[clkidx_].ctrl.write(t | ((auxsrc << defs::CLOCKS_CLK_SYS_CTRL_AUXSRC_LSB) & defs::CLOCKS_CLK_SYS_CTRL_AUXSRC_BITS));
        }
    
        if ClockIndex::has_glitchless_mux(*clkidx) {
            let t = self.clk[clkidx_].ctrl.read() & !defs::CLOCKS_CLK_REF_CTRL_SRC_BITS;
            self.clk[clkidx_].ctrl.write(t | ((src << defs::CLOCKS_CLK_REF_CTRL_SRC_LSB) & defs::CLOCKS_CLK_REF_CTRL_SRC_BITS));
            
            while self.clk[clkidx_].selected.read() & (1 << src) == 0 {/*NOP*/}        
        }
        
        {
            let t = self.clk[clkidx_].ctrl.read();
            self.clk[clkidx_].ctrl.write(t | defs::CLOCKS_CLK_GPOUT0_CTRL_ENABLE_BITS);
        }

        // Now that the source is configured, we can trust that the user-supplied
        // divisor is a safe value.
        self.clk[clkidx_].div.write(div);
    
        // Store the configured frequency
        unsafe {
            CONFIGURED_FREQ[clkidx_] = freq;
        }

        Ok(())
    }

    pub fn clock_get_hz(&mut self, clkidx: ClockIndex) -> u32 {
        let clkidx = clkidx as usize;
        unsafe {
            return CONFIGURED_FREQ[clkidx];
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