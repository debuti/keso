#![allow(dead_code)]

use volatile::Volatile;
use core::ops;
use core::marker::PhantomData;

pub mod defs;

pub enum GpioFunction {
    XIP = 0,
    SPI = 1,
    UART = 2,
    I2C = 3,
    PWM = 4,
    SIO = 5,
    PIO0 = 6,
    PIO1 = 7,
    GPCK = 8,
    USB = 9,
    NULL = 0xf,
}

#[repr(C)]
pub struct io_hw {
    status: Volatile<u32>,
    ctrl: Volatile<u32>,
}

#[repr(C)]
pub struct io_irq_ctrl_hw {
    inte: [Volatile<u32>; 0x4],
    intf: [Volatile<u32>; 0x4],
    ints: [Volatile<u32>; 0x4],
}

#[repr(C)]
pub struct iobank0_hw {
    io: [io_hw; 30],
    intr: [Volatile<u32>; 0x4],
    proc0_irq_ctrl: io_irq_ctrl_hw,
    proc1_irq_ctrl: io_irq_ctrl_hw,
    dormant_wake_irq_ctrl: io_irq_ctrl_hw,
   
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

    pub const PTR: *mut self::iobank0_hw = super::IO_BANK0_BASE as *mut _;

    #[inline(never)]
    pub fn writeio(&mut self, idx: usize, value: u32) {
        self.io[idx].ctrl.write(value)
    }

    pub fn force_high(&mut self, idx: usize) {
        self.writeio(idx, defs::IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_ENABLE << defs::IO_BANK0_GPIO0_CTRL_OEOVER_LSB
            | defs::IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_HIGH << defs::IO_BANK0_GPIO0_CTRL_OUTOVER_LSB
            | defs::IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_NULL << defs::IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB)
    }

    pub fn force_low(&mut self, idx: usize) {
        self.writeio(idx, defs::IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_ENABLE << defs::IO_BANK0_GPIO0_CTRL_OEOVER_LSB
            | defs::IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_LOW << defs::IO_BANK0_GPIO0_CTRL_OUTOVER_LSB
            | defs::IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_NULL << defs::IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB)
    }

    /// Function Select Table
    ///
    ///  GPIO   | F1       | F2        | F3       | F4     | F5  | F6   | F7   | F8            | F9
    ///  -------|----------|-----------|----------|--------|-----|------|------|---------------|----
    ///  0      | SPI0 RX  | UART0 TX  | I2C0 SDA | PWM0 A | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  1      | SPI0 CSn | UART0 RX  | I2C0 SCL | PWM0 B | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  2      | SPI0 SCK | UART0 CTS | I2C1 SDA | PWM1 A | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  3      | SPI0 TX  | UART0 RTS | I2C1 SCL | PWM1 B | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  4      | SPI0 RX  | UART1 TX  | I2C0 SDA | PWM2 A | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  5      | SPI0 CSn | UART1 RX  | I2C0 SCL | PWM2 B | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  6      | SPI0 SCK | UART1 CTS | I2C1 SDA | PWM3 A | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  7      | SPI0 TX  | UART1 RTS | I2C1 SCL | PWM3 B | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  8      | SPI1 RX  | UART1 TX  | I2C0 SDA | PWM4 A | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  9      | SPI1 CSn | UART1 RX  | I2C0 SCL | PWM4 B | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  10     | SPI1 SCK | UART1 CTS | I2C1 SDA | PWM5 A | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  11     | SPI1 TX  | UART1 RTS | I2C1 SCL | PWM5 B | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  12     | SPI1 RX  | UART0 TX  | I2C0 SDA | PWM6 A | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  13     | SPI1 CSn | UART0 RX  | I2C0 SCL | PWM6 B | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  14     | SPI1 SCK | UART0 CTS | I2C1 SDA | PWM7 A | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  15     | SPI1 TX  | UART0 RTS | I2C1 SCL | PWM7 B | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  16     | SPI0 RX  | UART0 TX  | I2C0 SDA | PWM0 A | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  17     | SPI0 CSn | UART0 RX  | I2C0 SCL | PWM0 B | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  18     | SPI0 SCK | UART0 CTS | I2C1 SDA | PWM1 A | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  19     | SPI0 TX  | UART0 RTS | I2C1 SCL | PWM1 B | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  20     | SPI0 RX  | UART1 TX  | I2C0 SDA | PWM2 A | SIO | PIO0 | PIO1 | CLOCK GPIN0   | USB VBUS EN
    ///  21     | SPI0 CSn | UART1 RX  | I2C0 SCL | PWM2 B | SIO | PIO0 | PIO1 | CLOCK GPOUT0  | USB OVCUR DET
    ///  22     | SPI0 SCK | UART1 CTS | I2C1 SDA | PWM3 A | SIO | PIO0 | PIO1 | CLOCK GPIN1   | USB VBUS DET
    ///  23     | SPI0 TX  | UART1 RTS | I2C1 SCL | PWM3 B | SIO | PIO0 | PIO1 | CLOCK GPOUT1  | USB VBUS EN
    ///  24     | SPI1 RX  | UART1 TX  | I2C0 SDA | PWM4 A | SIO | PIO0 | PIO1 | CLOCK GPOUT2  | USB OVCUR DET
    ///  25     | SPI1 CSn | UART1 RX  | I2C0 SCL | PWM4 B | SIO | PIO0 | PIO1 | CLOCK GPOUT3  | USB VBUS DET
    ///  26     | SPI1 SCK | UART1 CTS | I2C1 SDA | PWM5 A | SIO | PIO0 | PIO1 |               | USB VBUS EN
    ///  27     | SPI1 TX  | UART1 RTS | I2C1 SCL | PWM5 B | SIO | PIO0 | PIO1 |               | USB OVCUR DET
    ///  28     | SPI1 RX  | UART0 TX  | I2C0 SDA | PWM6 A | SIO | PIO0 | PIO1 |               | USB VBUS DET
    ///  29     | SPI1 CSn | UART0 RX  | I2C0 SCL | PWM6 B | SIO | PIO0 | PIO1 |               | USB VBUS EN
    pub fn gpio_set_function(&mut self, gpio: usize, function: GpioFunction) {
        //invalid_params_if(GPIO, gpio >= NUM_BANK0_GPIOS);
        //invalid_params_if(GPIO, ((uint32_t)fn << IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB) & ~IO_BANK0_GPIO0_CTRL_FUNCSEL_BITS);
        
        // Set input enable on, output disable off
        let mut padsbank0 = super::padsbank0::Peripheral::new();
        {
            let t = padsbank0.readio(gpio);
            padsbank0.writeio(gpio, (t & !(super::padsbank0::defs::PADS_BANK0_GPIO0_IE_BITS | super::padsbank0::defs::PADS_BANK0_GPIO0_OD_BITS)) | super::padsbank0::defs::PADS_BANK0_GPIO0_IE_BITS);
        }
        // Zero all fields apart from fsel; we want this IO to do what the peripheral tells it.
        // This doesn't affect e.g. pullup/pulldown, as these are in pad controls.
        self.writeio(gpio, (function as u32) << defs::IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB)
    }
    
}

impl ops::Deref for Peripheral {
    type Target = self::iobank0_hw;

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
