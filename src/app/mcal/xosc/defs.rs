// =============================================================================
// Register block : XOSC
// Version        : 1
// Bus type       : apb
// Description    : Controls the crystal oscillator
// =============================================================================
// =============================================================================
// Register    : XOSC_CTRL
// Description : Crystal Oscillator Control
pub(super) const XOSC_CTRL_OFFSET: u32 = 0x00000000;
pub(super) const XOSC_CTRL_BITS: u32 = 0x00ffffff;
pub(super) const XOSC_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : XOSC_CTRL_ENABLE
// Description : On power-up this field is initialised to DISABLE and the chip
//               runs from the ROSC.
//               If the chip has subsequently been programmed to run from the
//               XOSC then setting this field to DISABLE may lock-up the chip.
//               If this is a concern then run the clk_ref from the ROSC and
//               enable the clk_sys RESUS feature.
//               The 12-bit code is intended to give some protection against
//               accidental writes. An invalid setting will enable the
//               oscillator.
//               0xd1e -> DISABLE
//               0xfab -> ENABLE
pub(super) const XOSC_CTRL_ENABLE_RESET: &str = "-";
pub(super) const XOSC_CTRL_ENABLE_BITS: u32 = 0x00fff000;
pub(super) const XOSC_CTRL_ENABLE_MSB: i32 = 23;
pub(super) const XOSC_CTRL_ENABLE_LSB: i32 = 12;
pub(super) const XOSC_CTRL_ENABLE_ACCESS: &str = "RW";
pub(super) const XOSC_CTRL_ENABLE_VALUE_DISABLE: u32 = 0xd1e;
pub(super) const XOSC_CTRL_ENABLE_VALUE_ENABLE: u32 = 0xfab;
// -----------------------------------------------------------------------------
// Field       : XOSC_CTRL_FREQ_RANGE
// Description : Frequency range. This resets to 0xAA0 and cannot be changed.
//               0xaa0 -> 1_15MHZ
//               0xaa1 -> RESERVED_1
//               0xaa2 -> RESERVED_2
//               0xaa3 -> RESERVED_3
pub(super) const XOSC_CTRL_FREQ_RANGE_RESET: &str = "-";
pub(super) const XOSC_CTRL_FREQ_RANGE_BITS: u32 = 0x00000fff;
pub(super) const XOSC_CTRL_FREQ_RANGE_MSB: i32 = 11;
pub(super) const XOSC_CTRL_FREQ_RANGE_LSB: i32 = 0;
pub(super) const XOSC_CTRL_FREQ_RANGE_ACCESS: &str = "RW";
pub(super) const XOSC_CTRL_FREQ_RANGE_VALUE_1_15MHZ: u32 = 0xaa0;
pub(super) const XOSC_CTRL_FREQ_RANGE_VALUE_RESERVED_1: u32 = 0xaa1;
pub(super) const XOSC_CTRL_FREQ_RANGE_VALUE_RESERVED_2: u32 = 0xaa2;
pub(super) const XOSC_CTRL_FREQ_RANGE_VALUE_RESERVED_3: u32 = 0xaa3;
// =============================================================================
// Register    : XOSC_STATUS
// Description : Crystal Oscillator Status
pub(super) const XOSC_STATUS_OFFSET: u32 = 0x00000004;
pub(super) const XOSC_STATUS_BITS: u32 = 0x81001003;
pub(super) const XOSC_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : XOSC_STATUS_STABLE
// Description : Oscillator is running and stable
pub(super) const XOSC_STATUS_STABLE_RESET: u32 = 0x0;
pub(super) const XOSC_STATUS_STABLE_BITS: u32 = 0x80000000;
pub(super) const XOSC_STATUS_STABLE_MSB: i32 = 31;
pub(super) const XOSC_STATUS_STABLE_LSB: i32 = 31;
pub(super) const XOSC_STATUS_STABLE_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : XOSC_STATUS_BADWRITE
// Description : An invalid value has been written to CTRL_ENABLE or
//               CTRL_FREQ_RANGE or DORMANT
pub(super) const XOSC_STATUS_BADWRITE_RESET: u32 = 0x0;
pub(super) const XOSC_STATUS_BADWRITE_BITS: u32 = 0x01000000;
pub(super) const XOSC_STATUS_BADWRITE_MSB: i32 = 24;
pub(super) const XOSC_STATUS_BADWRITE_LSB: i32 = 24;
pub(super) const XOSC_STATUS_BADWRITE_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : XOSC_STATUS_ENABLED
// Description : Oscillator is enabled but not necessarily running and stable,
//               resets to 0
pub(super) const XOSC_STATUS_ENABLED_RESET: &str = "-";
pub(super) const XOSC_STATUS_ENABLED_BITS: u32 = 0x00001000;
pub(super) const XOSC_STATUS_ENABLED_MSB: i32 = 12;
pub(super) const XOSC_STATUS_ENABLED_LSB: i32 = 12;
pub(super) const XOSC_STATUS_ENABLED_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : XOSC_STATUS_FREQ_RANGE
// Description : The current frequency range setting, always reads 0
//               0x0 -> 1_15MHZ
//               0x1 -> RESERVED_1
//               0x2 -> RESERVED_2
//               0x3 -> RESERVED_3
pub(super) const XOSC_STATUS_FREQ_RANGE_RESET: &str = "-";
pub(super) const XOSC_STATUS_FREQ_RANGE_BITS: u32 = 0x00000003;
pub(super) const XOSC_STATUS_FREQ_RANGE_MSB: i32 = 1;
pub(super) const XOSC_STATUS_FREQ_RANGE_LSB: i32 = 0;
pub(super) const XOSC_STATUS_FREQ_RANGE_ACCESS: &str = "RO";
pub(super) const XOSC_STATUS_FREQ_RANGE_VALUE_1_15MHZ: u32 = 0x0;
pub(super) const XOSC_STATUS_FREQ_RANGE_VALUE_RESERVED_1: u32 = 0x1;
pub(super) const XOSC_STATUS_FREQ_RANGE_VALUE_RESERVED_2: u32 = 0x2;
pub(super) const XOSC_STATUS_FREQ_RANGE_VALUE_RESERVED_3: u32 = 0x3;
// =============================================================================
// Register    : XOSC_DORMANT
// Description : Crystal Oscillator pause control
//               This is used to save power by pausing the XOSC
//               On power-up this field is initialised to WAKE
//               An invalid write will also select WAKE
//               WARNING: stop the PLLs before selecting dormant mode
//               WARNING: setup the irq before selecting dormant mode
//               0x636f6d61 -> DORMANT
//               0x77616b65 -> WAKE
pub(super) const XOSC_DORMANT_OFFSET: u32 = 0x00000008;
pub(super) const XOSC_DORMANT_BITS: u32 = 0xffffffff;
pub(super) const XOSC_DORMANT_RESET: &str = "-";
pub(super) const XOSC_DORMANT_MSB: i32 = 31;
pub(super) const XOSC_DORMANT_LSB: i32 = 0;
pub(super) const XOSC_DORMANT_ACCESS: &str = "RW";
pub(super) const XOSC_DORMANT_VALUE_DORMANT: u32 = 0x636f6d61;
pub(super) const XOSC_DORMANT_VALUE_WAKE: u32 = 0x77616b65;
// =============================================================================
// Register    : XOSC_STARTUP
// Description : Controls the startup delay
pub(super) const XOSC_STARTUP_OFFSET: u32 = 0x0000000c;
pub(super) const XOSC_STARTUP_BITS: u32 = 0x00103fff;
pub(super) const XOSC_STARTUP_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : XOSC_STARTUP_X4
// Description : Multiplies the startup_delay by 4. This is of little value to
//               the user given that the delay can be programmed directly
pub(super) const XOSC_STARTUP_X4_RESET: &str = "-";
pub(super) const XOSC_STARTUP_X4_BITS: u32 = 0x00100000;
pub(super) const XOSC_STARTUP_X4_MSB: i32 = 20;
pub(super) const XOSC_STARTUP_X4_LSB: i32 = 20;
pub(super) const XOSC_STARTUP_X4_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : XOSC_STARTUP_DELAY
// Description : in multiples of 256*xtal_period
pub(super) const XOSC_STARTUP_DELAY_RESET: &str = "-";
pub(super) const XOSC_STARTUP_DELAY_BITS: u32 = 0x00003fff;
pub(super) const XOSC_STARTUP_DELAY_MSB: i32 = 13;
pub(super) const XOSC_STARTUP_DELAY_LSB: i32 = 0;
pub(super) const XOSC_STARTUP_DELAY_ACCESS: &str = "RW";
// =============================================================================
// Register    : XOSC_COUNT
// Description : A down counter running at the xosc frequency which counts to
//               zero and stops.
//               To start the counter write a non-zero value.
//               Can be used for short software pauses when setting up time
//               sensitive hardware.
pub(super) const XOSC_COUNT_OFFSET: u32 = 0x0000001c;
pub(super) const XOSC_COUNT_BITS: u32 = 0x000000ff;
pub(super) const XOSC_COUNT_RESET: u32 = 0x00000000;
pub(super) const XOSC_COUNT_MSB: i32 = 7;
pub(super) const XOSC_COUNT_LSB: i32 = 0;
pub(super) const XOSC_COUNT_ACCESS: &str = "RW";
// =============================================================================
