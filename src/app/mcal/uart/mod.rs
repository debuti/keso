#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

pub const PICO_UART_DEFAULT_CRLF: bool = false;
pub const PICO_DEFAULT_UART_BAUD_RATE: u32 = 115200;

const NUM_UARTS: usize = 2;

#[derive(Copy, Clone)]
pub enum Uart {
    Uart0,
    Uart1,
}

#[derive(PartialEq)]
pub enum Parity {
    NONE = 0,
    EVEN,
    ODD,
}

#[repr(C)]
pub struct RegisterBlock {
    dr: Volatile<u32>,
    rsr: Volatile<u32>,
    _pad0: [Volatile<u32>; 4],
    fr: Volatile<u32>,
    _pad1: Volatile<u32>,
    ilpr: Volatile<u32>,
    ibrd: Volatile<u32>,
    fbrd: Volatile<u32>,
    lcr_h: Volatile<u32>,
    cr: Volatile<u32>,
    ifls: Volatile<u32>,
    imsc: Volatile<u32>,
    ris: Volatile<u32>,
    mis: Volatile<u32>,
    icr: Volatile<u32>,
    dmacr: Volatile<u32>,
}

pub struct Peripheral {
    _marker: PhantomData<*const ()>,
    sel : Uart,
}

static mut UART_CHAR_TO_LINE_FEED: [char; NUM_UARTS] = ['\n'; NUM_UARTS];

unsafe impl Send for Peripheral {}

impl Peripheral {
    #[inline(always)]
    pub(crate) const fn new(sel: Uart) -> Self {
        Self {
            _marker: PhantomData,
            sel: sel,
        }
    }

    pub const PTR_UART0: *mut self::RegisterBlock = super::UART0_BASE as *mut _;
    pub const PTR_UART1: *mut self::RegisterBlock = super::UART1_BASE as *mut _;

    #[inline(never)]
    pub fn uart_init(&mut self, baudrate:u32) -> Result<u32,()> {
        {
            let mut clks = super::clocks::Peripheral::new();

            if clks.clock_get_hz(super::clocks::ClockIndex::Peri) == 0 {return Err(());}
        }
        self.uart_reset();
        self.uart_unreset();
    
        // #if PICO_UART_ENABLE_CRLF_SUPPORT
        self.uart_set_translate_crlf(PICO_UART_DEFAULT_CRLF);
        //#endif
     
        // Any LCR writes need to take place before enabling the UART
        let baud: u32 = self.uart_set_baudrate(baudrate);
        self.uart_set_format(8, 1, Parity::NONE);
        
        // Enable the UART, both TX and RX
        self.cr.write(defs::UART_UARTCR_UARTEN_BITS | defs::UART_UARTCR_TXE_BITS | defs::UART_UARTCR_RXE_BITS);
        // Enable FIFOs
        {
            let t = self.lcr_h.read();
            self.lcr_h.write(t | defs::UART_UARTLCR_H_FEN_BITS);
        }
        // Always enable DREQ signals -- no harm in this if DMA is not listening
        self.dmacr.write(defs::UART_UARTDMACR_TXDMAE_BITS | defs::UART_UARTDMACR_RXDMAE_BITS);
    
        Ok(baud)
    }

    #[inline(never)]
    pub fn uart_reset(&mut self) {
        let mut resets = super::resets::Peripheral::new();
        match &self.sel {
            Uart::Uart0 => resets.disable(super::resets::ResetDevice::Uart0),
            Uart::Uart1 => resets.disable(super::resets::ResetDevice::Uart1),
        }
    }
    
    #[inline(never)]
    pub fn uart_unreset(&mut self) {
        let mut resets = super::resets::Peripheral::new();
        match &self.sel {
            Uart::Uart0 => resets.enable(super::resets::ResetDevice::Uart0),
            Uart::Uart1 => resets.enable(super::resets::ResetDevice::Uart1),
        }
    }

    #[inline(never)]
    pub fn uart_set_format(&mut self, data_bits: u32, stop_bits: u32, parity: Parity) {
//TODO:        invalid_params_if(UART, data_bits < 5 || data_bits > 8);
//TODO:        invalid_params_if(UART, stop_bits != 1 && stop_bits != 2);
        let t = self.lcr_h.read();
        let value = ((data_bits - 5) << defs::UART_UARTLCR_H_WLEN_LSB) |
                    ((stop_bits - 1) << defs::UART_UARTLCR_H_STP2_LSB) |
                    (((parity != Parity::NONE) as u32) << defs::UART_UARTLCR_H_PEN_LSB) |
                    (((parity == Parity::EVEN) as u32) << defs::UART_UARTLCR_H_EPS_LSB);
        let mask = defs::UART_UARTLCR_H_WLEN_BITS | defs::UART_UARTLCR_H_STP2_BITS |
                   defs::UART_UARTLCR_H_PEN_BITS | defs::UART_UARTLCR_H_EPS_BITS;
        self.lcr_h.write((t & !mask) | value);        
        
                       
    }

    #[inline(never)]
    pub fn uart_set_baudrate(&mut self, baudrate: u32) -> u32 {
        assert_eq!(baudrate == 0, false); 

        let mut clks = super::clocks::Peripheral::new();
        let baud_rate_div: u32 = 8 * clks.clock_get_hz(super::clocks::ClockIndex::Peri) / baudrate;
        let mut baud_ibrd: u32 = baud_rate_div >> 7;
        let mut baud_fbrd: u32 = ((baud_rate_div & 0x7f) + 1) / 2;

        if baud_ibrd == 0 {
            baud_ibrd = 1;
            baud_fbrd = 0;
        } else if baud_ibrd >= 65535 {
            baud_ibrd = 65535;
            baud_fbrd = 0;
        }
    
        // Load PL011's baud divisor registers
        self.ibrd.write(baud_ibrd);
        self.fbrd.write(baud_fbrd);
    
        // PL011 needs a (dummy) line control register write to latch in the
        // divisors. We don't want to actually change LCR contents here.
        let t = self.lcr_h.read(); 
        self.lcr_h.write(t);
    
        // See datasheet
        (4 * clks.clock_get_hz(super::clocks::ClockIndex::Peri)) / (64 * baud_ibrd + baud_fbrd)
    }

    #[inline(never)]
    pub fn uart_set_translate_crlf(&mut self, crlf: bool) {
        unsafe {
            UART_CHAR_TO_LINE_FEED[self.sel as usize] = {
                if crlf {'\n'}
                else {0 as char}
            };
        }
    }
    
    #[inline(never)]
    pub fn puts(&mut self, s: &str) {
    //#if PICO_UART_ENABLE_CRLF_SUPPORT
        let mut last_was_cr: bool = false;
        for c in s.chars() {
            // Don't add extra carriage returns if one is present
            if last_was_cr {self.putc_raw(c);}
            else {self.putc(c);}
            last_was_cr = c == '\r';
        }
    //#endif
    }

    #[inline(never)]
    pub fn putc(&mut self, c: char) {
    //#if PICO_UART_ENABLE_CRLF_SUPPORT
      unsafe {
        if UART_CHAR_TO_LINE_FEED[self.sel as usize] == c {
            self.putc_raw('\r');
        }
      }
    //#endif
      self.putc_raw(c);
    }

    #[inline(never)]
    pub fn putc_raw(&mut self, c: char) {
        self.uart_write_blocking(c);
    }

    #[inline(never)]
    pub fn uart_write_blocking(&mut self, c:char) {
        while !self.uart_is_writable() {/*NOP*/}
        self.dr.write(c as u32);   
    }

    #[inline(never)]
    pub fn uart_is_writable(&self) -> bool {
        (self.fr.read() & defs::UART_UARTFR_TXFF_BITS) == 0
    }
}

impl ops::Deref for Peripheral {
    type Target = self::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe {
            match self.sel {
                Uart::Uart0 => &*Self::PTR_UART0,
                Uart::Uart1 => &*Self::PTR_UART1,
            }
        }
    }
}

impl ops::DerefMut for Peripheral {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            match self.sel {
                Uart::Uart0 => &mut *Self::PTR_UART0,
                Uart::Uart1 => &mut *Self::PTR_UART1
            }
        }
    }
}