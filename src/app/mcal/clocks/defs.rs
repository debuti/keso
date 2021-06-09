// =============================================================================
// Register block : CLOCKS
// Version        : 1
// Bus type       : apb
// Description    : None
// =============================================================================
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT0_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_OFFSET: u32 = 0x00000000;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_BITS: u32 = 0x00131de0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_DC50
// Description : Enables duty cycle correction for odd divisors
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_DC50_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_DC50_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_DC50_MSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_DC50_LSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_DC50_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_sys
//               0x1 -> clksrc_gpin0
//               0x2 -> clksrc_gpin1
//               0x3 -> clksrc_pll_usb
//               0x4 -> rosc_clksrc
//               0x5 -> xosc_clksrc
//               0x6 -> clk_sys
//               0x7 -> clk_usb
//               0x8 -> clk_adc
//               0x9 -> clk_rtc
//               0xa -> clk_ref
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_BITS: u32 = 0x000001e0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_MSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x1;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x2;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x3;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_ROSC_CLKSRC: u32 = 0x4;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x5;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_SYS: u32 = 0x6;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_USB: u32 = 0x7;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_ADC: u32 = 0x8;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_RTC: u32 = 0x9;
pub(super) const CLOCKS_CLK_GPOUT0_CTRL_AUXSRC_VALUE_CLK_REF: u32 = 0xa;
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT0_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_GPOUT0_DIV_OFFSET: u32 = 0x00000004;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_GPOUT0_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT0_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_GPOUT0_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT0_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT0_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_OFFSET: u32 = 0x00000008;
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT0_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT1_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_OFFSET: u32 = 0x0000000c;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_BITS: u32 = 0x00131de0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_DC50
// Description : Enables duty cycle correction for odd divisors
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_DC50_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_DC50_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_DC50_MSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_DC50_LSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_DC50_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_sys
//               0x1 -> clksrc_gpin0
//               0x2 -> clksrc_gpin1
//               0x3 -> clksrc_pll_usb
//               0x4 -> rosc_clksrc
//               0x5 -> xosc_clksrc
//               0x6 -> clk_sys
//               0x7 -> clk_usb
//               0x8 -> clk_adc
//               0x9 -> clk_rtc
//               0xa -> clk_ref
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_BITS: u32 = 0x000001e0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_MSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x1;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x2;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x3;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_ROSC_CLKSRC: u32 = 0x4;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x5;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLK_SYS: u32 = 0x6;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLK_USB: u32 = 0x7;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLK_ADC: u32 = 0x8;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLK_RTC: u32 = 0x9;
pub(super) const CLOCKS_CLK_GPOUT1_CTRL_AUXSRC_VALUE_CLK_REF: u32 = 0xa;
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT1_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_GPOUT1_DIV_OFFSET: u32 = 0x00000010;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_GPOUT1_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT1_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_GPOUT1_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT1_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT1_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_OFFSET: u32 = 0x00000014;
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT1_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT2_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_OFFSET: u32 = 0x00000018;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_BITS: u32 = 0x00131de0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_DC50
// Description : Enables duty cycle correction for odd divisors
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_DC50_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_DC50_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_DC50_MSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_DC50_LSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_DC50_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_sys
//               0x1 -> clksrc_gpin0
//               0x2 -> clksrc_gpin1
//               0x3 -> clksrc_pll_usb
//               0x4 -> rosc_clksrc_ph
//               0x5 -> xosc_clksrc
//               0x6 -> clk_sys
//               0x7 -> clk_usb
//               0x8 -> clk_adc
//               0x9 -> clk_rtc
//               0xa -> clk_ref
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_BITS: u32 = 0x000001e0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_MSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x1;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x2;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x3;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x4;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x5;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLK_SYS: u32 = 0x6;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLK_USB: u32 = 0x7;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLK_ADC: u32 = 0x8;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLK_RTC: u32 = 0x9;
pub(super) const CLOCKS_CLK_GPOUT2_CTRL_AUXSRC_VALUE_CLK_REF: u32 = 0xa;
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT2_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_GPOUT2_DIV_OFFSET: u32 = 0x0000001c;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_GPOUT2_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT2_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_GPOUT2_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT2_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT2_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_OFFSET: u32 = 0x00000020;
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT2_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT3_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_OFFSET: u32 = 0x00000024;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_BITS: u32 = 0x00131de0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_DC50
// Description : Enables duty cycle correction for odd divisors
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_DC50_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_DC50_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_DC50_MSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_DC50_LSB: i32 = 12;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_DC50_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_sys
//               0x1 -> clksrc_gpin0
//               0x2 -> clksrc_gpin1
//               0x3 -> clksrc_pll_usb
//               0x4 -> rosc_clksrc_ph
//               0x5 -> xosc_clksrc
//               0x6 -> clk_sys
//               0x7 -> clk_usb
//               0x8 -> clk_adc
//               0x9 -> clk_rtc
//               0xa -> clk_ref
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_BITS: u32 = 0x000001e0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_MSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x1;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x2;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x3;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x4;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x5;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLK_SYS: u32 = 0x6;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLK_USB: u32 = 0x7;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLK_ADC: u32 = 0x8;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLK_RTC: u32 = 0x9;
pub(super) const CLOCKS_CLK_GPOUT3_CTRL_AUXSRC_VALUE_CLK_REF: u32 = 0xa;
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT3_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_GPOUT3_DIV_OFFSET: u32 = 0x00000028;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_GPOUT3_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_GPOUT3_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_GPOUT3_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT3_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_GPOUT3_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_OFFSET: u32 = 0x0000002c;
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_GPOUT3_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_REF_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_REF_CTRL_OFFSET: u32 = 0x00000030;
pub(super) const CLOCKS_CLK_REF_CTRL_BITS: u32 = 0x00000063;
pub(super) const CLOCKS_CLK_REF_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_REF_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_usb
//               0x1 -> clksrc_gpin0
//               0x2 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_BITS: u32 = 0x00000060;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_MSB: i32 = 6;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x0;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x1;
pub(super) const CLOCKS_CLK_REF_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x2;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_REF_CTRL_SRC
// Description : Selects the clock source glitchlessly, can be changed
//               on-the-fly
//               0x0 -> rosc_clksrc_ph
//               0x1 -> clksrc_clk_ref_aux
//               0x2 -> xosc_clksrc
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_RESET: &str = "-";
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_BITS: u32 = 0x00000003;
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_MSB: i32 = 1;
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x0;
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_VALUE_CLKSRC_CLK_REF_AUX: u32 = 0x1;
pub(super) const CLOCKS_CLK_REF_CTRL_SRC_VALUE_XOSC_CLKSRC: u32 = 0x2;
// =============================================================================
// Register    : CLOCKS_CLK_REF_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_REF_DIV_OFFSET: u32 = 0x00000034;
pub(super) const CLOCKS_CLK_REF_DIV_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_REF_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_REF_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_REF_DIV_INT_RESET: u32 = 0x1;
pub(super) const CLOCKS_CLK_REF_DIV_INT_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_REF_DIV_INT_MSB: i32 = 9;
pub(super) const CLOCKS_CLK_REF_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_REF_DIV_INT_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_REF_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               The glitchless multiplexer does not switch instantaneously (to
//               avoid glitches), so software should poll this register to wait
//               for the switch to complete. This register contains one decoded
//               bit for each of the clock sources enumerated in the CTRL SRC
//               field. At most one of these bits will be set at any time,
//               indicating that clock is currently present at the output of the
//               glitchless mux. Whilst switching is in progress, this register
//               may briefly show all-0s.
pub(super) const CLOCKS_CLK_REF_SELECTED_OFFSET: u32 = 0x00000038;
pub(super) const CLOCKS_CLK_REF_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_REF_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_REF_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_REF_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_REF_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_SYS_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_SYS_CTRL_OFFSET: u32 = 0x0000003c;
pub(super) const CLOCKS_CLK_SYS_CTRL_BITS: u32 = 0x000000e1;
pub(super) const CLOCKS_CLK_SYS_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_sys
//               0x1 -> clksrc_pll_usb
//               0x2 -> rosc_clksrc
//               0x3 -> xosc_clksrc
//               0x4 -> clksrc_gpin0
//               0x5 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_BITS: u32 = 0x000000e0;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x1;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_ROSC_CLKSRC: u32 = 0x2;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x3;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x4;
pub(super) const CLOCKS_CLK_SYS_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x5;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_CTRL_SRC
// Description : Selects the clock source glitchlessly, can be changed
//               on-the-fly
//               0x0 -> clk_ref
//               0x1 -> clksrc_clk_sys_aux
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_MSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_VALUE_CLK_REF: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_CTRL_SRC_VALUE_CLKSRC_CLK_SYS_AUX: u32 = 0x1;
// =============================================================================
// Register    : CLOCKS_CLK_SYS_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_SYS_DIV_OFFSET: u32 = 0x00000040;
pub(super) const CLOCKS_CLK_SYS_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_SYS_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_SYS_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_SYS_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_SYS_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_SYS_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_SYS_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_SYS_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_SYS_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_SYS_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_SYS_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_SYS_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               The glitchless multiplexer does not switch instantaneously (to
//               avoid glitches), so software should poll this register to wait
//               for the switch to complete. This register contains one decoded
//               bit for each of the clock sources enumerated in the CTRL SRC
//               field. At most one of these bits will be set at any time,
//               indicating that clock is currently present at the output of the
//               glitchless mux. Whilst switching is in progress, this register
//               may briefly show all-0s.
pub(super) const CLOCKS_CLK_SYS_SELECTED_OFFSET: u32 = 0x00000044;
pub(super) const CLOCKS_CLK_SYS_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_SYS_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_SYS_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_SYS_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_PERI_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_PERI_CTRL_OFFSET: u32 = 0x00000048;
pub(super) const CLOCKS_CLK_PERI_CTRL_BITS: u32 = 0x00000ce0;
pub(super) const CLOCKS_CLK_PERI_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_PERI_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_PERI_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_PERI_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_PERI_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_PERI_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_PERI_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_PERI_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_PERI_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_PERI_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_PERI_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_PERI_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_PERI_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_PERI_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clk_sys
//               0x1 -> clksrc_pll_sys
//               0x2 -> clksrc_pll_usb
//               0x3 -> rosc_clksrc_ph
//               0x4 -> xosc_clksrc
//               0x5 -> clksrc_gpin0
//               0x6 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_BITS: u32 = 0x000000e0;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLK_SYS: u32 = 0x0;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x1;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x2;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x3;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x4;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x5;
pub(super) const CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x6;
// =============================================================================
// Register    : CLOCKS_CLK_PERI_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_PERI_SELECTED_OFFSET: u32 = 0x00000050;
pub(super) const CLOCKS_CLK_PERI_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_PERI_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_PERI_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_PERI_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_PERI_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_USB_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_USB_CTRL_OFFSET: u32 = 0x00000054;
pub(super) const CLOCKS_CLK_USB_CTRL_BITS: u32 = 0x00130ce0;
pub(super) const CLOCKS_CLK_USB_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_USB_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_USB_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_USB_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_USB_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_USB_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_USB_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_USB_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_USB_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_USB_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_USB_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_USB_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_USB_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_USB_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_USB_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_USB_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_USB_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_usb
//               0x1 -> clksrc_pll_sys
//               0x2 -> rosc_clksrc_ph
//               0x3 -> xosc_clksrc
//               0x4 -> clksrc_gpin0
//               0x5 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_BITS: u32 = 0x000000e0;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x0;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x1;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x2;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x3;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x4;
pub(super) const CLOCKS_CLK_USB_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x5;
// =============================================================================
// Register    : CLOCKS_CLK_USB_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_USB_DIV_OFFSET: u32 = 0x00000058;
pub(super) const CLOCKS_CLK_USB_DIV_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_USB_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_USB_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_USB_DIV_INT_RESET: u32 = 0x1;
pub(super) const CLOCKS_CLK_USB_DIV_INT_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_USB_DIV_INT_MSB: i32 = 9;
pub(super) const CLOCKS_CLK_USB_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_USB_DIV_INT_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_USB_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_USB_SELECTED_OFFSET: u32 = 0x0000005c;
pub(super) const CLOCKS_CLK_USB_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_USB_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_USB_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_USB_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_USB_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_ADC_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_ADC_CTRL_OFFSET: u32 = 0x00000060;
pub(super) const CLOCKS_CLK_ADC_CTRL_BITS: u32 = 0x00130ce0;
pub(super) const CLOCKS_CLK_ADC_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_ADC_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_ADC_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_ADC_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_ADC_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_ADC_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_ADC_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_ADC_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_ADC_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_ADC_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_ADC_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_ADC_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_ADC_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_ADC_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_ADC_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_ADC_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_ADC_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_usb
//               0x1 -> clksrc_pll_sys
//               0x2 -> rosc_clksrc_ph
//               0x3 -> xosc_clksrc
//               0x4 -> clksrc_gpin0
//               0x5 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_BITS: u32 = 0x000000e0;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x0;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x1;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x2;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x3;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x4;
pub(super) const CLOCKS_CLK_ADC_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x5;
// =============================================================================
// Register    : CLOCKS_CLK_ADC_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_ADC_DIV_OFFSET: u32 = 0x00000064;
pub(super) const CLOCKS_CLK_ADC_DIV_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_ADC_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_ADC_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_ADC_DIV_INT_RESET: u32 = 0x1;
pub(super) const CLOCKS_CLK_ADC_DIV_INT_BITS: u32 = 0x00000300;
pub(super) const CLOCKS_CLK_ADC_DIV_INT_MSB: i32 = 9;
pub(super) const CLOCKS_CLK_ADC_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_ADC_DIV_INT_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_ADC_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_ADC_SELECTED_OFFSET: u32 = 0x00000068;
pub(super) const CLOCKS_CLK_ADC_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_ADC_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_ADC_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_ADC_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_ADC_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_RTC_CTRL
// Description : Clock control, can be changed on-the-fly (except for auxsrc)
pub(super) const CLOCKS_CLK_RTC_CTRL_OFFSET: u32 = 0x0000006c;
pub(super) const CLOCKS_CLK_RTC_CTRL_BITS: u32 = 0x00130ce0;
pub(super) const CLOCKS_CLK_RTC_CTRL_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_CTRL_NUDGE
// Description : An edge on this signal shifts the phase of the output by 1
//               cycle of the input clock
//               This can be done at any time
pub(super) const CLOCKS_CLK_RTC_CTRL_NUDGE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_NUDGE_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_CLK_RTC_CTRL_NUDGE_MSB: i32 = 20;
pub(super) const CLOCKS_CLK_RTC_CTRL_NUDGE_LSB: i32 = 20;
pub(super) const CLOCKS_CLK_RTC_CTRL_NUDGE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_CTRL_PHASE
// Description : This delays the enable signal by up to 3 cycles of the input
//               clock
//               This must be set before the clock is enabled to have any effect
pub(super) const CLOCKS_CLK_RTC_CTRL_PHASE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_PHASE_BITS: u32 = 0x00030000;
pub(super) const CLOCKS_CLK_RTC_CTRL_PHASE_MSB: i32 = 17;
pub(super) const CLOCKS_CLK_RTC_CTRL_PHASE_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_RTC_CTRL_PHASE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_CTRL_ENABLE
// Description : Starts and stops the clock generator cleanly
pub(super) const CLOCKS_CLK_RTC_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_ENABLE_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_CLK_RTC_CTRL_ENABLE_MSB: i32 = 11;
pub(super) const CLOCKS_CLK_RTC_CTRL_ENABLE_LSB: i32 = 11;
pub(super) const CLOCKS_CLK_RTC_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_CTRL_KILL
// Description : Asynchronously kills the clock generator
pub(super) const CLOCKS_CLK_RTC_CTRL_KILL_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_KILL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_CLK_RTC_CTRL_KILL_MSB: i32 = 10;
pub(super) const CLOCKS_CLK_RTC_CTRL_KILL_LSB: i32 = 10;
pub(super) const CLOCKS_CLK_RTC_CTRL_KILL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_CTRL_AUXSRC
// Description : Selects the auxiliary clock source, will glitch when switching
//               0x0 -> clksrc_pll_usb
//               0x1 -> clksrc_pll_sys
//               0x2 -> rosc_clksrc_ph
//               0x3 -> xosc_clksrc
//               0x4 -> clksrc_gpin0
//               0x5 -> clksrc_gpin1
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_BITS: u32 = 0x000000e0;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_LSB: i32 = 5;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_CLKSRC_PLL_USB: u32 = 0x0;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS: u32 = 0x1;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x2;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_XOSC_CLKSRC: u32 = 0x3;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_CLKSRC_GPIN0: u32 = 0x4;
pub(super) const CLOCKS_CLK_RTC_CTRL_AUXSRC_VALUE_CLKSRC_GPIN1: u32 = 0x5;
// =============================================================================
// Register    : CLOCKS_CLK_RTC_DIV
// Description : Clock divisor, can be changed on-the-fly
pub(super) const CLOCKS_CLK_RTC_DIV_OFFSET: u32 = 0x00000070;
pub(super) const CLOCKS_CLK_RTC_DIV_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_RTC_DIV_RESET: u32 = 0x00000100;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_DIV_INT
// Description : Integer component of the divisor, 0 -> divide by 2^16
pub(super) const CLOCKS_CLK_RTC_DIV_INT_RESET: u32 = 0x000001;
pub(super) const CLOCKS_CLK_RTC_DIV_INT_BITS: u32 = 0xffffff00;
pub(super) const CLOCKS_CLK_RTC_DIV_INT_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_RTC_DIV_INT_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_RTC_DIV_INT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_RTC_DIV_FRAC
// Description : Fractional component of the divisor
pub(super) const CLOCKS_CLK_RTC_DIV_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_CLK_RTC_DIV_FRAC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_RTC_DIV_FRAC_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_RTC_DIV_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_RTC_DIV_FRAC_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_RTC_SELECTED
// Description : Indicates which SRC is currently selected by the glitchless mux
//               (one-hot).
//               This slice does not have a glitchless mux (only the AUX_SRC
//               field is present, not SRC) so this register is hardwired to
//               0x1.
pub(super) const CLOCKS_CLK_RTC_SELECTED_OFFSET: u32 = 0x00000074;
pub(super) const CLOCKS_CLK_RTC_SELECTED_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_CLK_RTC_SELECTED_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_RTC_SELECTED_MSB: i32 = 31;
pub(super) const CLOCKS_CLK_RTC_SELECTED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_RTC_SELECTED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_CLK_SYS_RESUS_CTRL
// Description : None
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_OFFSET: u32 = 0x00000078;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_BITS: u32 = 0x000111ff;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_RESET: u32 = 0x000000ff;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR
// Description : For clearing the resus after the fault that triggered it has
//               been corrected
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_BITS: u32 = 0x00010000;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_MSB: i32 = 16;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_LSB: i32 = 16;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_CLEAR_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_RESUS_CTRL_FRCE
// Description : Force a resus, for test purposes only
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_MSB: i32 = 12;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_LSB: i32 = 12;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_FRCE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE
// Description : Enable resus
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_MSB: i32 = 8;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_LSB: i32 = 8;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT
// Description : This is expressed as a number of clk_ref cycles
//               and must be >= 2x clk_ref_freq/min_clk_tst_freq
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_RESET: u32 = 0xff;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_MSB: i32 = 7;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_RESUS_CTRL_TIMEOUT_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_CLK_SYS_RESUS_STATUS
// Description : None
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_OFFSET: u32 = 0x0000007c;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED
// Description : Clock has been resuscitated, correct the error then send
//               ctrl_clear=1
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_RESET: u32 = 0x0;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_MSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_LSB: i32 = 0;
pub(super) const CLOCKS_CLK_SYS_RESUS_STATUS_RESUSSED_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_FC0_REF_KHZ
// Description : Reference clock frequency in kHz
pub(super) const CLOCKS_FC0_REF_KHZ_OFFSET: u32 = 0x00000080;
pub(super) const CLOCKS_FC0_REF_KHZ_BITS: u32 = 0x000fffff;
pub(super) const CLOCKS_FC0_REF_KHZ_RESET: u32 = 0x00000000;
pub(super) const CLOCKS_FC0_REF_KHZ_MSB: i32 = 19;
pub(super) const CLOCKS_FC0_REF_KHZ_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_REF_KHZ_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_FC0_MIN_KHZ
// Description : Minimum pass frequency in kHz. This is optional. Set to 0 if
//               you are not using the pass/fail flags
pub(super) const CLOCKS_FC0_MIN_KHZ_OFFSET: u32 = 0x00000084;
pub(super) const CLOCKS_FC0_MIN_KHZ_BITS: u32 = 0x01ffffff;
pub(super) const CLOCKS_FC0_MIN_KHZ_RESET: u32 = 0x00000000;
pub(super) const CLOCKS_FC0_MIN_KHZ_MSB: i32 = 24;
pub(super) const CLOCKS_FC0_MIN_KHZ_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_MIN_KHZ_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_FC0_MAX_KHZ
// Description : Maximum pass frequency in kHz. This is optional. Set to
//               0x1ffffff if you are not using the pass/fail flags
pub(super) const CLOCKS_FC0_MAX_KHZ_OFFSET: u32 = 0x00000088;
pub(super) const CLOCKS_FC0_MAX_KHZ_BITS: u32 = 0x01ffffff;
pub(super) const CLOCKS_FC0_MAX_KHZ_RESET: u32 = 0x01ffffff;
pub(super) const CLOCKS_FC0_MAX_KHZ_MSB: i32 = 24;
pub(super) const CLOCKS_FC0_MAX_KHZ_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_MAX_KHZ_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_FC0_DELAY
// Description : Delays the start of frequency counting to allow the mux to
//               settle
//               Delay is measured in multiples of the reference clock period
pub(super) const CLOCKS_FC0_DELAY_OFFSET: u32 = 0x0000008c;
pub(super) const CLOCKS_FC0_DELAY_BITS: u32 = 0x00000007;
pub(super) const CLOCKS_FC0_DELAY_RESET: u32 = 0x00000001;
pub(super) const CLOCKS_FC0_DELAY_MSB: i32 = 2;
pub(super) const CLOCKS_FC0_DELAY_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_DELAY_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_FC0_INTERVAL
// Description : The test interval is 0.98us * 2**interval, but let's call it
//               1us * 2**interval
//               The default gives a test interval of 250us
pub(super) const CLOCKS_FC0_INTERVAL_OFFSET: u32 = 0x00000090;
pub(super) const CLOCKS_FC0_INTERVAL_BITS: u32 = 0x0000000f;
pub(super) const CLOCKS_FC0_INTERVAL_RESET: u32 = 0x00000008;
pub(super) const CLOCKS_FC0_INTERVAL_MSB: i32 = 3;
pub(super) const CLOCKS_FC0_INTERVAL_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_INTERVAL_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_FC0_SRC
// Description : Clock sent to frequency counter, set to 0 when not required
//               Writing to this register initiates the frequency count
//               0x00 -> NULL
//               0x01 -> pll_sys_clksrc_primary
//               0x02 -> pll_usb_clksrc_primary
//               0x03 -> rosc_clksrc
//               0x04 -> rosc_clksrc_ph
//               0x05 -> xosc_clksrc
//               0x06 -> clksrc_gpin0
//               0x07 -> clksrc_gpin1
//               0x08 -> clk_ref
//               0x09 -> clk_sys
//               0x0a -> clk_peri
//               0x0b -> clk_usb
//               0x0c -> clk_adc
//               0x0d -> clk_rtc
pub(super) const CLOCKS_FC0_SRC_OFFSET: u32 = 0x00000094;
pub(super) const CLOCKS_FC0_SRC_BITS: u32 = 0x000000ff;
pub(super) const CLOCKS_FC0_SRC_RESET: u32 = 0x00000000;
pub(super) const CLOCKS_FC0_SRC_MSB: i32 = 7;
pub(super) const CLOCKS_FC0_SRC_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_SRC_ACCESS: &str = "RW";
pub(super) const CLOCKS_FC0_SRC_VALUE_NULL: u32 = 0x00;
pub(super) const CLOCKS_FC0_SRC_VALUE_PLL_SYS_CLKSRC_PRIMARY: u32 = 0x01;
pub(super) const CLOCKS_FC0_SRC_VALUE_PLL_USB_CLKSRC_PRIMARY: u32 = 0x02;
pub(super) const CLOCKS_FC0_SRC_VALUE_ROSC_CLKSRC: u32 = 0x03;
pub(super) const CLOCKS_FC0_SRC_VALUE_ROSC_CLKSRC_PH: u32 = 0x04;
pub(super) const CLOCKS_FC0_SRC_VALUE_XOSC_CLKSRC: u32 = 0x05;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLKSRC_GPIN0: u32 = 0x06;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLKSRC_GPIN1: u32 = 0x07;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_REF: u32 = 0x08;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_SYS: u32 = 0x09;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_PERI: u32 = 0x0a;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_USB: u32 = 0x0b;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_ADC: u32 = 0x0c;
pub(super) const CLOCKS_FC0_SRC_VALUE_CLK_RTC: u32 = 0x0d;
// =============================================================================
// Register    : CLOCKS_FC0_STATUS
// Description : Frequency counter status
pub(super) const CLOCKS_FC0_STATUS_OFFSET: u32 = 0x00000098;
pub(super) const CLOCKS_FC0_STATUS_BITS: u32 = 0x11111111;
pub(super) const CLOCKS_FC0_STATUS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_DIED
// Description : Test clock stopped during test
pub(super) const CLOCKS_FC0_STATUS_DIED_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_DIED_BITS: u32 = 0x10000000;
pub(super) const CLOCKS_FC0_STATUS_DIED_MSB: i32 = 28;
pub(super) const CLOCKS_FC0_STATUS_DIED_LSB: i32 = 28;
pub(super) const CLOCKS_FC0_STATUS_DIED_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_FAST
// Description : Test clock faster than expected, only valid when status_done=1
pub(super) const CLOCKS_FC0_STATUS_FAST_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_FAST_BITS: u32 = 0x01000000;
pub(super) const CLOCKS_FC0_STATUS_FAST_MSB: i32 = 24;
pub(super) const CLOCKS_FC0_STATUS_FAST_LSB: i32 = 24;
pub(super) const CLOCKS_FC0_STATUS_FAST_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_SLOW
// Description : Test clock slower than expected, only valid when status_done=1
pub(super) const CLOCKS_FC0_STATUS_SLOW_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_SLOW_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_FC0_STATUS_SLOW_MSB: i32 = 20;
pub(super) const CLOCKS_FC0_STATUS_SLOW_LSB: i32 = 20;
pub(super) const CLOCKS_FC0_STATUS_SLOW_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_FAIL
// Description : Test failed
pub(super) const CLOCKS_FC0_STATUS_FAIL_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_FAIL_BITS: u32 = 0x00010000;
pub(super) const CLOCKS_FC0_STATUS_FAIL_MSB: i32 = 16;
pub(super) const CLOCKS_FC0_STATUS_FAIL_LSB: i32 = 16;
pub(super) const CLOCKS_FC0_STATUS_FAIL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_WAITING
// Description : Waiting for test clock to start
pub(super) const CLOCKS_FC0_STATUS_WAITING_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_WAITING_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_FC0_STATUS_WAITING_MSB: i32 = 12;
pub(super) const CLOCKS_FC0_STATUS_WAITING_LSB: i32 = 12;
pub(super) const CLOCKS_FC0_STATUS_WAITING_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_RUNNING
// Description : Test running
pub(super) const CLOCKS_FC0_STATUS_RUNNING_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_RUNNING_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_FC0_STATUS_RUNNING_MSB: i32 = 8;
pub(super) const CLOCKS_FC0_STATUS_RUNNING_LSB: i32 = 8;
pub(super) const CLOCKS_FC0_STATUS_RUNNING_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_DONE
// Description : Test complete
pub(super) const CLOCKS_FC0_STATUS_DONE_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_DONE_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_FC0_STATUS_DONE_MSB: i32 = 4;
pub(super) const CLOCKS_FC0_STATUS_DONE_LSB: i32 = 4;
pub(super) const CLOCKS_FC0_STATUS_DONE_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_STATUS_PASS
// Description : Test passed
pub(super) const CLOCKS_FC0_STATUS_PASS_RESET: u32 = 0x0;
pub(super) const CLOCKS_FC0_STATUS_PASS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_FC0_STATUS_PASS_MSB: i32 = 0;
pub(super) const CLOCKS_FC0_STATUS_PASS_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_STATUS_PASS_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_FC0_RESULT
// Description : Result of frequency measurement, only valid when status_done=1
pub(super) const CLOCKS_FC0_RESULT_OFFSET: u32 = 0x0000009c;
pub(super) const CLOCKS_FC0_RESULT_BITS: u32 = 0x3fffffff;
pub(super) const CLOCKS_FC0_RESULT_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_RESULT_KHZ
// Description : None
pub(super) const CLOCKS_FC0_RESULT_KHZ_RESET: u32 = 0x0000000;
pub(super) const CLOCKS_FC0_RESULT_KHZ_BITS: u32 = 0x3fffffe0;
pub(super) const CLOCKS_FC0_RESULT_KHZ_MSB: i32 = 29;
pub(super) const CLOCKS_FC0_RESULT_KHZ_LSB: i32 = 5;
pub(super) const CLOCKS_FC0_RESULT_KHZ_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_FC0_RESULT_FRAC
// Description : None
pub(super) const CLOCKS_FC0_RESULT_FRAC_RESET: u32 = 0x00;
pub(super) const CLOCKS_FC0_RESULT_FRAC_BITS: u32 = 0x0000001f;
pub(super) const CLOCKS_FC0_RESULT_FRAC_MSB: i32 = 4;
pub(super) const CLOCKS_FC0_RESULT_FRAC_LSB: i32 = 0;
pub(super) const CLOCKS_FC0_RESULT_FRAC_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_WAKE_EN0
// Description : enable clock in wake mode
pub(super) const CLOCKS_WAKE_EN0_OFFSET: u32 = 0x000000a0;
pub(super) const CLOCKS_WAKE_EN0_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_WAKE_EN0_RESET: u32 = 0xffffffff;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SRAM3
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_BITS: u32 = 0x80000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_MSB: i32 = 31;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_LSB: i32 = 31;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM3_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SRAM2
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_BITS: u32 = 0x40000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_MSB: i32 = 30;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_LSB: i32 = 30;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM2_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SRAM1
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_BITS: u32 = 0x20000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_MSB: i32 = 29;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_LSB: i32 = 29;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SRAM0
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_BITS: u32 = 0x10000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_MSB: i32 = 28;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_LSB: i32 = 28;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SRAM0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SPI1
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_BITS: u32 = 0x08000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_MSB: i32 = 27;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_LSB: i32 = 27;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_PERI_SPI1
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_BITS: u32 = 0x04000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_MSB: i32 = 26;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_LSB: i32 = 26;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SPI0
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_BITS: u32 = 0x02000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_MSB: i32 = 25;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_LSB: i32 = 25;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_PERI_SPI0
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_BITS: u32 = 0x01000000;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_MSB: i32 = 24;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_LSB: i32 = 24;
pub(super) const CLOCKS_WAKE_EN0_CLK_PERI_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_SIO
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SIO_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SIO_BITS: u32 = 0x00800000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SIO_MSB: i32 = 23;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SIO_LSB: i32 = 23;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_SIO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_RTC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RTC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RTC_BITS: u32 = 0x00400000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RTC_MSB: i32 = 22;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RTC_LSB: i32 = 22;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_RTC_RTC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_RTC_RTC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_RTC_RTC_BITS: u32 = 0x00200000;
pub(super) const CLOCKS_WAKE_EN0_CLK_RTC_RTC_MSB: i32 = 21;
pub(super) const CLOCKS_WAKE_EN0_CLK_RTC_RTC_LSB: i32 = 21;
pub(super) const CLOCKS_WAKE_EN0_CLK_RTC_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_ROSC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_MSB: i32 = 20;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_LSB: i32 = 20;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROSC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_ROM
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROM_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROM_BITS: u32 = 0x00080000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROM_MSB: i32 = 19;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROM_LSB: i32 = 19;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ROM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_RESETS
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_BITS: u32 = 0x00040000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_MSB: i32 = 18;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_LSB: i32 = 18;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_RESETS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PWM
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PWM_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PWM_BITS: u32 = 0x00020000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PWM_MSB: i32 = 17;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PWM_LSB: i32 = 17;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PWM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PSM
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PSM_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PSM_BITS: u32 = 0x00010000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PSM_MSB: i32 = 16;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PSM_LSB: i32 = 16;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PSM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_BITS: u32 = 0x00008000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_MSB: i32 = 15;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_LSB: i32 = 15;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_USB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_MSB: i32 = 14;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_LSB: i32 = 14;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PLL_SYS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PIO1
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_MSB: i32 = 13;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_LSB: i32 = 13;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PIO0
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_MSB: i32 = 12;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_LSB: i32 = 12;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PIO0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_PADS
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PADS_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PADS_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PADS_MSB: i32 = 11;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PADS_LSB: i32 = 11;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_PADS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_MSB: i32 = 10;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_LSB: i32 = 10;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_VREG_AND_CHIP_RESET_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_JTAG
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_MSB: i32 = 9;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_LSB: i32 = 9;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_JTAG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_IO
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_IO_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_IO_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_IO_MSB: i32 = 8;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_IO_LSB: i32 = 8;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_IO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_I2C1
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_MSB: i32 = 7;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_LSB: i32 = 7;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_I2C0
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_MSB: i32 = 6;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_LSB: i32 = 6;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_I2C0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_DMA
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_DMA_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_DMA_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_DMA_MSB: i32 = 5;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_DMA_LSB: i32 = 5;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_DMA_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_MSB: i32 = 4;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_LSB: i32 = 4;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSFABRIC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_MSB: i32 = 3;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_LSB: i32 = 3;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_BUSCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_ADC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ADC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ADC_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ADC_MSB: i32 = 2;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ADC_LSB: i32 = 2;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_ADC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_ADC_ADC
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_ADC_ADC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_ADC_ADC_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_WAKE_EN0_CLK_ADC_ADC_MSB: i32 = 1;
pub(super) const CLOCKS_WAKE_EN0_CLK_ADC_ADC_LSB: i32 = 1;
pub(super) const CLOCKS_WAKE_EN0_CLK_ADC_ADC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS
// Description : None
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_MSB: i32 = 0;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_LSB: i32 = 0;
pub(super) const CLOCKS_WAKE_EN0_CLK_SYS_CLOCKS_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_WAKE_EN1
// Description : enable clock in wake mode
pub(super) const CLOCKS_WAKE_EN1_OFFSET: u32 = 0x000000a4;
pub(super) const CLOCKS_WAKE_EN1_BITS: u32 = 0x00007fff;
pub(super) const CLOCKS_WAKE_EN1_RESET: u32 = 0x00007fff;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_XOSC
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_MSB: i32 = 14;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_LSB: i32 = 14;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XOSC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_XIP
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XIP_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XIP_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XIP_MSB: i32 = 13;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XIP_LSB: i32 = 13;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_XIP_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_MSB: i32 = 12;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_LSB: i32 = 12;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_WATCHDOG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_USB_USBCTRL
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_MSB: i32 = 11;
pub(super) const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_LSB: i32 = 11;
pub(super) const CLOCKS_WAKE_EN1_CLK_USB_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_MSB: i32 = 10;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_LSB: i32 = 10;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_UART1
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART1_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART1_MSB: i32 = 9;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART1_LSB: i32 = 9;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_PERI_UART1
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART1_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART1_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART1_MSB: i32 = 8;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART1_LSB: i32 = 8;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_UART0
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART0_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART0_MSB: i32 = 7;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART0_LSB: i32 = 7;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_PERI_UART0
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART0_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART0_MSB: i32 = 6;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART0_LSB: i32 = 6;
pub(super) const CLOCKS_WAKE_EN1_CLK_PERI_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_TIMER
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_MSB: i32 = 5;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_LSB: i32 = 5;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TIMER_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_TBMAN
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_MSB: i32 = 4;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_LSB: i32 = 4;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_TBMAN_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_MSB: i32 = 3;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_LSB: i32 = 3;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSINFO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_MSB: i32 = 2;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_LSB: i32 = 2;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SYSCFG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_SRAM5
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_MSB: i32 = 1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_LSB: i32 = 1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM5_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_WAKE_EN1_CLK_SYS_SRAM4
// Description : None
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_RESET: u32 = 0x1;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_MSB: i32 = 0;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_LSB: i32 = 0;
pub(super) const CLOCKS_WAKE_EN1_CLK_SYS_SRAM4_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_SLEEP_EN0
// Description : enable clock in sleep mode
pub(super) const CLOCKS_SLEEP_EN0_OFFSET: u32 = 0x000000a8;
pub(super) const CLOCKS_SLEEP_EN0_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_SLEEP_EN0_RESET: u32 = 0xffffffff;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_BITS: u32 = 0x80000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_MSB: i32 = 31;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_LSB: i32 = 31;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM3_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_BITS: u32 = 0x40000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_MSB: i32 = 30;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_LSB: i32 = 30;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM2_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_BITS: u32 = 0x20000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_MSB: i32 = 29;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_LSB: i32 = 29;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_BITS: u32 = 0x10000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_MSB: i32 = 28;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_LSB: i32 = 28;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SRAM0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SPI1
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_BITS: u32 = 0x08000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_MSB: i32 = 27;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_LSB: i32 = 27;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_PERI_SPI1
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_BITS: u32 = 0x04000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_MSB: i32 = 26;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_LSB: i32 = 26;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SPI0
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_BITS: u32 = 0x02000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_MSB: i32 = 25;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_LSB: i32 = 25;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_PERI_SPI0
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_BITS: u32 = 0x01000000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_MSB: i32 = 24;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_LSB: i32 = 24;
pub(super) const CLOCKS_SLEEP_EN0_CLK_PERI_SPI0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_SIO
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_BITS: u32 = 0x00800000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_MSB: i32 = 23;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_LSB: i32 = 23;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_SIO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_RTC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_BITS: u32 = 0x00400000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_MSB: i32 = 22;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_LSB: i32 = 22;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_RTC_RTC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_BITS: u32 = 0x00200000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_MSB: i32 = 21;
pub(super) const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_LSB: i32 = 21;
pub(super) const CLOCKS_SLEEP_EN0_CLK_RTC_RTC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_ROSC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_MSB: i32 = 20;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_LSB: i32 = 20;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROSC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_ROM
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_BITS: u32 = 0x00080000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_MSB: i32 = 19;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_LSB: i32 = 19;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ROM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_RESETS
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_BITS: u32 = 0x00040000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_MSB: i32 = 18;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_LSB: i32 = 18;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_RESETS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PWM
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_BITS: u32 = 0x00020000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_MSB: i32 = 17;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_LSB: i32 = 17;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PWM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PSM
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_BITS: u32 = 0x00010000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_MSB: i32 = 16;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_LSB: i32 = 16;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PSM_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_BITS: u32 = 0x00008000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_MSB: i32 = 15;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_LSB: i32 = 15;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_USB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_MSB: i32 = 14;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_LSB: i32 = 14;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PLL_SYS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PIO1
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_MSB: i32 = 13;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_LSB: i32 = 13;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PIO0
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_MSB: i32 = 12;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_LSB: i32 = 12;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PIO0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_PADS
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_MSB: i32 = 11;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_LSB: i32 = 11;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_PADS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_MSB: i32 = 10;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_LSB: i32 = 10;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_VREG_AND_CHIP_RESET_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_JTAG
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_MSB: i32 = 9;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_LSB: i32 = 9;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_JTAG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_IO
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_IO_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_IO_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_IO_MSB: i32 = 8;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_IO_LSB: i32 = 8;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_IO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_I2C1
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_MSB: i32 = 7;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_LSB: i32 = 7;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_I2C0
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_MSB: i32 = 6;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_LSB: i32 = 6;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_I2C0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_DMA
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_MSB: i32 = 5;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_LSB: i32 = 5;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_DMA_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_MSB: i32 = 4;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_LSB: i32 = 4;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSFABRIC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_MSB: i32 = 3;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_LSB: i32 = 3;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_BUSCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_ADC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_MSB: i32 = 2;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_LSB: i32 = 2;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_ADC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_ADC_ADC
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_MSB: i32 = 1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_LSB: i32 = 1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_ADC_ADC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS
// Description : None
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_MSB: i32 = 0;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_LSB: i32 = 0;
pub(super) const CLOCKS_SLEEP_EN0_CLK_SYS_CLOCKS_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_SLEEP_EN1
// Description : enable clock in sleep mode
pub(super) const CLOCKS_SLEEP_EN1_OFFSET: u32 = 0x000000ac;
pub(super) const CLOCKS_SLEEP_EN1_BITS: u32 = 0x00007fff;
pub(super) const CLOCKS_SLEEP_EN1_RESET: u32 = 0x00007fff;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_XOSC
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_MSB: i32 = 14;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_LSB: i32 = 14;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XOSC_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_XIP
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_MSB: i32 = 13;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_LSB: i32 = 13;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_XIP_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_MSB: i32 = 12;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_LSB: i32 = 12;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_WATCHDOG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_MSB: i32 = 11;
pub(super) const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_LSB: i32 = 11;
pub(super) const CLOCKS_SLEEP_EN1_CLK_USB_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_MSB: i32 = 10;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_LSB: i32 = 10;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_USBCTRL_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_UART1
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_MSB: i32 = 9;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_LSB: i32 = 9;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_PERI_UART1
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_MSB: i32 = 8;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_LSB: i32 = 8;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_UART0
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_MSB: i32 = 7;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_LSB: i32 = 7;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_PERI_UART0
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_MSB: i32 = 6;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_LSB: i32 = 6;
pub(super) const CLOCKS_SLEEP_EN1_CLK_PERI_UART0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_TIMER
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_MSB: i32 = 5;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_LSB: i32 = 5;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TIMER_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_MSB: i32 = 4;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_LSB: i32 = 4;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_TBMAN_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_MSB: i32 = 3;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_LSB: i32 = 3;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSINFO_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_MSB: i32 = 2;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_LSB: i32 = 2;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SYSCFG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_MSB: i32 = 1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_LSB: i32 = 1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM5_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4
// Description : None
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_RESET: u32 = 0x1;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_MSB: i32 = 0;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_LSB: i32 = 0;
pub(super) const CLOCKS_SLEEP_EN1_CLK_SYS_SRAM4_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_ENABLED0
// Description : indicates the state of the clock enable
pub(super) const CLOCKS_ENABLED0_OFFSET: u32 = 0x000000b0;
pub(super) const CLOCKS_ENABLED0_BITS: u32 = 0xffffffff;
pub(super) const CLOCKS_ENABLED0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SRAM3
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM3_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM3_BITS: u32 = 0x80000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM3_MSB: i32 = 31;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM3_LSB: i32 = 31;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM3_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SRAM2
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM2_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM2_BITS: u32 = 0x40000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM2_MSB: i32 = 30;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM2_LSB: i32 = 30;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM2_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SRAM1
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM1_BITS: u32 = 0x20000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM1_MSB: i32 = 29;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM1_LSB: i32 = 29;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SRAM0
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM0_BITS: u32 = 0x10000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM0_MSB: i32 = 28;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM0_LSB: i32 = 28;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SRAM0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SPI1
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI1_BITS: u32 = 0x08000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI1_MSB: i32 = 27;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI1_LSB: i32 = 27;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_PERI_SPI1
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI1_BITS: u32 = 0x04000000;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI1_MSB: i32 = 26;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI1_LSB: i32 = 26;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SPI0
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI0_BITS: u32 = 0x02000000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI0_MSB: i32 = 25;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI0_LSB: i32 = 25;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SPI0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_PERI_SPI0
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI0_BITS: u32 = 0x01000000;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI0_MSB: i32 = 24;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI0_LSB: i32 = 24;
pub(super) const CLOCKS_ENABLED0_CLK_PERI_SPI0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_SIO
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SIO_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SIO_BITS: u32 = 0x00800000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SIO_MSB: i32 = 23;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SIO_LSB: i32 = 23;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_SIO_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_RTC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RTC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RTC_BITS: u32 = 0x00400000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RTC_MSB: i32 = 22;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RTC_LSB: i32 = 22;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RTC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_RTC_RTC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_RTC_RTC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_RTC_RTC_BITS: u32 = 0x00200000;
pub(super) const CLOCKS_ENABLED0_CLK_RTC_RTC_MSB: i32 = 21;
pub(super) const CLOCKS_ENABLED0_CLK_RTC_RTC_LSB: i32 = 21;
pub(super) const CLOCKS_ENABLED0_CLK_RTC_RTC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_ROSC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROSC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROSC_BITS: u32 = 0x00100000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROSC_MSB: i32 = 20;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROSC_LSB: i32 = 20;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROSC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_ROM
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROM_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROM_BITS: u32 = 0x00080000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROM_MSB: i32 = 19;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROM_LSB: i32 = 19;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ROM_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_RESETS
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RESETS_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RESETS_BITS: u32 = 0x00040000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RESETS_MSB: i32 = 18;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RESETS_LSB: i32 = 18;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_RESETS_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PWM
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PWM_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PWM_BITS: u32 = 0x00020000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PWM_MSB: i32 = 17;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PWM_LSB: i32 = 17;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PWM_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PSM
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PSM_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PSM_BITS: u32 = 0x00010000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PSM_MSB: i32 = 16;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PSM_LSB: i32 = 16;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PSM_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PLL_USB
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_BITS: u32 = 0x00008000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_MSB: i32 = 15;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_LSB: i32 = 15;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_USB_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PLL_SYS
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_MSB: i32 = 14;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_LSB: i32 = 14;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PLL_SYS_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PIO1
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO1_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO1_MSB: i32 = 13;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO1_LSB: i32 = 13;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PIO0
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO0_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO0_MSB: i32 = 12;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO0_LSB: i32 = 12;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PIO0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_PADS
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PADS_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PADS_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PADS_MSB: i32 = 11;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PADS_LSB: i32 = 11;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_PADS_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_MSB: i32 = 10;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_LSB: i32 = 10;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_VREG_AND_CHIP_RESET_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_JTAG
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_JTAG_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_JTAG_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_JTAG_MSB: i32 = 9;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_JTAG_LSB: i32 = 9;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_JTAG_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_IO
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_IO_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_IO_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_IO_MSB: i32 = 8;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_IO_LSB: i32 = 8;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_IO_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_I2C1
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C1_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C1_MSB: i32 = 7;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C1_LSB: i32 = 7;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_I2C0
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C0_MSB: i32 = 6;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C0_LSB: i32 = 6;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_I2C0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_DMA
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_DMA_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_DMA_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_DMA_MSB: i32 = 5;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_DMA_LSB: i32 = 5;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_DMA_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_MSB: i32 = 4;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_LSB: i32 = 4;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSFABRIC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_BUSCTRL
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_MSB: i32 = 3;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_LSB: i32 = 3;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_BUSCTRL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_ADC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ADC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ADC_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ADC_MSB: i32 = 2;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ADC_LSB: i32 = 2;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_ADC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_ADC_ADC
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_ADC_ADC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_ADC_ADC_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_ENABLED0_CLK_ADC_ADC_MSB: i32 = 1;
pub(super) const CLOCKS_ENABLED0_CLK_ADC_ADC_LSB: i32 = 1;
pub(super) const CLOCKS_ENABLED0_CLK_ADC_ADC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED0_CLK_SYS_CLOCKS
// Description : None
pub(super) const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_MSB: i32 = 0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_LSB: i32 = 0;
pub(super) const CLOCKS_ENABLED0_CLK_SYS_CLOCKS_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_ENABLED1
// Description : indicates the state of the clock enable
pub(super) const CLOCKS_ENABLED1_OFFSET: u32 = 0x000000b4;
pub(super) const CLOCKS_ENABLED1_BITS: u32 = 0x00007fff;
pub(super) const CLOCKS_ENABLED1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_XOSC
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XOSC_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XOSC_BITS: u32 = 0x00004000;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XOSC_MSB: i32 = 14;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XOSC_LSB: i32 = 14;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XOSC_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_XIP
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XIP_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XIP_BITS: u32 = 0x00002000;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XIP_MSB: i32 = 13;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XIP_LSB: i32 = 13;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_XIP_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_WATCHDOG
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_BITS: u32 = 0x00001000;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_MSB: i32 = 12;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_LSB: i32 = 12;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_WATCHDOG_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_USB_USBCTRL
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_USB_USBCTRL_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_USB_USBCTRL_BITS: u32 = 0x00000800;
pub(super) const CLOCKS_ENABLED1_CLK_USB_USBCTRL_MSB: i32 = 11;
pub(super) const CLOCKS_ENABLED1_CLK_USB_USBCTRL_LSB: i32 = 11;
pub(super) const CLOCKS_ENABLED1_CLK_USB_USBCTRL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_USBCTRL
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_BITS: u32 = 0x00000400;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_MSB: i32 = 10;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_LSB: i32 = 10;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_USBCTRL_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_UART1
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART1_BITS: u32 = 0x00000200;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART1_MSB: i32 = 9;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART1_LSB: i32 = 9;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_PERI_UART1
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART1_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART1_BITS: u32 = 0x00000100;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART1_MSB: i32 = 8;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART1_LSB: i32 = 8;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_UART0
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART0_BITS: u32 = 0x00000080;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART0_MSB: i32 = 7;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART0_LSB: i32 = 7;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_UART0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_PERI_UART0
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART0_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART0_BITS: u32 = 0x00000040;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART0_MSB: i32 = 6;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART0_LSB: i32 = 6;
pub(super) const CLOCKS_ENABLED1_CLK_PERI_UART0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_TIMER
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TIMER_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TIMER_BITS: u32 = 0x00000020;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TIMER_MSB: i32 = 5;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TIMER_LSB: i32 = 5;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TIMER_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_TBMAN
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TBMAN_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TBMAN_BITS: u32 = 0x00000010;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TBMAN_MSB: i32 = 4;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TBMAN_LSB: i32 = 4;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_TBMAN_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_SYSINFO
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_BITS: u32 = 0x00000008;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_MSB: i32 = 3;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_LSB: i32 = 3;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSINFO_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_SYSCFG
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_BITS: u32 = 0x00000004;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_MSB: i32 = 2;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_LSB: i32 = 2;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SYSCFG_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_SRAM5
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM5_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM5_BITS: u32 = 0x00000002;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM5_MSB: i32 = 1;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM5_LSB: i32 = 1;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM5_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : CLOCKS_ENABLED1_CLK_SYS_SRAM4
// Description : None
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM4_RESET: u32 = 0x0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM4_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM4_MSB: i32 = 0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM4_LSB: i32 = 0;
pub(super) const CLOCKS_ENABLED1_CLK_SYS_SRAM4_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_INTR
// Description : Raw Interrupts
pub(super) const CLOCKS_INTR_OFFSET: u32 = 0x000000b8;
pub(super) const CLOCKS_INTR_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTR_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_INTR_CLK_SYS_RESUS
// Description : None
pub(super) const CLOCKS_INTR_CLK_SYS_RESUS_RESET: u32 = 0x0;
pub(super) const CLOCKS_INTR_CLK_SYS_RESUS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTR_CLK_SYS_RESUS_MSB: i32 = 0;
pub(super) const CLOCKS_INTR_CLK_SYS_RESUS_LSB: i32 = 0;
pub(super) const CLOCKS_INTR_CLK_SYS_RESUS_ACCESS: &str = "RO";
// =============================================================================
// Register    : CLOCKS_INTE
// Description : Interrupt Enable
pub(super) const CLOCKS_INTE_OFFSET: u32 = 0x000000bc;
pub(super) const CLOCKS_INTE_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTE_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_INTE_CLK_SYS_RESUS
// Description : None
pub(super) const CLOCKS_INTE_CLK_SYS_RESUS_RESET: u32 = 0x0;
pub(super) const CLOCKS_INTE_CLK_SYS_RESUS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTE_CLK_SYS_RESUS_MSB: i32 = 0;
pub(super) const CLOCKS_INTE_CLK_SYS_RESUS_LSB: i32 = 0;
pub(super) const CLOCKS_INTE_CLK_SYS_RESUS_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_INTF
// Description : Interrupt Force
pub(super) const CLOCKS_INTF_OFFSET: u32 = 0x000000c0;
pub(super) const CLOCKS_INTF_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTF_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_INTF_CLK_SYS_RESUS
// Description : None
pub(super) const CLOCKS_INTF_CLK_SYS_RESUS_RESET: u32 = 0x0;
pub(super) const CLOCKS_INTF_CLK_SYS_RESUS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTF_CLK_SYS_RESUS_MSB: i32 = 0;
pub(super) const CLOCKS_INTF_CLK_SYS_RESUS_LSB: i32 = 0;
pub(super) const CLOCKS_INTF_CLK_SYS_RESUS_ACCESS: &str = "RW";
// =============================================================================
// Register    : CLOCKS_INTS
// Description : Interrupt status after masking & forcing
pub(super) const CLOCKS_INTS_OFFSET: u32 = 0x000000c4;
pub(super) const CLOCKS_INTS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : CLOCKS_INTS_CLK_SYS_RESUS
// Description : None
pub(super) const CLOCKS_INTS_CLK_SYS_RESUS_RESET: u32 = 0x0;
pub(super) const CLOCKS_INTS_CLK_SYS_RESUS_BITS: u32 = 0x00000001;
pub(super) const CLOCKS_INTS_CLK_SYS_RESUS_MSB: i32 = 0;
pub(super) const CLOCKS_INTS_CLK_SYS_RESUS_LSB: i32 = 0;
pub(super) const CLOCKS_INTS_CLK_SYS_RESUS_ACCESS: &str = "RO";
// =============================================================================
