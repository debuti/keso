// =============================================================================
// Register block : PLL
// Version        : 1
// Bus type       : apb
// Description    : None
// =============================================================================
// =============================================================================
// Register    : PLL_CS
// Description : Control and Status
//               GENERAL CONSTRAINTS:
//               Reference clock frequency min=5MHz, max=800MHz
//               Feedback divider min=16, max=320
//               VCO frequency min=400MHz, max=1600MHz
pub(super) const PLL_CS_OFFSET: u32 = 0x00000000;
pub(super) const PLL_CS_BITS: u32 = 0x8000013f;
pub(super) const PLL_CS_RESET: u32 = 0x00000001;
// -----------------------------------------------------------------------------
// Field       : PLL_CS_LOCK
// Description : PLL is locked
pub(super) const PLL_CS_LOCK_RESET: u32 = 0x0;
pub(super) const PLL_CS_LOCK_BITS: u32 = 0x80000000;
pub(super) const PLL_CS_LOCK_MSB: i32 = 31;
pub(super) const PLL_CS_LOCK_LSB: i32 = 31;
pub(super) const PLL_CS_LOCK_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : PLL_CS_BYPASS
// Description : Passes the reference clock to the output instead of the divided
//               VCO. The VCO continues to run so the user can switch between
//               the reference clock and the divided VCO but the output will
//               glitch when doing so.
pub(super) const PLL_CS_BYPASS_RESET: u32 = 0x0;
pub(super) const PLL_CS_BYPASS_BITS: u32 = 0x00000100;
pub(super) const PLL_CS_BYPASS_MSB: i32 = 8;
pub(super) const PLL_CS_BYPASS_LSB: i32 = 8;
pub(super) const PLL_CS_BYPASS_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : PLL_CS_REFDIV
// Description : Divides the PLL input reference clock.
//               Behaviour is undefined for div=0.
//               PLL output will be unpredictable during refdiv changes, wait
//               for lock=1 before using it.
pub(super) const PLL_CS_REFDIV_RESET: u32 = 0x01;
pub(super) const PLL_CS_REFDIV_BITS: u32 = 0x0000003f;
pub(super) const PLL_CS_REFDIV_MSB: i32 = 5;
pub(super) const PLL_CS_REFDIV_LSB: i32 = 0;
pub(super) const PLL_CS_REFDIV_ACCESS: &str = "RW";
// =============================================================================
// Register    : PLL_PWR
// Description : Controls the PLL power modes.
pub(super) const PLL_PWR_OFFSET: u32 = 0x00000004;
pub(super) const PLL_PWR_BITS: u32 = 0x0000002d;
pub(super) const PLL_PWR_RESET: u32 = 0x0000002d;
// -----------------------------------------------------------------------------
// Field       : PLL_PWR_VCOPD
// Description : PLL VCO powerdown
//               To save power set high when PLL output not required or
//               bypass=1.
pub(super) const PLL_PWR_VCOPD_RESET: u32 = 0x1;
pub(super) const PLL_PWR_VCOPD_BITS: u32 = 0x00000020;
pub(super) const PLL_PWR_VCOPD_MSB: i32 = 5;
pub(super) const PLL_PWR_VCOPD_LSB: i32 = 5;
pub(super) const PLL_PWR_VCOPD_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : PLL_PWR_POSTDIVPD
// Description : PLL post divider powerdown
//               To save power set high when PLL output not required or
//               bypass=1.
pub(super) const PLL_PWR_POSTDIVPD_RESET: u32 = 0x1;
pub(super) const PLL_PWR_POSTDIVPD_BITS: u32 = 0x00000008;
pub(super) const PLL_PWR_POSTDIVPD_MSB: i32 = 3;
pub(super) const PLL_PWR_POSTDIVPD_LSB: i32 = 3;
pub(super) const PLL_PWR_POSTDIVPD_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : PLL_PWR_DSMPD
// Description : PLL DSM powerdown
//               Nothing is achieved by setting this low.
pub(super) const PLL_PWR_DSMPD_RESET: u32 = 0x1;
pub(super) const PLL_PWR_DSMPD_BITS: u32 = 0x00000004;
pub(super) const PLL_PWR_DSMPD_MSB: i32 = 2;
pub(super) const PLL_PWR_DSMPD_LSB: i32 = 2;
pub(super) const PLL_PWR_DSMPD_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : PLL_PWR_PD
// Description : PLL powerdown
//               To save power set high when PLL output not required.
pub(super) const PLL_PWR_PD_RESET: u32 = 0x1;
pub(super) const PLL_PWR_PD_BITS: u32 = 0x00000001;
pub(super) const PLL_PWR_PD_MSB: i32 = 0;
pub(super) const PLL_PWR_PD_LSB: i32 = 0;
pub(super) const PLL_PWR_PD_ACCESS: &str = "RW";
// =============================================================================
// Register    : PLL_FBDIV_INT
// Description : Feedback divisor
//               (note: this PLL does not support fractional division)
//               see ctrl reg description for constraints
pub(super) const PLL_FBDIV_INT_OFFSET: u32 = 0x00000008;
pub(super) const PLL_FBDIV_INT_BITS: u32 = 0x00000fff;
pub(super) const PLL_FBDIV_INT_RESET: u32 = 0x00000000;
pub(super) const PLL_FBDIV_INT_MSB: i32 = 11;
pub(super) const PLL_FBDIV_INT_LSB: i32 = 0;
pub(super) const PLL_FBDIV_INT_ACCESS: &str = "RW";
// =============================================================================
// Register    : PLL_PRIM
// Description : Controls the PLL post dividers for the primary output
//               (note: this PLL does not have a secondary output)
//               the primary output is driven from VCO divided by
//               postdiv1*postdiv2
pub(super) const PLL_PRIM_OFFSET: u32 = 0x0000000c;
pub(super) const PLL_PRIM_BITS: u32 = 0x00077000;
pub(super) const PLL_PRIM_RESET: u32 = 0x00077000;
// -----------------------------------------------------------------------------
// Field       : PLL_PRIM_POSTDIV1
// Description : divide by 1-7
pub(super) const PLL_PRIM_POSTDIV1_RESET: u32 = 0x7;
pub(super) const PLL_PRIM_POSTDIV1_BITS: u32 = 0x00070000;
pub(super) const PLL_PRIM_POSTDIV1_MSB: i32 = 18;
pub(super) const PLL_PRIM_POSTDIV1_LSB: i32 = 16;
pub(super) const PLL_PRIM_POSTDIV1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : PLL_PRIM_POSTDIV2
// Description : divide by 1-7
pub(super) const PLL_PRIM_POSTDIV2_RESET: u32 = 0x7;
pub(super) const PLL_PRIM_POSTDIV2_BITS: u32 = 0x00007000;
pub(super) const PLL_PRIM_POSTDIV2_MSB: i32 = 14;
pub(super) const PLL_PRIM_POSTDIV2_LSB: i32 = 12;
pub(super) const PLL_PRIM_POSTDIV2_ACCESS: &str = "RW";
// =============================================================================
