// =============================================================================
// Register block : IO_BANK0
// Version        : 1
// Bus type       : apb
// Description    : None
// =============================================================================
// =============================================================================
// Register    : IO_BANK0_GPIO0_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO0_STATUS_OFFSET: u32 = 0x00000000;
pub(super) const IO_BANK0_GPIO0_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO0_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO0_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO0_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO0_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO0_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO0_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO0_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO0_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO0_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO0_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO0_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO0_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO0_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO0_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO0_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO0_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO0_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO0_CTRL_OFFSET: u32 = 0x00000004;
pub(super) const IO_BANK0_GPIO0_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO0_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO0_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO0_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO0_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO0_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO0_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x00 -> jtag_tck
//               0x01 -> spi0_rx
//               0x02 -> uart0_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_0
//               0x05 -> sio_0
//               0x06 -> pio0_0
//               0x07 -> pio1_0
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_JTAG_TCK: u32 = 0x00;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_SPI0_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_UART0_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_PWM_A_0: u32 = 0x04;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_SIO_0: u32 = 0x05;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_PIO0_0: u32 = 0x06;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_PIO1_0: u32 = 0x07;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO0_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO1_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO1_STATUS_OFFSET: u32 = 0x00000008;
pub(super) const IO_BANK0_GPIO1_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO1_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO1_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO1_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO1_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO1_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO1_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO1_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO1_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO1_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO1_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO1_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO1_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO1_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO1_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO1_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO1_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO1_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO1_CTRL_OFFSET: u32 = 0x0000000c;
pub(super) const IO_BANK0_GPIO1_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO1_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO1_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO1_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO1_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO1_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO1_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x00 -> jtag_tms
//               0x01 -> spi0_ss_n
//               0x02 -> uart0_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_0
//               0x05 -> sio_1
//               0x06 -> pio0_1
//               0x07 -> pio1_1
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_JTAG_TMS: u32 = 0x00;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_SPI0_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_UART0_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_PWM_B_0: u32 = 0x04;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_SIO_1: u32 = 0x05;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_PIO0_1: u32 = 0x06;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_PIO1_1: u32 = 0x07;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO1_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO2_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO2_STATUS_OFFSET: u32 = 0x00000010;
pub(super) const IO_BANK0_GPIO2_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO2_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO2_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO2_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO2_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO2_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO2_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO2_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO2_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO2_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO2_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO2_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO2_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO2_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO2_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO2_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO2_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO2_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO2_CTRL_OFFSET: u32 = 0x00000014;
pub(super) const IO_BANK0_GPIO2_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO2_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO2_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO2_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO2_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO2_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO2_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x00 -> jtag_tdi
//               0x01 -> spi0_sclk
//               0x02 -> uart0_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_1
//               0x05 -> sio_2
//               0x06 -> pio0_2
//               0x07 -> pio1_2
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_JTAG_TDI: u32 = 0x00;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_SPI0_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_UART0_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_PWM_A_1: u32 = 0x04;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_SIO_2: u32 = 0x05;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_PIO0_2: u32 = 0x06;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_PIO1_2: u32 = 0x07;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO2_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO3_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO3_STATUS_OFFSET: u32 = 0x00000018;
pub(super) const IO_BANK0_GPIO3_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO3_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO3_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO3_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO3_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO3_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO3_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO3_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO3_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO3_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO3_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO3_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO3_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO3_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO3_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO3_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO3_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO3_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO3_CTRL_OFFSET: u32 = 0x0000001c;
pub(super) const IO_BANK0_GPIO3_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO3_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO3_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO3_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO3_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO3_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO3_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x00 -> jtag_tdo
//               0x01 -> spi0_tx
//               0x02 -> uart0_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_1
//               0x05 -> sio_3
//               0x06 -> pio0_3
//               0x07 -> pio1_3
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_JTAG_TDO: u32 = 0x00;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_SPI0_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_UART0_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_PWM_B_1: u32 = 0x04;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_SIO_3: u32 = 0x05;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_PIO0_3: u32 = 0x06;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_PIO1_3: u32 = 0x07;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO3_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO4_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO4_STATUS_OFFSET: u32 = 0x00000020;
pub(super) const IO_BANK0_GPIO4_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO4_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO4_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO4_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO4_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO4_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO4_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO4_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO4_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO4_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO4_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO4_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO4_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO4_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO4_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO4_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO4_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO4_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO4_CTRL_OFFSET: u32 = 0x00000024;
pub(super) const IO_BANK0_GPIO4_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO4_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO4_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO4_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO4_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO4_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO4_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_rx
//               0x02 -> uart1_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_2
//               0x05 -> sio_4
//               0x06 -> pio0_4
//               0x07 -> pio1_4
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_SPI0_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_UART1_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_PWM_A_2: u32 = 0x04;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_SIO_4: u32 = 0x05;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_PIO0_4: u32 = 0x06;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_PIO1_4: u32 = 0x07;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO4_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO5_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO5_STATUS_OFFSET: u32 = 0x00000028;
pub(super) const IO_BANK0_GPIO5_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO5_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO5_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO5_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO5_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO5_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO5_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO5_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO5_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO5_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO5_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO5_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO5_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO5_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO5_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO5_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO5_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO5_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO5_CTRL_OFFSET: u32 = 0x0000002c;
pub(super) const IO_BANK0_GPIO5_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO5_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO5_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO5_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO5_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO5_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO5_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_ss_n
//               0x02 -> uart1_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_2
//               0x05 -> sio_5
//               0x06 -> pio0_5
//               0x07 -> pio1_5
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_SPI0_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_UART1_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_PWM_B_2: u32 = 0x04;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_SIO_5: u32 = 0x05;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_PIO0_5: u32 = 0x06;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_PIO1_5: u32 = 0x07;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO5_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO6_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO6_STATUS_OFFSET: u32 = 0x00000030;
pub(super) const IO_BANK0_GPIO6_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO6_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO6_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO6_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO6_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO6_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO6_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO6_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO6_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO6_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO6_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO6_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO6_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO6_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO6_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO6_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO6_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO6_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO6_CTRL_OFFSET: u32 = 0x00000034;
pub(super) const IO_BANK0_GPIO6_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO6_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO6_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO6_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO6_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO6_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO6_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_sclk
//               0x02 -> uart1_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_3
//               0x05 -> sio_6
//               0x06 -> pio0_6
//               0x07 -> pio1_6
//               0x08 -> usb_muxing_extphy_softcon
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_SPI0_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_UART1_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_PWM_A_3: u32 = 0x04;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_SIO_6: u32 = 0x05;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_PIO0_6: u32 = 0x06;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_PIO1_6: u32 = 0x07;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_SOFTCON: u32 = 0x08;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO6_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO7_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO7_STATUS_OFFSET: u32 = 0x00000038;
pub(super) const IO_BANK0_GPIO7_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO7_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO7_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO7_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO7_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO7_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO7_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO7_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO7_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO7_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO7_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO7_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO7_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO7_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO7_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO7_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO7_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO7_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO7_CTRL_OFFSET: u32 = 0x0000003c;
pub(super) const IO_BANK0_GPIO7_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO7_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO7_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO7_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO7_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO7_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO7_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_tx
//               0x02 -> uart1_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_3
//               0x05 -> sio_7
//               0x06 -> pio0_7
//               0x07 -> pio1_7
//               0x08 -> usb_muxing_extphy_oe_n
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_SPI0_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_UART1_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_PWM_B_3: u32 = 0x04;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_SIO_7: u32 = 0x05;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_PIO0_7: u32 = 0x06;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_PIO1_7: u32 = 0x07;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_OE_N: u32 = 0x08;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO7_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO8_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO8_STATUS_OFFSET: u32 = 0x00000040;
pub(super) const IO_BANK0_GPIO8_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO8_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO8_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO8_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO8_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO8_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO8_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO8_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO8_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO8_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO8_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO8_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO8_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO8_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO8_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO8_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO8_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO8_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO8_CTRL_OFFSET: u32 = 0x00000044;
pub(super) const IO_BANK0_GPIO8_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO8_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO8_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO8_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO8_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO8_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO8_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_rx
//               0x02 -> uart1_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_4
//               0x05 -> sio_8
//               0x06 -> pio0_8
//               0x07 -> pio1_8
//               0x08 -> usb_muxing_extphy_rcv
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_SPI1_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_UART1_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_PWM_A_4: u32 = 0x04;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_SIO_8: u32 = 0x05;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_PIO0_8: u32 = 0x06;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_PIO1_8: u32 = 0x07;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_RCV: u32 = 0x08;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO8_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO9_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO9_STATUS_OFFSET: u32 = 0x00000048;
pub(super) const IO_BANK0_GPIO9_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO9_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO9_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO9_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO9_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO9_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO9_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO9_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO9_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO9_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO9_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO9_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO9_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO9_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO9_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO9_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO9_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO9_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO9_CTRL_OFFSET: u32 = 0x0000004c;
pub(super) const IO_BANK0_GPIO9_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO9_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO9_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO9_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO9_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO9_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO9_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_ss_n
//               0x02 -> uart1_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_4
//               0x05 -> sio_9
//               0x06 -> pio0_9
//               0x07 -> pio1_9
//               0x08 -> usb_muxing_extphy_vp
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_SPI1_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_UART1_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_PWM_B_4: u32 = 0x04;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_SIO_9: u32 = 0x05;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_PIO0_9: u32 = 0x06;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_PIO1_9: u32 = 0x07;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_VP: u32 = 0x08;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO9_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO10_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO10_STATUS_OFFSET: u32 = 0x00000050;
pub(super) const IO_BANK0_GPIO10_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO10_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO10_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO10_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO10_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO10_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO10_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO10_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO10_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO10_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO10_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO10_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO10_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO10_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO10_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO10_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO10_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO10_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO10_CTRL_OFFSET: u32 = 0x00000054;
pub(super) const IO_BANK0_GPIO10_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO10_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO10_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO10_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO10_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO10_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO10_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_sclk
//               0x02 -> uart1_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_5
//               0x05 -> sio_10
//               0x06 -> pio0_10
//               0x07 -> pio1_10
//               0x08 -> usb_muxing_extphy_vm
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_SPI1_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_UART1_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_PWM_A_5: u32 = 0x04;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_SIO_10: u32 = 0x05;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_PIO0_10: u32 = 0x06;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_PIO1_10: u32 = 0x07;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_VM: u32 = 0x08;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO10_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO11_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO11_STATUS_OFFSET: u32 = 0x00000058;
pub(super) const IO_BANK0_GPIO11_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO11_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO11_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO11_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO11_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO11_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO11_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO11_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO11_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO11_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO11_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO11_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO11_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO11_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO11_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO11_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO11_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO11_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO11_CTRL_OFFSET: u32 = 0x0000005c;
pub(super) const IO_BANK0_GPIO11_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO11_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO11_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO11_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO11_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO11_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO11_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_tx
//               0x02 -> uart1_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_5
//               0x05 -> sio_11
//               0x06 -> pio0_11
//               0x07 -> pio1_11
//               0x08 -> usb_muxing_extphy_suspnd
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_SPI1_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_UART1_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_PWM_B_5: u32 = 0x04;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_SIO_11: u32 = 0x05;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_PIO0_11: u32 = 0x06;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_PIO1_11: u32 = 0x07;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_SUSPND: u32 = 0x08;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO11_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO12_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO12_STATUS_OFFSET: u32 = 0x00000060;
pub(super) const IO_BANK0_GPIO12_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO12_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO12_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO12_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO12_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO12_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO12_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO12_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO12_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO12_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO12_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO12_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO12_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO12_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO12_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO12_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO12_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO12_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO12_CTRL_OFFSET: u32 = 0x00000064;
pub(super) const IO_BANK0_GPIO12_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO12_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO12_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO12_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO12_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO12_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO12_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_rx
//               0x02 -> uart0_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_6
//               0x05 -> sio_12
//               0x06 -> pio0_12
//               0x07 -> pio1_12
//               0x08 -> usb_muxing_extphy_speed
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_SPI1_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_UART0_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_PWM_A_6: u32 = 0x04;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_SIO_12: u32 = 0x05;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_PIO0_12: u32 = 0x06;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_PIO1_12: u32 = 0x07;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_SPEED: u32 = 0x08;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO12_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO13_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO13_STATUS_OFFSET: u32 = 0x00000068;
pub(super) const IO_BANK0_GPIO13_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO13_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO13_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO13_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO13_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO13_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO13_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO13_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO13_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO13_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO13_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO13_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO13_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO13_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO13_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO13_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO13_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO13_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO13_CTRL_OFFSET: u32 = 0x0000006c;
pub(super) const IO_BANK0_GPIO13_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO13_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO13_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO13_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO13_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO13_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO13_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_ss_n
//               0x02 -> uart0_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_6
//               0x05 -> sio_13
//               0x06 -> pio0_13
//               0x07 -> pio1_13
//               0x08 -> usb_muxing_extphy_vpo
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_SPI1_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_UART0_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_PWM_B_6: u32 = 0x04;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_SIO_13: u32 = 0x05;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_PIO0_13: u32 = 0x06;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_PIO1_13: u32 = 0x07;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_VPO: u32 = 0x08;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO13_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO14_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO14_STATUS_OFFSET: u32 = 0x00000070;
pub(super) const IO_BANK0_GPIO14_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO14_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO14_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO14_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO14_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO14_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO14_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO14_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO14_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO14_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO14_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO14_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO14_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO14_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO14_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO14_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO14_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO14_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO14_CTRL_OFFSET: u32 = 0x00000074;
pub(super) const IO_BANK0_GPIO14_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO14_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO14_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO14_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO14_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO14_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO14_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_sclk
//               0x02 -> uart0_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_7
//               0x05 -> sio_14
//               0x06 -> pio0_14
//               0x07 -> pio1_14
//               0x08 -> usb_muxing_extphy_vmo
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_SPI1_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_UART0_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_PWM_A_7: u32 = 0x04;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_SIO_14: u32 = 0x05;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_PIO0_14: u32 = 0x06;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_PIO1_14: u32 = 0x07;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_USB_MUXING_EXTPHY_VMO: u32 = 0x08;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO14_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO15_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO15_STATUS_OFFSET: u32 = 0x00000078;
pub(super) const IO_BANK0_GPIO15_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO15_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO15_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO15_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO15_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO15_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO15_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO15_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO15_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO15_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO15_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO15_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO15_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO15_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO15_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO15_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO15_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO15_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO15_CTRL_OFFSET: u32 = 0x0000007c;
pub(super) const IO_BANK0_GPIO15_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO15_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO15_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO15_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO15_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO15_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO15_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_tx
//               0x02 -> uart0_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_7
//               0x05 -> sio_15
//               0x06 -> pio0_15
//               0x07 -> pio1_15
//               0x08 -> usb_muxing_digital_dp
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_SPI1_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_UART0_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_PWM_B_7: u32 = 0x04;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_SIO_15: u32 = 0x05;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_PIO0_15: u32 = 0x06;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_PIO1_15: u32 = 0x07;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_USB_MUXING_DIGITAL_DP: u32 = 0x08;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO15_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO16_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO16_STATUS_OFFSET: u32 = 0x00000080;
pub(super) const IO_BANK0_GPIO16_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO16_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO16_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO16_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO16_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO16_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO16_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO16_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO16_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO16_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO16_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO16_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO16_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO16_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO16_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO16_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO16_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO16_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO16_CTRL_OFFSET: u32 = 0x00000084;
pub(super) const IO_BANK0_GPIO16_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO16_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO16_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO16_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO16_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO16_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO16_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_rx
//               0x02 -> uart0_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_0
//               0x05 -> sio_16
//               0x06 -> pio0_16
//               0x07 -> pio1_16
//               0x08 -> usb_muxing_digital_dm
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_SPI0_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_UART0_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_PWM_A_0: u32 = 0x04;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_SIO_16: u32 = 0x05;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_PIO0_16: u32 = 0x06;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_PIO1_16: u32 = 0x07;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_USB_MUXING_DIGITAL_DM: u32 = 0x08;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO16_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO17_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO17_STATUS_OFFSET: u32 = 0x00000088;
pub(super) const IO_BANK0_GPIO17_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO17_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO17_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO17_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO17_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO17_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO17_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO17_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO17_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO17_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO17_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO17_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO17_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO17_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO17_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO17_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO17_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO17_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO17_CTRL_OFFSET: u32 = 0x0000008c;
pub(super) const IO_BANK0_GPIO17_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO17_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO17_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO17_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO17_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO17_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO17_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_ss_n
//               0x02 -> uart0_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_0
//               0x05 -> sio_17
//               0x06 -> pio0_17
//               0x07 -> pio1_17
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_SPI0_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_UART0_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_PWM_B_0: u32 = 0x04;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_SIO_17: u32 = 0x05;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_PIO0_17: u32 = 0x06;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_PIO1_17: u32 = 0x07;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO17_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO18_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO18_STATUS_OFFSET: u32 = 0x00000090;
pub(super) const IO_BANK0_GPIO18_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO18_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO18_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO18_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO18_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO18_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO18_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO18_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO18_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO18_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO18_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO18_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO18_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO18_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO18_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO18_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO18_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO18_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO18_CTRL_OFFSET: u32 = 0x00000094;
pub(super) const IO_BANK0_GPIO18_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO18_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO18_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO18_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO18_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO18_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO18_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_sclk
//               0x02 -> uart0_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_1
//               0x05 -> sio_18
//               0x06 -> pio0_18
//               0x07 -> pio1_18
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_SPI0_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_UART0_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_PWM_A_1: u32 = 0x04;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_SIO_18: u32 = 0x05;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_PIO0_18: u32 = 0x06;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_PIO1_18: u32 = 0x07;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO18_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO19_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO19_STATUS_OFFSET: u32 = 0x00000098;
pub(super) const IO_BANK0_GPIO19_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO19_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO19_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO19_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO19_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO19_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO19_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO19_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO19_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO19_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO19_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO19_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO19_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO19_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO19_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO19_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO19_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO19_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO19_CTRL_OFFSET: u32 = 0x0000009c;
pub(super) const IO_BANK0_GPIO19_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO19_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO19_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO19_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO19_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO19_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO19_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_tx
//               0x02 -> uart0_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_1
//               0x05 -> sio_19
//               0x06 -> pio0_19
//               0x07 -> pio1_19
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_SPI0_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_UART0_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_PWM_B_1: u32 = 0x04;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_SIO_19: u32 = 0x05;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_PIO0_19: u32 = 0x06;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_PIO1_19: u32 = 0x07;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO19_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO20_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO20_STATUS_OFFSET: u32 = 0x000000a0;
pub(super) const IO_BANK0_GPIO20_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO20_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO20_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO20_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO20_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO20_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO20_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO20_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO20_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO20_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO20_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO20_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO20_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO20_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO20_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO20_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO20_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO20_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO20_CTRL_OFFSET: u32 = 0x000000a4;
pub(super) const IO_BANK0_GPIO20_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO20_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO20_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO20_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO20_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO20_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO20_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_rx
//               0x02 -> uart1_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_2
//               0x05 -> sio_20
//               0x06 -> pio0_20
//               0x07 -> pio1_20
//               0x08 -> clocks_gpin_0
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_SPI0_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_UART1_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_PWM_A_2: u32 = 0x04;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_SIO_20: u32 = 0x05;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_PIO0_20: u32 = 0x06;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_PIO1_20: u32 = 0x07;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_CLOCKS_GPIN_0: u32 = 0x08;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO20_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO21_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO21_STATUS_OFFSET: u32 = 0x000000a8;
pub(super) const IO_BANK0_GPIO21_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO21_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO21_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO21_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO21_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO21_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO21_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO21_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO21_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO21_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO21_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO21_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO21_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO21_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO21_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO21_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO21_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO21_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO21_CTRL_OFFSET: u32 = 0x000000ac;
pub(super) const IO_BANK0_GPIO21_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO21_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO21_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO21_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO21_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO21_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO21_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_ss_n
//               0x02 -> uart1_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_2
//               0x05 -> sio_21
//               0x06 -> pio0_21
//               0x07 -> pio1_21
//               0x08 -> clocks_gpout_0
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_SPI0_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_UART1_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_PWM_B_2: u32 = 0x04;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_SIO_21: u32 = 0x05;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_PIO0_21: u32 = 0x06;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_PIO1_21: u32 = 0x07;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_CLOCKS_GPOUT_0: u32 = 0x08;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO21_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO22_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO22_STATUS_OFFSET: u32 = 0x000000b0;
pub(super) const IO_BANK0_GPIO22_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO22_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO22_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO22_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO22_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO22_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO22_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO22_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO22_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO22_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO22_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO22_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO22_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO22_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO22_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO22_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO22_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO22_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO22_CTRL_OFFSET: u32 = 0x000000b4;
pub(super) const IO_BANK0_GPIO22_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO22_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO22_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO22_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO22_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO22_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO22_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_sclk
//               0x02 -> uart1_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_3
//               0x05 -> sio_22
//               0x06 -> pio0_22
//               0x07 -> pio1_22
//               0x08 -> clocks_gpin_1
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_SPI0_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_UART1_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_PWM_A_3: u32 = 0x04;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_SIO_22: u32 = 0x05;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_PIO0_22: u32 = 0x06;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_PIO1_22: u32 = 0x07;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_CLOCKS_GPIN_1: u32 = 0x08;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO22_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO23_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO23_STATUS_OFFSET: u32 = 0x000000b8;
pub(super) const IO_BANK0_GPIO23_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO23_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO23_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO23_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO23_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO23_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO23_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO23_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO23_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO23_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO23_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO23_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO23_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO23_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO23_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO23_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO23_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO23_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO23_CTRL_OFFSET: u32 = 0x000000bc;
pub(super) const IO_BANK0_GPIO23_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO23_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO23_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO23_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO23_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO23_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO23_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi0_tx
//               0x02 -> uart1_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_3
//               0x05 -> sio_23
//               0x06 -> pio0_23
//               0x07 -> pio1_23
//               0x08 -> clocks_gpout_1
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_SPI0_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_UART1_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_PWM_B_3: u32 = 0x04;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_SIO_23: u32 = 0x05;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_PIO0_23: u32 = 0x06;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_PIO1_23: u32 = 0x07;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_CLOCKS_GPOUT_1: u32 = 0x08;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO23_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO24_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO24_STATUS_OFFSET: u32 = 0x000000c0;
pub(super) const IO_BANK0_GPIO24_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO24_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO24_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO24_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO24_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO24_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO24_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO24_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO24_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO24_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO24_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO24_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO24_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO24_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO24_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO24_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO24_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO24_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO24_CTRL_OFFSET: u32 = 0x000000c4;
pub(super) const IO_BANK0_GPIO24_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO24_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO24_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO24_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO24_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO24_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO24_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_rx
//               0x02 -> uart1_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_4
//               0x05 -> sio_24
//               0x06 -> pio0_24
//               0x07 -> pio1_24
//               0x08 -> clocks_gpout_2
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_SPI1_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_UART1_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_PWM_A_4: u32 = 0x04;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_SIO_24: u32 = 0x05;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_PIO0_24: u32 = 0x06;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_PIO1_24: u32 = 0x07;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_CLOCKS_GPOUT_2: u32 = 0x08;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO24_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO25_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO25_STATUS_OFFSET: u32 = 0x000000c8;
pub(super) const IO_BANK0_GPIO25_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO25_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO25_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO25_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO25_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO25_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO25_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO25_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO25_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO25_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO25_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO25_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO25_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO25_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO25_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO25_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO25_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO25_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO25_CTRL_OFFSET: u32 = 0x000000cc;
pub(super) const IO_BANK0_GPIO25_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO25_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO25_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO25_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO25_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO25_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO25_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_ss_n
//               0x02 -> uart1_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_4
//               0x05 -> sio_25
//               0x06 -> pio0_25
//               0x07 -> pio1_25
//               0x08 -> clocks_gpout_3
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_SPI1_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_UART1_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_PWM_B_4: u32 = 0x04;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_SIO_25: u32 = 0x05;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_PIO0_25: u32 = 0x06;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_PIO1_25: u32 = 0x07;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_CLOCKS_GPOUT_3: u32 = 0x08;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO25_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO26_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO26_STATUS_OFFSET: u32 = 0x000000d0;
pub(super) const IO_BANK0_GPIO26_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO26_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO26_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO26_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO26_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO26_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO26_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO26_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO26_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO26_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO26_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO26_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO26_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO26_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO26_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO26_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO26_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO26_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO26_CTRL_OFFSET: u32 = 0x000000d4;
pub(super) const IO_BANK0_GPIO26_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO26_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO26_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO26_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO26_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO26_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO26_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_sclk
//               0x02 -> uart1_cts
//               0x03 -> i2c1_sda
//               0x04 -> pwm_a_5
//               0x05 -> sio_26
//               0x06 -> pio0_26
//               0x07 -> pio1_26
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_SPI1_SCLK: u32 = 0x01;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_UART1_CTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_I2C1_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_PWM_A_5: u32 = 0x04;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_SIO_26: u32 = 0x05;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_PIO0_26: u32 = 0x06;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_PIO1_26: u32 = 0x07;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO26_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO27_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO27_STATUS_OFFSET: u32 = 0x000000d8;
pub(super) const IO_BANK0_GPIO27_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO27_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO27_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO27_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO27_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO27_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO27_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO27_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO27_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO27_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO27_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO27_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO27_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO27_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO27_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO27_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO27_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO27_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO27_CTRL_OFFSET: u32 = 0x000000dc;
pub(super) const IO_BANK0_GPIO27_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO27_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO27_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO27_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO27_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO27_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO27_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_tx
//               0x02 -> uart1_rts
//               0x03 -> i2c1_scl
//               0x04 -> pwm_b_5
//               0x05 -> sio_27
//               0x06 -> pio0_27
//               0x07 -> pio1_27
//               0x09 -> usb_muxing_overcurr_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_SPI1_TX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_UART1_RTS: u32 = 0x02;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_I2C1_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_PWM_B_5: u32 = 0x04;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_SIO_27: u32 = 0x05;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_PIO0_27: u32 = 0x06;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_PIO1_27: u32 = 0x07;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_USB_MUXING_OVERCURR_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO27_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO28_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO28_STATUS_OFFSET: u32 = 0x000000e0;
pub(super) const IO_BANK0_GPIO28_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO28_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO28_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO28_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO28_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO28_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO28_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO28_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO28_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO28_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO28_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO28_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO28_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO28_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO28_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO28_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO28_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO28_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO28_CTRL_OFFSET: u32 = 0x000000e4;
pub(super) const IO_BANK0_GPIO28_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO28_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO28_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO28_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO28_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO28_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO28_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_rx
//               0x02 -> uart0_tx
//               0x03 -> i2c0_sda
//               0x04 -> pwm_a_6
//               0x05 -> sio_28
//               0x06 -> pio0_28
//               0x07 -> pio1_28
//               0x09 -> usb_muxing_vbus_detect
//               0x1f -> null
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_SPI1_RX: u32 = 0x01;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_UART0_TX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_I2C0_SDA: u32 = 0x03;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_PWM_A_6: u32 = 0x04;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_SIO_28: u32 = 0x05;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_PIO0_28: u32 = 0x06;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_PIO1_28: u32 = 0x07;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_DETECT: u32 = 0x09;
pub(super) const IO_BANK0_GPIO28_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_GPIO29_STATUS
// Description : GPIO status
pub(super) const IO_BANK0_GPIO29_STATUS_OFFSET: u32 = 0x000000e8;
pub(super) const IO_BANK0_GPIO29_STATUS_BITS: u32 = 0x050a3300;
pub(super) const IO_BANK0_GPIO29_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_IRQTOPROC
// Description : interrupt to processors, after override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_IRQTOPROC_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQTOPROC_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQTOPROC_MSB: i32 = 26;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQTOPROC_LSB: i32 = 26;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQTOPROC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_IRQFROMPAD
// Description : interrupt from pad before override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_IRQFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQFROMPAD_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQFROMPAD_MSB: i32 = 24;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQFROMPAD_LSB: i32 = 24;
pub(super) const IO_BANK0_GPIO29_STATUS_IRQFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_INTOPERI
// Description : input signal to peripheral, after override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_INTOPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_INTOPERI_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_GPIO29_STATUS_INTOPERI_MSB: i32 = 19;
pub(super) const IO_BANK0_GPIO29_STATUS_INTOPERI_LSB: i32 = 19;
pub(super) const IO_BANK0_GPIO29_STATUS_INTOPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_INFROMPAD
// Description : input signal from pad, before override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_INFROMPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_INFROMPAD_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_GPIO29_STATUS_INFROMPAD_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO29_STATUS_INFROMPAD_LSB: i32 = 17;
pub(super) const IO_BANK0_GPIO29_STATUS_INFROMPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_OETOPAD
// Description : output enable to pad after register override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_OETOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_OETOPAD_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_GPIO29_STATUS_OETOPAD_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO29_STATUS_OETOPAD_LSB: i32 = 13;
pub(super) const IO_BANK0_GPIO29_STATUS_OETOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_OEFROMPERI
// Description : output enable from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_OEFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_OEFROMPERI_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_GPIO29_STATUS_OEFROMPERI_MSB: i32 = 12;
pub(super) const IO_BANK0_GPIO29_STATUS_OEFROMPERI_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO29_STATUS_OEFROMPERI_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_OUTTOPAD
// Description : output signal to pad after register override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_OUTTOPAD_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTTOPAD_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTTOPAD_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTTOPAD_LSB: i32 = 9;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTTOPAD_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_STATUS_OUTFROMPERI
// Description : output signal from selected peripheral, before register
//               override is applied
pub(super) const IO_BANK0_GPIO29_STATUS_OUTFROMPERI_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTFROMPERI_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTFROMPERI_MSB: i32 = 8;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTFROMPERI_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO29_STATUS_OUTFROMPERI_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_GPIO29_CTRL
// Description : GPIO control including function select and overrides.
pub(super) const IO_BANK0_GPIO29_CTRL_OFFSET: u32 = 0x000000ec;
pub(super) const IO_BANK0_GPIO29_CTRL_BITS: u32 = 0x3003331f;
pub(super) const IO_BANK0_GPIO29_CTRL_RESET: u32 = 0x0000001f;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_CTRL_IRQOVER
// Description : 0x0 -> don't invert the interrupt
//               0x1 -> invert the interrupt
//               0x2 -> drive interrupt low
//               0x3 -> drive interrupt high
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_BITS: u32 = 0x30000000;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_MSB: i32 = 29;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_LSB: i32 = 28;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO29_CTRL_IRQOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_CTRL_INOVER
// Description : 0x0 -> don't invert the peri input
//               0x1 -> invert the peri input
//               0x2 -> drive peri input low
//               0x3 -> drive peri input high
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_BITS: u32 = 0x00030000;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_MSB: i32 = 17;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_LSB: i32 = 16;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO29_CTRL_INOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_CTRL_OEOVER
// Description : 0x0 -> drive output enable from peripheral signal selected by
//               funcsel
//               0x1 -> drive output enable from inverse of peripheral signal
//               selected by funcsel
//               0x2 -> disable output
//               0x3 -> enable output
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_BITS: u32 = 0x00003000;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_MSB: i32 = 13;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_LSB: i32 = 12;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_VALUE_DISABLE: u32 = 0x2;
pub(super) const IO_BANK0_GPIO29_CTRL_OEOVER_VALUE_ENABLE: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_CTRL_OUTOVER
// Description : 0x0 -> drive output from peripheral signal selected by funcsel
//               0x1 -> drive output from inverse of peripheral signal selected
//               by funcsel
//               0x2 -> drive output low
//               0x3 -> drive output high
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_RESET: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_BITS: u32 = 0x00000300;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_MSB: i32 = 9;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_LSB: i32 = 8;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_VALUE_NORMAL: u32 = 0x0;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_VALUE_INVERT: u32 = 0x1;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_VALUE_LOW: u32 = 0x2;
pub(super) const IO_BANK0_GPIO29_CTRL_OUTOVER_VALUE_HIGH: u32 = 0x3;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_GPIO29_CTRL_FUNCSEL
// Description : 0-31 -> selects pin function according to the gpio table
//               31 == NULL
//               0x01 -> spi1_ss_n
//               0x02 -> uart0_rx
//               0x03 -> i2c0_scl
//               0x04 -> pwm_b_6
//               0x05 -> sio_29
//               0x06 -> pio0_29
//               0x07 -> pio1_29
//               0x09 -> usb_muxing_vbus_en
//               0x1f -> null
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_RESET: u32 = 0x1f;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_BITS: u32 = 0x0000001f;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_MSB: i32 = 4;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_LSB: i32 = 0;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_ACCESS: &str = "RW";
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_SPI1_SS_N: u32 = 0x01;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_UART0_RX: u32 = 0x02;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_I2C0_SCL: u32 = 0x03;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_PWM_B_6: u32 = 0x04;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_SIO_29: u32 = 0x05;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_PIO0_29: u32 = 0x06;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_PIO1_29: u32 = 0x07;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_USB_MUXING_VBUS_EN: u32 = 0x09;
pub(super) const IO_BANK0_GPIO29_CTRL_FUNCSEL_VALUE_NULL: u32 = 0x1f;
// =============================================================================
// Register    : IO_BANK0_INTR0
// Description : Raw Interrupts
pub(super) const IO_BANK0_INTR0_OFFSET: u32 = 0x000000f0;
pub(super) const IO_BANK0_INTR0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_INTR0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_INTR0_GPIO7_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_INTR0_GPIO7_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_INTR0_GPIO6_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_INTR0_GPIO6_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_INTR0_GPIO5_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_INTR0_GPIO5_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_INTR0_GPIO4_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_INTR0_GPIO4_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_INTR0_GPIO3_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_INTR0_GPIO3_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_INTR0_GPIO2_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_INTR0_GPIO2_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_INTR0_GPIO1_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_INTR0_GPIO1_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_INTR0_GPIO0_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_INTR0_GPIO0_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_INTR1
// Description : Raw Interrupts
pub(super) const IO_BANK0_INTR1_OFFSET: u32 = 0x000000f4;
pub(super) const IO_BANK0_INTR1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_INTR1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_INTR1_GPIO15_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_INTR1_GPIO15_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_INTR1_GPIO14_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_INTR1_GPIO14_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_INTR1_GPIO13_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_INTR1_GPIO13_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_INTR1_GPIO12_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_INTR1_GPIO12_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_INTR1_GPIO11_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_INTR1_GPIO11_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_INTR1_GPIO10_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_INTR1_GPIO10_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_INTR1_GPIO9_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_INTR1_GPIO9_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_INTR1_GPIO8_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_INTR1_GPIO8_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_INTR2
// Description : Raw Interrupts
pub(super) const IO_BANK0_INTR2_OFFSET: u32 = 0x000000f8;
pub(super) const IO_BANK0_INTR2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_INTR2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_INTR2_GPIO23_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_INTR2_GPIO23_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_INTR2_GPIO22_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_INTR2_GPIO22_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_INTR2_GPIO21_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_INTR2_GPIO21_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_INTR2_GPIO20_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_INTR2_GPIO20_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_INTR2_GPIO19_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_INTR2_GPIO19_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_INTR2_GPIO18_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_INTR2_GPIO18_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_INTR2_GPIO17_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_INTR2_GPIO17_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_INTR2_GPIO16_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_INTR2_GPIO16_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_INTR3
// Description : Raw Interrupts
pub(super) const IO_BANK0_INTR3_OFFSET: u32 = 0x000000fc;
pub(super) const IO_BANK0_INTR3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_INTR3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_INTR3_GPIO29_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_INTR3_GPIO29_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_INTR3_GPIO28_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_INTR3_GPIO28_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_INTR3_GPIO27_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_INTR3_GPIO27_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_INTR3_GPIO26_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_INTR3_GPIO26_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_INTR3_GPIO25_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_INTR3_GPIO25_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_HIGH_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_INTR3_GPIO24_EDGE_LOW_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_INTR3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_INTR3_GPIO24_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTE0
// Description : Interrupt Enable for proc0
pub(super) const IO_BANK0_PROC0_INTE0_OFFSET: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTE0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTE0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTE1
// Description : Interrupt Enable for proc0
pub(super) const IO_BANK0_PROC0_INTE1_OFFSET: u32 = 0x00000104;
pub(super) const IO_BANK0_PROC0_INTE1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTE1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTE2
// Description : Interrupt Enable for proc0
pub(super) const IO_BANK0_PROC0_INTE2_OFFSET: u32 = 0x00000108;
pub(super) const IO_BANK0_PROC0_INTE2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTE2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTE3
// Description : Interrupt Enable for proc0
pub(super) const IO_BANK0_PROC0_INTE3_OFFSET: u32 = 0x0000010c;
pub(super) const IO_BANK0_PROC0_INTE3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC0_INTE3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTE3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTF0
// Description : Interrupt Force for proc0
pub(super) const IO_BANK0_PROC0_INTF0_OFFSET: u32 = 0x00000110;
pub(super) const IO_BANK0_PROC0_INTF0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTF0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTF1
// Description : Interrupt Force for proc0
pub(super) const IO_BANK0_PROC0_INTF1_OFFSET: u32 = 0x00000114;
pub(super) const IO_BANK0_PROC0_INTF1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTF1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTF2
// Description : Interrupt Force for proc0
pub(super) const IO_BANK0_PROC0_INTF2_OFFSET: u32 = 0x00000118;
pub(super) const IO_BANK0_PROC0_INTF2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTF2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTF3
// Description : Interrupt Force for proc0
pub(super) const IO_BANK0_PROC0_INTF3_OFFSET: u32 = 0x0000011c;
pub(super) const IO_BANK0_PROC0_INTF3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC0_INTF3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTF3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTS0
// Description : Interrupt status after masking & forcing for proc0
pub(super) const IO_BANK0_PROC0_INTS0_OFFSET: u32 = 0x00000120;
pub(super) const IO_BANK0_PROC0_INTS0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTS0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO7_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO6_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO5_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO4_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO3_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO2_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO1_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS0_GPIO0_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTS1
// Description : Interrupt status after masking & forcing for proc0
pub(super) const IO_BANK0_PROC0_INTS1_OFFSET: u32 = 0x00000124;
pub(super) const IO_BANK0_PROC0_INTS1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTS1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO15_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO14_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO13_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO12_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO11_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO10_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO9_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS1_GPIO8_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTS2
// Description : Interrupt status after masking & forcing for proc0
pub(super) const IO_BANK0_PROC0_INTS2_OFFSET: u32 = 0x00000128;
pub(super) const IO_BANK0_PROC0_INTS2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC0_INTS2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO23_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO22_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO21_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO20_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO19_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO18_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO17_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS2_GPIO16_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC0_INTS3
// Description : Interrupt status after masking & forcing for proc0
pub(super) const IO_BANK0_PROC0_INTS3_OFFSET: u32 = 0x0000012c;
pub(super) const IO_BANK0_PROC0_INTS3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC0_INTS3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO29_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO28_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO27_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO26_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO25_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC0_INTS3_GPIO24_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTE0
// Description : Interrupt Enable for proc1
pub(super) const IO_BANK0_PROC1_INTE0_OFFSET: u32 = 0x00000130;
pub(super) const IO_BANK0_PROC1_INTE0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTE0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTE1
// Description : Interrupt Enable for proc1
pub(super) const IO_BANK0_PROC1_INTE1_OFFSET: u32 = 0x00000134;
pub(super) const IO_BANK0_PROC1_INTE1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTE1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTE2
// Description : Interrupt Enable for proc1
pub(super) const IO_BANK0_PROC1_INTE2_OFFSET: u32 = 0x00000138;
pub(super) const IO_BANK0_PROC1_INTE2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTE2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTE3
// Description : Interrupt Enable for proc1
pub(super) const IO_BANK0_PROC1_INTE3_OFFSET: u32 = 0x0000013c;
pub(super) const IO_BANK0_PROC1_INTE3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC1_INTE3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTE3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTF0
// Description : Interrupt Force for proc1
pub(super) const IO_BANK0_PROC1_INTF0_OFFSET: u32 = 0x00000140;
pub(super) const IO_BANK0_PROC1_INTF0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTF0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTF1
// Description : Interrupt Force for proc1
pub(super) const IO_BANK0_PROC1_INTF1_OFFSET: u32 = 0x00000144;
pub(super) const IO_BANK0_PROC1_INTF1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTF1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTF2
// Description : Interrupt Force for proc1
pub(super) const IO_BANK0_PROC1_INTF2_OFFSET: u32 = 0x00000148;
pub(super) const IO_BANK0_PROC1_INTF2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTF2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTF3
// Description : Interrupt Force for proc1
pub(super) const IO_BANK0_PROC1_INTF3_OFFSET: u32 = 0x0000014c;
pub(super) const IO_BANK0_PROC1_INTF3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC1_INTF3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTF3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTS0
// Description : Interrupt status after masking & forcing for proc1
pub(super) const IO_BANK0_PROC1_INTS0_OFFSET: u32 = 0x00000150;
pub(super) const IO_BANK0_PROC1_INTS0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTS0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO7_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO6_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO5_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO4_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO3_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO2_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO1_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS0_GPIO0_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTS1
// Description : Interrupt status after masking & forcing for proc1
pub(super) const IO_BANK0_PROC1_INTS1_OFFSET: u32 = 0x00000154;
pub(super) const IO_BANK0_PROC1_INTS1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTS1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO15_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO14_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO13_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO12_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO11_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO10_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO9_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS1_GPIO8_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTS2
// Description : Interrupt status after masking & forcing for proc1
pub(super) const IO_BANK0_PROC1_INTS2_OFFSET: u32 = 0x00000158;
pub(super) const IO_BANK0_PROC1_INTS2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_PROC1_INTS2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO23_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO22_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO21_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO20_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO19_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO18_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO17_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS2_GPIO16_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_PROC1_INTS3
// Description : Interrupt status after masking & forcing for proc1
pub(super) const IO_BANK0_PROC1_INTS3_OFFSET: u32 = 0x0000015c;
pub(super) const IO_BANK0_PROC1_INTS3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_PROC1_INTS3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO29_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO28_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO27_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO26_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO25_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_PROC1_INTS3_GPIO24_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTE0
// Description : Interrupt Enable for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_OFFSET: u32 = 0x00000160;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTE1
// Description : Interrupt Enable for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_OFFSET: u32 = 0x00000164;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTE2
// Description : Interrupt Enable for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_OFFSET: u32 = 0x00000168;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTE3
// Description : Interrupt Enable for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_OFFSET: u32 = 0x0000016c;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTE3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTF0
// Description : Interrupt Force for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_OFFSET: u32 = 0x00000170;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO7_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO6_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO5_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO4_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO3_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO2_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO1_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF0_GPIO0_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTF1
// Description : Interrupt Force for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_OFFSET: u32 = 0x00000174;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO15_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO14_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO13_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO12_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO11_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO10_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO9_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF1_GPIO8_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTF2
// Description : Interrupt Force for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_OFFSET: u32 = 0x00000178;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO23_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO22_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO21_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO20_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO19_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO18_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO17_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF2_GPIO16_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTF3
// Description : Interrupt Force for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_OFFSET: u32 = 0x0000017c;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO29_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO28_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO27_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO26_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO25_LEVEL_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_EDGE_LOW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTF3_GPIO24_LEVEL_LOW_ACCESS: &str = "RW";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTS0
// Description : Interrupt status after masking & forcing for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_OFFSET: u32 = 0x00000180;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO7_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO6_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO5_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO4_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO3_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO2_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO1_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS0_GPIO0_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTS1
// Description : Interrupt status after masking & forcing for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_OFFSET: u32 = 0x00000184;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO15_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO14_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO13_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO12_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO11_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO10_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO9_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS1_GPIO8_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTS2
// Description : Interrupt status after masking & forcing for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_OFFSET: u32 = 0x00000188;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_BITS: u32 = 0xffffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH_BITS: u32 = 0x80000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH_MSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH_LSB: i32 = 31;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW_BITS: u32 = 0x40000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW_MSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW_LSB: i32 = 30;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH_BITS: u32 = 0x20000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH_MSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH_LSB: i32 = 29;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW_BITS: u32 = 0x10000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW_MSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW_LSB: i32 = 28;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO23_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH_BITS: u32 = 0x08000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH_MSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH_LSB: i32 = 27;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW_BITS: u32 = 0x04000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW_MSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW_LSB: i32 = 26;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH_BITS: u32 = 0x02000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH_MSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH_LSB: i32 = 25;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW_BITS: u32 = 0x01000000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW_MSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW_LSB: i32 = 24;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO22_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO21_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO20_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO19_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO18_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO17_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS2_GPIO16_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
// Register    : IO_BANK0_DORMANT_WAKE_INTS3
// Description : Interrupt status after masking & forcing for dormant_wake
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_OFFSET: u32 = 0x0000018c;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_BITS: u32 = 0x00ffffff;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH_BITS: u32 = 0x00800000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH_MSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH_LSB: i32 = 23;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW_BITS: u32 = 0x00400000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW_MSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW_LSB: i32 = 22;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH_BITS: u32 = 0x00200000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH_MSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH_LSB: i32 = 21;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW_BITS: u32 = 0x00100000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW_MSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW_LSB: i32 = 20;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO29_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH_BITS: u32 = 0x00080000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH_MSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH_LSB: i32 = 19;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW_BITS: u32 = 0x00040000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW_MSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW_LSB: i32 = 18;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH_BITS: u32 = 0x00020000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH_MSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH_LSB: i32 = 17;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW_BITS: u32 = 0x00010000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW_MSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW_LSB: i32 = 16;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO28_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH_BITS: u32 = 0x00008000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH_MSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH_LSB: i32 = 15;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW_BITS: u32 = 0x00004000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW_MSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW_LSB: i32 = 14;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH_BITS: u32 = 0x00002000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH_MSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH_LSB: i32 = 13;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW_BITS: u32 = 0x00001000;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW_MSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW_LSB: i32 = 12;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO27_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH_BITS: u32 = 0x00000800;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH_MSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH_LSB: i32 = 11;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW_BITS: u32 = 0x00000400;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW_MSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW_LSB: i32 = 10;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH_BITS: u32 = 0x00000200;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH_MSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH_LSB: i32 = 9;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW_BITS: u32 = 0x00000100;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW_MSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW_LSB: i32 = 8;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO26_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH_BITS: u32 = 0x00000080;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH_MSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH_LSB: i32 = 7;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW_BITS: u32 = 0x00000040;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW_MSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW_LSB: i32 = 6;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH_BITS: u32 = 0x00000020;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH_MSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH_LSB: i32 = 5;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW_BITS: u32 = 0x00000010;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW_MSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW_LSB: i32 = 4;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO25_LEVEL_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH_BITS: u32 = 0x00000008;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH_MSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH_LSB: i32 = 3;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW_BITS: u32 = 0x00000004;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW_MSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW_LSB: i32 = 2;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_EDGE_LOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH_BITS: u32 = 0x00000002;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH_MSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH_LSB: i32 = 1;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_HIGH_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW
// Description : None
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW_RESET: u32 = 0x0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW_BITS: u32 = 0x00000001;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW_MSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW_LSB: i32 = 0;
pub(super) const IO_BANK0_DORMANT_WAKE_INTS3_GPIO24_LEVEL_LOW_ACCESS: &str = "RO";
// =============================================================================
