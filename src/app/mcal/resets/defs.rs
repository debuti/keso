// =============================================================================
// Register block : RESETS
// Version        : 1
// Bus type       : apb
// Description    : None
// =============================================================================
// =============================================================================
// Register    : RESETS_RESET
// Description : Reset control. If a bit is set it means the peripheral is in
//               reset. 0 means the peripheral's reset is deasserted.
pub(super) const RESETS_RESET_OFFSET: u32 = 0x00000000;
pub(super) const RESETS_RESET_BITS: u32 = 0x01ffffff;
pub(super) const RESETS_RESET_RESET: u32 = 0x01ffffff;
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_USBCTRL
// Description : None
pub(super) const RESETS_RESET_USBCTRL_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_USBCTRL_BITS: u32 = 0x01000000;
pub(super) const RESETS_RESET_USBCTRL_MSB: i32 = 24;
pub(super) const RESETS_RESET_USBCTRL_LSB: i32 = 24;
pub(super) const RESETS_RESET_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_UART1
// Description : None
pub(super) const RESETS_RESET_UART1_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_UART1_BITS: u32 = 0x00800000;
pub(super) const RESETS_RESET_UART1_MSB: i32 = 23;
pub(super) const RESETS_RESET_UART1_LSB: i32 = 23;
pub(super) const RESETS_RESET_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_UART0
// Description : None
pub(super) const RESETS_RESET_UART0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_UART0_BITS: u32 = 0x00400000;
pub(super) const RESETS_RESET_UART0_MSB: i32 = 22;
pub(super) const RESETS_RESET_UART0_LSB: i32 = 22;
pub(super) const RESETS_RESET_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_TIMER
// Description : None
pub(super) const RESETS_RESET_TIMER_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_TIMER_BITS: u32 = 0x00200000;
pub(super) const RESETS_RESET_TIMER_MSB: i32 = 21;
pub(super) const RESETS_RESET_TIMER_LSB: i32 = 21;
pub(super) const RESETS_RESET_TIMER_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_TBMAN
// Description : None
pub(super) const RESETS_RESET_TBMAN_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_TBMAN_BITS: u32 = 0x00100000;
pub(super) const RESETS_RESET_TBMAN_MSB: i32 = 20;
pub(super) const RESETS_RESET_TBMAN_LSB: i32 = 20;
pub(super) const RESETS_RESET_TBMAN_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_SYSINFO
// Description : None
pub(super) const RESETS_RESET_SYSINFO_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_SYSINFO_BITS: u32 = 0x00080000;
pub(super) const RESETS_RESET_SYSINFO_MSB: i32 = 19;
pub(super) const RESETS_RESET_SYSINFO_LSB: i32 = 19;
pub(super) const RESETS_RESET_SYSINFO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_SYSCFG
// Description : None
pub(super) const RESETS_RESET_SYSCFG_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_SYSCFG_BITS: u32 = 0x00040000;
pub(super) const RESETS_RESET_SYSCFG_MSB: i32 = 18;
pub(super) const RESETS_RESET_SYSCFG_LSB: i32 = 18;
pub(super) const RESETS_RESET_SYSCFG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_SPI1
// Description : None
pub(super) const RESETS_RESET_SPI1_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_SPI1_BITS: u32 = 0x00020000;
pub(super) const RESETS_RESET_SPI1_MSB: i32 = 17;
pub(super) const RESETS_RESET_SPI1_LSB: i32 = 17;
pub(super) const RESETS_RESET_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_SPI0
// Description : None
pub(super) const RESETS_RESET_SPI0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_SPI0_BITS: u32 = 0x00010000;
pub(super) const RESETS_RESET_SPI0_MSB: i32 = 16;
pub(super) const RESETS_RESET_SPI0_LSB: i32 = 16;
pub(super) const RESETS_RESET_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_RTC
// Description : None
pub(super) const RESETS_RESET_RTC_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_RTC_BITS: u32 = 0x00008000;
pub(super) const RESETS_RESET_RTC_MSB: i32 = 15;
pub(super) const RESETS_RESET_RTC_LSB: i32 = 15;
pub(super) const RESETS_RESET_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PWM
// Description : None
pub(super) const RESETS_RESET_PWM_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PWM_BITS: u32 = 0x00004000;
pub(super) const RESETS_RESET_PWM_MSB: i32 = 14;
pub(super) const RESETS_RESET_PWM_LSB: i32 = 14;
pub(super) const RESETS_RESET_PWM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PLL_USB
// Description : None
pub(super) const RESETS_RESET_PLL_USB_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PLL_USB_BITS: u32 = 0x00002000;
pub(super) const RESETS_RESET_PLL_USB_MSB: i32 = 13;
pub(super) const RESETS_RESET_PLL_USB_LSB: i32 = 13;
pub(super) const RESETS_RESET_PLL_USB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PLL_SYS
// Description : None
pub(super) const RESETS_RESET_PLL_SYS_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PLL_SYS_BITS: u32 = 0x00001000;
pub(super) const RESETS_RESET_PLL_SYS_MSB: i32 = 12;
pub(super) const RESETS_RESET_PLL_SYS_LSB: i32 = 12;
pub(super) const RESETS_RESET_PLL_SYS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PIO1
// Description : None
pub(super) const RESETS_RESET_PIO1_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PIO1_BITS: u32 = 0x00000800;
pub(super) const RESETS_RESET_PIO1_MSB: i32 = 11;
pub(super) const RESETS_RESET_PIO1_LSB: i32 = 11;
pub(super) const RESETS_RESET_PIO1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PIO0
// Description : None
pub(super) const RESETS_RESET_PIO0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PIO0_BITS: u32 = 0x00000400;
pub(super) const RESETS_RESET_PIO0_MSB: i32 = 10;
pub(super) const RESETS_RESET_PIO0_LSB: i32 = 10;
pub(super) const RESETS_RESET_PIO0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PADS_QSPI
// Description : None
pub(super) const RESETS_RESET_PADS_QSPI_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PADS_QSPI_BITS: u32 = 0x00000200;
pub(super) const RESETS_RESET_PADS_QSPI_MSB: i32 = 9;
pub(super) const RESETS_RESET_PADS_QSPI_LSB: i32 = 9;
pub(super) const RESETS_RESET_PADS_QSPI_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_PADS_BANK0
// Description : None
pub(super) const RESETS_RESET_PADS_BANK0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_PADS_BANK0_BITS: u32 = 0x00000100;
pub(super) const RESETS_RESET_PADS_BANK0_MSB: i32 = 8;
pub(super) const RESETS_RESET_PADS_BANK0_LSB: i32 = 8;
pub(super) const RESETS_RESET_PADS_BANK0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_JTAG
// Description : None
pub(super) const RESETS_RESET_JTAG_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_JTAG_BITS: u32 = 0x00000080;
pub(super) const RESETS_RESET_JTAG_MSB: i32 = 7;
pub(super) const RESETS_RESET_JTAG_LSB: i32 = 7;
pub(super) const RESETS_RESET_JTAG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_IO_QSPI
// Description : None
pub(super) const RESETS_RESET_IO_QSPI_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_IO_QSPI_BITS: u32 = 0x00000040;
pub(super) const RESETS_RESET_IO_QSPI_MSB: i32 = 6;
pub(super) const RESETS_RESET_IO_QSPI_LSB: i32 = 6;
pub(super) const RESETS_RESET_IO_QSPI_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_IO_BANK0
// Description : None
pub(super) const RESETS_RESET_IO_BANK0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_IO_BANK0_BITS: u32 = 0x00000020;
pub(super) const RESETS_RESET_IO_BANK0_MSB: i32 = 5;
pub(super) const RESETS_RESET_IO_BANK0_LSB: i32 = 5;
pub(super) const RESETS_RESET_IO_BANK0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_I2C1
// Description : None
pub(super) const RESETS_RESET_I2C1_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_I2C1_BITS: u32 = 0x00000010;
pub(super) const RESETS_RESET_I2C1_MSB: i32 = 4;
pub(super) const RESETS_RESET_I2C1_LSB: i32 = 4;
pub(super) const RESETS_RESET_I2C1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_I2C0
// Description : None
pub(super) const RESETS_RESET_I2C0_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_I2C0_BITS: u32 = 0x00000008;
pub(super) const RESETS_RESET_I2C0_MSB: i32 = 3;
pub(super) const RESETS_RESET_I2C0_LSB: i32 = 3;
pub(super) const RESETS_RESET_I2C0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DMA
// Description : None
pub(super) const RESETS_RESET_DMA_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_DMA_BITS: u32 = 0x00000004;
pub(super) const RESETS_RESET_DMA_MSB: i32 = 2;
pub(super) const RESETS_RESET_DMA_LSB: i32 = 2;
pub(super) const RESETS_RESET_DMA_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_BUSCTRL
// Description : None
pub(super) const RESETS_RESET_BUSCTRL_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_BUSCTRL_BITS: u32 = 0x00000002;
pub(super) const RESETS_RESET_BUSCTRL_MSB: i32 = 1;
pub(super) const RESETS_RESET_BUSCTRL_LSB: i32 = 1;
pub(super) const RESETS_RESET_BUSCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_ADC
// Description : None
pub(super) const RESETS_RESET_ADC_RESET: u32 = 0x1;
pub(super) const RESETS_RESET_ADC_BITS: u32 = 0x00000001;
pub(super) const RESETS_RESET_ADC_MSB: i32 = 0;
pub(super) const RESETS_RESET_ADC_LSB: i32 = 0;
pub(super) const RESETS_RESET_ADC_ACCESS: &str = "RW";
// =============================================================================
// Register    : RESETS_WDSEL
// Description : Watchdog select. If a bit is set then the watchdog will reset
//               this peripheral when the watchdog fires.
pub(super) const RESETS_WDSEL_OFFSET: u32 = 0x00000004;
pub(super) const RESETS_WDSEL_BITS: u32 = 0x01ffffff;
pub(super) const RESETS_WDSEL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_USBCTRL
// Description : None
pub(super) const RESETS_WDSEL_USBCTRL_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_USBCTRL_BITS: u32 = 0x01000000;
pub(super) const RESETS_WDSEL_USBCTRL_MSB: i32 = 24;
pub(super) const RESETS_WDSEL_USBCTRL_LSB: i32 = 24;
pub(super) const RESETS_WDSEL_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_UART1
// Description : None
pub(super) const RESETS_WDSEL_UART1_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_UART1_BITS: u32 = 0x00800000;
pub(super) const RESETS_WDSEL_UART1_MSB: i32 = 23;
pub(super) const RESETS_WDSEL_UART1_LSB: i32 = 23;
pub(super) const RESETS_WDSEL_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_UART0
// Description : None
pub(super) const RESETS_WDSEL_UART0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_UART0_BITS: u32 = 0x00400000;
pub(super) const RESETS_WDSEL_UART0_MSB: i32 = 22;
pub(super) const RESETS_WDSEL_UART0_LSB: i32 = 22;
pub(super) const RESETS_WDSEL_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_TIMER
// Description : None
pub(super) const RESETS_WDSEL_TIMER_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_TIMER_BITS: u32 = 0x00200000;
pub(super) const RESETS_WDSEL_TIMER_MSB: i32 = 21;
pub(super) const RESETS_WDSEL_TIMER_LSB: i32 = 21;
pub(super) const RESETS_WDSEL_TIMER_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_TBMAN
// Description : None
pub(super) const RESETS_WDSEL_TBMAN_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_TBMAN_BITS: u32 = 0x00100000;
pub(super) const RESETS_WDSEL_TBMAN_MSB: i32 = 20;
pub(super) const RESETS_WDSEL_TBMAN_LSB: i32 = 20;
pub(super) const RESETS_WDSEL_TBMAN_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_SYSINFO
// Description : None
pub(super) const RESETS_WDSEL_SYSINFO_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_SYSINFO_BITS: u32 = 0x00080000;
pub(super) const RESETS_WDSEL_SYSINFO_MSB: i32 = 19;
pub(super) const RESETS_WDSEL_SYSINFO_LSB: i32 = 19;
pub(super) const RESETS_WDSEL_SYSINFO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_SYSCFG
// Description : None
pub(super) const RESETS_WDSEL_SYSCFG_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_SYSCFG_BITS: u32 = 0x00040000;
pub(super) const RESETS_WDSEL_SYSCFG_MSB: i32 = 18;
pub(super) const RESETS_WDSEL_SYSCFG_LSB: i32 = 18;
pub(super) const RESETS_WDSEL_SYSCFG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_SPI1
// Description : None
pub(super) const RESETS_WDSEL_SPI1_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_SPI1_BITS: u32 = 0x00020000;
pub(super) const RESETS_WDSEL_SPI1_MSB: i32 = 17;
pub(super) const RESETS_WDSEL_SPI1_LSB: i32 = 17;
pub(super) const RESETS_WDSEL_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_SPI0
// Description : None
pub(super) const RESETS_WDSEL_SPI0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_SPI0_BITS: u32 = 0x00010000;
pub(super) const RESETS_WDSEL_SPI0_MSB: i32 = 16;
pub(super) const RESETS_WDSEL_SPI0_LSB: i32 = 16;
pub(super) const RESETS_WDSEL_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_RTC
// Description : None
pub(super) const RESETS_WDSEL_RTC_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_RTC_BITS: u32 = 0x00008000;
pub(super) const RESETS_WDSEL_RTC_MSB: i32 = 15;
pub(super) const RESETS_WDSEL_RTC_LSB: i32 = 15;
pub(super) const RESETS_WDSEL_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PWM
// Description : None
pub(super) const RESETS_WDSEL_PWM_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PWM_BITS: u32 = 0x00004000;
pub(super) const RESETS_WDSEL_PWM_MSB: i32 = 14;
pub(super) const RESETS_WDSEL_PWM_LSB: i32 = 14;
pub(super) const RESETS_WDSEL_PWM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PLL_USB
// Description : None
pub(super) const RESETS_WDSEL_PLL_USB_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PLL_USB_BITS: u32 = 0x00002000;
pub(super) const RESETS_WDSEL_PLL_USB_MSB: i32 = 13;
pub(super) const RESETS_WDSEL_PLL_USB_LSB: i32 = 13;
pub(super) const RESETS_WDSEL_PLL_USB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PLL_SYS
// Description : None
pub(super) const RESETS_WDSEL_PLL_SYS_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PLL_SYS_BITS: u32 = 0x00001000;
pub(super) const RESETS_WDSEL_PLL_SYS_MSB: i32 = 12;
pub(super) const RESETS_WDSEL_PLL_SYS_LSB: i32 = 12;
pub(super) const RESETS_WDSEL_PLL_SYS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PIO1
// Description : None
pub(super) const RESETS_WDSEL_PIO1_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PIO1_BITS: u32 = 0x00000800;
pub(super) const RESETS_WDSEL_PIO1_MSB: i32 = 11;
pub(super) const RESETS_WDSEL_PIO1_LSB: i32 = 11;
pub(super) const RESETS_WDSEL_PIO1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PIO0
// Description : None
pub(super) const RESETS_WDSEL_PIO0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PIO0_BITS: u32 = 0x00000400;
pub(super) const RESETS_WDSEL_PIO0_MSB: i32 = 10;
pub(super) const RESETS_WDSEL_PIO0_LSB: i32 = 10;
pub(super) const RESETS_WDSEL_PIO0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PADS_QSPI
// Description : None
pub(super) const RESETS_WDSEL_PADS_QSPI_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PADS_QSPI_BITS: u32 = 0x00000200;
pub(super) const RESETS_WDSEL_PADS_QSPI_MSB: i32 = 9;
pub(super) const RESETS_WDSEL_PADS_QSPI_LSB: i32 = 9;
pub(super) const RESETS_WDSEL_PADS_QSPI_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_PADS_BANK0
// Description : None
pub(super) const RESETS_WDSEL_PADS_BANK0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_PADS_BANK0_BITS: u32 = 0x00000100;
pub(super) const RESETS_WDSEL_PADS_BANK0_MSB: i32 = 8;
pub(super) const RESETS_WDSEL_PADS_BANK0_LSB: i32 = 8;
pub(super) const RESETS_WDSEL_PADS_BANK0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_JTAG
// Description : None
pub(super) const RESETS_WDSEL_JTAG_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_JTAG_BITS: u32 = 0x00000080;
pub(super) const RESETS_WDSEL_JTAG_MSB: i32 = 7;
pub(super) const RESETS_WDSEL_JTAG_LSB: i32 = 7;
pub(super) const RESETS_WDSEL_JTAG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_IO_QSPI
// Description : None
pub(super) const RESETS_WDSEL_IO_QSPI_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_IO_QSPI_BITS: u32 = 0x00000040;
pub(super) const RESETS_WDSEL_IO_QSPI_MSB: i32 = 6;
pub(super) const RESETS_WDSEL_IO_QSPI_LSB: i32 = 6;
pub(super) const RESETS_WDSEL_IO_QSPI_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_IO_BANK0
// Description : None
pub(super) const RESETS_WDSEL_IO_BANK0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_IO_BANK0_BITS: u32 = 0x00000020;
pub(super) const RESETS_WDSEL_IO_BANK0_MSB: i32 = 5;
pub(super) const RESETS_WDSEL_IO_BANK0_LSB: i32 = 5;
pub(super) const RESETS_WDSEL_IO_BANK0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_I2C1
// Description : None
pub(super) const RESETS_WDSEL_I2C1_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_I2C1_BITS: u32 = 0x00000010;
pub(super) const RESETS_WDSEL_I2C1_MSB: i32 = 4;
pub(super) const RESETS_WDSEL_I2C1_LSB: i32 = 4;
pub(super) const RESETS_WDSEL_I2C1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_I2C0
// Description : None
pub(super) const RESETS_WDSEL_I2C0_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_I2C0_BITS: u32 = 0x00000008;
pub(super) const RESETS_WDSEL_I2C0_MSB: i32 = 3;
pub(super) const RESETS_WDSEL_I2C0_LSB: i32 = 3;
pub(super) const RESETS_WDSEL_I2C0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_DMA
// Description : None
pub(super) const RESETS_WDSEL_DMA_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_DMA_BITS: u32 = 0x00000004;
pub(super) const RESETS_WDSEL_DMA_MSB: i32 = 2;
pub(super) const RESETS_WDSEL_DMA_LSB: i32 = 2;
pub(super) const RESETS_WDSEL_DMA_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_BUSCTRL
// Description : None
pub(super) const RESETS_WDSEL_BUSCTRL_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_BUSCTRL_BITS: u32 = 0x00000002;
pub(super) const RESETS_WDSEL_BUSCTRL_MSB: i32 = 1;
pub(super) const RESETS_WDSEL_BUSCTRL_LSB: i32 = 1;
pub(super) const RESETS_WDSEL_BUSCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : RESETS_WDSEL_ADC
// Description : None
pub(super) const RESETS_WDSEL_ADC_RESET: u32 = 0x0;
pub(super) const RESETS_WDSEL_ADC_BITS: u32 = 0x00000001;
pub(super) const RESETS_WDSEL_ADC_MSB: i32 = 0;
pub(super) const RESETS_WDSEL_ADC_LSB: i32 = 0;
pub(super) const RESETS_WDSEL_ADC_ACCESS: &str = "RW";
// =============================================================================
// Register    : RESETS_RESET_DONE
// Description : Reset done. If a bit is set then a reset done signal has been
//               returned by the peripheral. This indicates that the
//               peripheral's registers are ready to be accessed.
pub(super) const RESETS_RESET_DONE_OFFSET: u32 = 0x00000008;
pub(super) const RESETS_RESET_DONE_BITS: u32 = 0x01ffffff;
pub(super) const RESETS_RESET_DONE_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_USBCTRL
// Description : None
pub(super) const RESETS_RESET_DONE_USBCTRL_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_USBCTRL_BITS: u32 = 0x01000000;
pub(super) const RESETS_RESET_DONE_USBCTRL_MSB: i32 = 24;
pub(super) const RESETS_RESET_DONE_USBCTRL_LSB: i32 = 24;
pub(super) const RESETS_RESET_DONE_USBCTRL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_UART1
// Description : None
pub(super) const RESETS_RESET_DONE_UART1_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_UART1_BITS: u32 = 0x00800000;
pub(super) const RESETS_RESET_DONE_UART1_MSB: i32 = 23;
pub(super) const RESETS_RESET_DONE_UART1_LSB: i32 = 23;
pub(super) const RESETS_RESET_DONE_UART1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_UART0
// Description : None
pub(super) const RESETS_RESET_DONE_UART0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_UART0_BITS: u32 = 0x00400000;
pub(super) const RESETS_RESET_DONE_UART0_MSB: i32 = 22;
pub(super) const RESETS_RESET_DONE_UART0_LSB: i32 = 22;
pub(super) const RESETS_RESET_DONE_UART0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_TIMER
// Description : None
pub(super) const RESETS_RESET_DONE_TIMER_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_TIMER_BITS: u32 = 0x00200000;
pub(super) const RESETS_RESET_DONE_TIMER_MSB: i32 = 21;
pub(super) const RESETS_RESET_DONE_TIMER_LSB: i32 = 21;
pub(super) const RESETS_RESET_DONE_TIMER_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_TBMAN
// Description : None
pub(super) const RESETS_RESET_DONE_TBMAN_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_TBMAN_BITS: u32 = 0x00100000;
pub(super) const RESETS_RESET_DONE_TBMAN_MSB: i32 = 20;
pub(super) const RESETS_RESET_DONE_TBMAN_LSB: i32 = 20;
pub(super) const RESETS_RESET_DONE_TBMAN_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_SYSINFO
// Description : None
pub(super) const RESETS_RESET_DONE_SYSINFO_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_SYSINFO_BITS: u32 = 0x00080000;
pub(super) const RESETS_RESET_DONE_SYSINFO_MSB: i32 = 19;
pub(super) const RESETS_RESET_DONE_SYSINFO_LSB: i32 = 19;
pub(super) const RESETS_RESET_DONE_SYSINFO_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_SYSCFG
// Description : None
pub(super) const RESETS_RESET_DONE_SYSCFG_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_SYSCFG_BITS: u32 = 0x00040000;
pub(super) const RESETS_RESET_DONE_SYSCFG_MSB: i32 = 18;
pub(super) const RESETS_RESET_DONE_SYSCFG_LSB: i32 = 18;
pub(super) const RESETS_RESET_DONE_SYSCFG_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_SPI1
// Description : None
pub(super) const RESETS_RESET_DONE_SPI1_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_SPI1_BITS: u32 = 0x00020000;
pub(super) const RESETS_RESET_DONE_SPI1_MSB: i32 = 17;
pub(super) const RESETS_RESET_DONE_SPI1_LSB: i32 = 17;
pub(super) const RESETS_RESET_DONE_SPI1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_SPI0
// Description : None
pub(super) const RESETS_RESET_DONE_SPI0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_SPI0_BITS: u32 = 0x00010000;
pub(super) const RESETS_RESET_DONE_SPI0_MSB: i32 = 16;
pub(super) const RESETS_RESET_DONE_SPI0_LSB: i32 = 16;
pub(super) const RESETS_RESET_DONE_SPI0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_RTC
// Description : None
pub(super) const RESETS_RESET_DONE_RTC_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_RTC_BITS: u32 = 0x00008000;
pub(super) const RESETS_RESET_DONE_RTC_MSB: i32 = 15;
pub(super) const RESETS_RESET_DONE_RTC_LSB: i32 = 15;
pub(super) const RESETS_RESET_DONE_RTC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PWM
// Description : None
pub(super) const RESETS_RESET_DONE_PWM_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PWM_BITS: u32 = 0x00004000;
pub(super) const RESETS_RESET_DONE_PWM_MSB: i32 = 14;
pub(super) const RESETS_RESET_DONE_PWM_LSB: i32 = 14;
pub(super) const RESETS_RESET_DONE_PWM_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PLL_USB
// Description : None
pub(super) const RESETS_RESET_DONE_PLL_USB_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PLL_USB_BITS: u32 = 0x00002000;
pub(super) const RESETS_RESET_DONE_PLL_USB_MSB: i32 = 13;
pub(super) const RESETS_RESET_DONE_PLL_USB_LSB: i32 = 13;
pub(super) const RESETS_RESET_DONE_PLL_USB_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PLL_SYS
// Description : None
pub(super) const RESETS_RESET_DONE_PLL_SYS_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PLL_SYS_BITS: u32 = 0x00001000;
pub(super) const RESETS_RESET_DONE_PLL_SYS_MSB: i32 = 12;
pub(super) const RESETS_RESET_DONE_PLL_SYS_LSB: i32 = 12;
pub(super) const RESETS_RESET_DONE_PLL_SYS_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PIO1
// Description : None
pub(super) const RESETS_RESET_DONE_PIO1_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PIO1_BITS: u32 = 0x00000800;
pub(super) const RESETS_RESET_DONE_PIO1_MSB: i32 = 11;
pub(super) const RESETS_RESET_DONE_PIO1_LSB: i32 = 11;
pub(super) const RESETS_RESET_DONE_PIO1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PIO0
// Description : None
pub(super) const RESETS_RESET_DONE_PIO0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PIO0_BITS: u32 = 0x00000400;
pub(super) const RESETS_RESET_DONE_PIO0_MSB: i32 = 10;
pub(super) const RESETS_RESET_DONE_PIO0_LSB: i32 = 10;
pub(super) const RESETS_RESET_DONE_PIO0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PADS_QSPI
// Description : None
pub(super) const RESETS_RESET_DONE_PADS_QSPI_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PADS_QSPI_BITS: u32 = 0x00000200;
pub(super) const RESETS_RESET_DONE_PADS_QSPI_MSB: i32 = 9;
pub(super) const RESETS_RESET_DONE_PADS_QSPI_LSB: i32 = 9;
pub(super) const RESETS_RESET_DONE_PADS_QSPI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_PADS_BANK0
// Description : None
pub(super) const RESETS_RESET_DONE_PADS_BANK0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_PADS_BANK0_BITS: u32 = 0x00000100;
pub(super) const RESETS_RESET_DONE_PADS_BANK0_MSB: i32 = 8;
pub(super) const RESETS_RESET_DONE_PADS_BANK0_LSB: i32 = 8;
pub(super) const RESETS_RESET_DONE_PADS_BANK0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_JTAG
// Description : None
pub(super) const RESETS_RESET_DONE_JTAG_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_JTAG_BITS: u32 = 0x00000080;
pub(super) const RESETS_RESET_DONE_JTAG_MSB: i32 = 7;
pub(super) const RESETS_RESET_DONE_JTAG_LSB: i32 = 7;
pub(super) const RESETS_RESET_DONE_JTAG_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_IO_QSPI
// Description : None
pub(super) const RESETS_RESET_DONE_IO_QSPI_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_IO_QSPI_BITS: u32 = 0x00000040;
pub(super) const RESETS_RESET_DONE_IO_QSPI_MSB: i32 = 6;
pub(super) const RESETS_RESET_DONE_IO_QSPI_LSB: i32 = 6;
pub(super) const RESETS_RESET_DONE_IO_QSPI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_IO_BANK0
// Description : None
pub(super) const RESETS_RESET_DONE_IO_BANK0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_IO_BANK0_BITS: u32 = 0x00000020;
pub(super) const RESETS_RESET_DONE_IO_BANK0_MSB: i32 = 5;
pub(super) const RESETS_RESET_DONE_IO_BANK0_LSB: i32 = 5;
pub(super) const RESETS_RESET_DONE_IO_BANK0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_I2C1
// Description : None
pub(super) const RESETS_RESET_DONE_I2C1_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_I2C1_BITS: u32 = 0x00000010;
pub(super) const RESETS_RESET_DONE_I2C1_MSB: i32 = 4;
pub(super) const RESETS_RESET_DONE_I2C1_LSB: i32 = 4;
pub(super) const RESETS_RESET_DONE_I2C1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_I2C0
// Description : None
pub(super) const RESETS_RESET_DONE_I2C0_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_I2C0_BITS: u32 = 0x00000008;
pub(super) const RESETS_RESET_DONE_I2C0_MSB: i32 = 3;
pub(super) const RESETS_RESET_DONE_I2C0_LSB: i32 = 3;
pub(super) const RESETS_RESET_DONE_I2C0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_DMA
// Description : None
pub(super) const RESETS_RESET_DONE_DMA_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_DMA_BITS: u32 = 0x00000004;
pub(super) const RESETS_RESET_DONE_DMA_MSB: i32 = 2;
pub(super) const RESETS_RESET_DONE_DMA_LSB: i32 = 2;
pub(super) const RESETS_RESET_DONE_DMA_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_BUSCTRL
// Description : None
pub(super) const RESETS_RESET_DONE_BUSCTRL_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_BUSCTRL_BITS: u32 = 0x00000002;
pub(super) const RESETS_RESET_DONE_BUSCTRL_MSB: i32 = 1;
pub(super) const RESETS_RESET_DONE_BUSCTRL_LSB: i32 = 1;
pub(super) const RESETS_RESET_DONE_BUSCTRL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : RESETS_RESET_DONE_ADC
// Description : None
pub(super) const RESETS_RESET_DONE_ADC_RESET: u32 = 0x0;
pub(super) const RESETS_RESET_DONE_ADC_BITS: u32 = 0x00000001;
pub(super) const RESETS_RESET_DONE_ADC_MSB: i32 = 0;
pub(super) const RESETS_RESET_DONE_ADC_LSB: i32 = 0;
pub(super) const RESETS_RESET_DONE_ADC_ACCESS: &str = "RO";
// =============================================================================
