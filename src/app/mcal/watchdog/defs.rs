// =============================================================================
// Register block : WATCHDOG
// Version        : 1
// Bus type       : apb
// Description    : None
// =============================================================================
// =============================================================================
// Register    : WATCHDOG_CTRL
// Description : Watchdog control
//               The rst_wdsel register determines which subsystems are reset
//               when the watchdog is triggered.
//               The watchdog can be triggered in software.
pub(super) const WATCHDOG_CTRL_OFFSET: u32 = 0x00000000;
pub(super) const WATCHDOG_CTRL_BITS: u32 = 0xc7ffffff;
pub(super) const WATCHDOG_CTRL_RESET: u32 = 0x07000000;
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_TRIGGER
// Description : Trigger a watchdog reset
pub(super) const WATCHDOG_CTRL_TRIGGER_RESET: u32 = 0x0;
pub(super) const WATCHDOG_CTRL_TRIGGER_BITS: u32 = 0x80000000;
pub(super) const WATCHDOG_CTRL_TRIGGER_MSB: i32 = 31;
pub(super) const WATCHDOG_CTRL_TRIGGER_LSB: i32 = 31;
pub(super) const WATCHDOG_CTRL_TRIGGER_ACCESS: &str = "SC";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_ENABLE
// Description : When not enabled the watchdog timer is paused
pub(super) const WATCHDOG_CTRL_ENABLE_RESET: u32 = 0x0;
pub(super) const WATCHDOG_CTRL_ENABLE_BITS: u32 = 0x40000000;
pub(super) const WATCHDOG_CTRL_ENABLE_MSB: i32 = 30;
pub(super) const WATCHDOG_CTRL_ENABLE_LSB: i32 = 30;
pub(super) const WATCHDOG_CTRL_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_PAUSE_DBG1
// Description : Pause the watchdog timer when processor 1 is in debug mode
pub(super) const WATCHDOG_CTRL_PAUSE_DBG1_RESET: u32 = 0x1;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG1_BITS: u32 = 0x04000000;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG1_MSB: i32 = 26;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG1_LSB: i32 = 26;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_PAUSE_DBG0
// Description : Pause the watchdog timer when processor 0 is in debug mode
pub(super) const WATCHDOG_CTRL_PAUSE_DBG0_RESET: u32 = 0x1;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG0_BITS: u32 = 0x02000000;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG0_MSB: i32 = 25;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG0_LSB: i32 = 25;
pub(super) const WATCHDOG_CTRL_PAUSE_DBG0_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_PAUSE_JTAG
// Description : Pause the watchdog timer when JTAG is accessing the bus fabric
pub(super) const WATCHDOG_CTRL_PAUSE_JTAG_RESET: u32 = 0x1;
pub(super) const WATCHDOG_CTRL_PAUSE_JTAG_BITS: u32 = 0x01000000;
pub(super) const WATCHDOG_CTRL_PAUSE_JTAG_MSB: i32 = 24;
pub(super) const WATCHDOG_CTRL_PAUSE_JTAG_LSB: i32 = 24;
pub(super) const WATCHDOG_CTRL_PAUSE_JTAG_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_CTRL_TIME
// Description : Indicates the number of ticks / 2 (see errata RP2040-E1) before
//               a watchdog reset will be triggered
pub(super) const WATCHDOG_CTRL_TIME_RESET: u32 = 0x000000;
pub(super) const WATCHDOG_CTRL_TIME_BITS: u32 = 0x00ffffff;
pub(super) const WATCHDOG_CTRL_TIME_MSB: i32 = 23;
pub(super) const WATCHDOG_CTRL_TIME_LSB: i32 = 0;
pub(super) const WATCHDOG_CTRL_TIME_ACCESS: &str = "RO";
// =============================================================================
// Register    : WATCHDOG_LOAD
// Description : Load the watchdog timer. The maximum setting is 0xffffff which
//               corresponds to 0xffffff / 2 ticks before triggering a watchdog
//               reset (see errata RP2040-E1).
pub(super) const WATCHDOG_LOAD_OFFSET: u32 = 0x00000004;
pub(super) const WATCHDOG_LOAD_BITS: u32 = 0x00ffffff;
pub(super) const WATCHDOG_LOAD_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_LOAD_MSB: i32 = 23;
pub(super) const WATCHDOG_LOAD_LSB: i32 = 0;
pub(super) const WATCHDOG_LOAD_ACCESS: &str = "WF";
// =============================================================================
// Register    : WATCHDOG_REASON
// Description : Logs the reason for the last reset. Both bits are zero for the
//               case of a hardware reset.
pub(super) const WATCHDOG_REASON_OFFSET: u32 = 0x00000008;
pub(super) const WATCHDOG_REASON_BITS: u32 = 0x00000003;
pub(super) const WATCHDOG_REASON_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_REASON_FORCE
// Description : None
pub(super) const WATCHDOG_REASON_FORCE_RESET: u32 = 0x0;
pub(super) const WATCHDOG_REASON_FORCE_BITS: u32 = 0x00000002;
pub(super) const WATCHDOG_REASON_FORCE_MSB: i32 = 1;
pub(super) const WATCHDOG_REASON_FORCE_LSB: i32 = 1;
pub(super) const WATCHDOG_REASON_FORCE_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_REASON_TIMER
// Description : None
pub(super) const WATCHDOG_REASON_TIMER_RESET: u32 = 0x0;
pub(super) const WATCHDOG_REASON_TIMER_BITS: u32 = 0x00000001;
pub(super) const WATCHDOG_REASON_TIMER_MSB: i32 = 0;
pub(super) const WATCHDOG_REASON_TIMER_LSB: i32 = 0;
pub(super) const WATCHDOG_REASON_TIMER_ACCESS: &str = "RO";
// =============================================================================
// Register    : WATCHDOG_SCRATCH0
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH0_OFFSET: u32 = 0x0000000c;
pub(super) const WATCHDOG_SCRATCH0_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH0_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH0_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH0_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH0_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH1
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH1_OFFSET: u32 = 0x00000010;
pub(super) const WATCHDOG_SCRATCH1_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH1_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH1_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH1_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH1_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH2
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH2_OFFSET: u32 = 0x00000014;
pub(super) const WATCHDOG_SCRATCH2_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH2_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH2_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH2_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH2_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH3
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH3_OFFSET: u32 = 0x00000018;
pub(super) const WATCHDOG_SCRATCH3_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH3_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH3_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH3_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH3_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH4
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH4_OFFSET: u32 = 0x0000001c;
pub(super) const WATCHDOG_SCRATCH4_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH4_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH4_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH4_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH4_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH5
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH5_OFFSET: u32 = 0x00000020;
pub(super) const WATCHDOG_SCRATCH5_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH5_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH5_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH5_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH5_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH6
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH6_OFFSET: u32 = 0x00000024;
pub(super) const WATCHDOG_SCRATCH6_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH6_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH6_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH6_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH6_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_SCRATCH7
// Description : Scratch register. Information persists through soft reset of
//               the chip.
pub(super) const WATCHDOG_SCRATCH7_OFFSET: u32 = 0x00000028;
pub(super) const WATCHDOG_SCRATCH7_BITS: u32 = 0xffffffff;
pub(super) const WATCHDOG_SCRATCH7_RESET: u32 = 0x00000000;
pub(super) const WATCHDOG_SCRATCH7_MSB: i32 = 31;
pub(super) const WATCHDOG_SCRATCH7_LSB: i32 = 0;
pub(super) const WATCHDOG_SCRATCH7_ACCESS: &str = "RW";
// =============================================================================
// Register    : WATCHDOG_TICK
// Description : Controls the tick generator
pub(super) const WATCHDOG_TICK_OFFSET: u32 = 0x0000002c;
pub(super) const WATCHDOG_TICK_BITS: u32 = 0x000fffff;
pub(super) const WATCHDOG_TICK_RESET: u32 = 0x00000200;
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_TICK_COUNT
// Description : Count down timer: the remaining number clk_tick cycles before
//               the next tick is generated.
pub(super) const WATCHDOG_TICK_COUNT_RESET: &str = "-";
pub(super) const WATCHDOG_TICK_COUNT_BITS: u32 = 0x000ff800;
pub(super) const WATCHDOG_TICK_COUNT_MSB: i32 = 19;
pub(super) const WATCHDOG_TICK_COUNT_LSB: i32 = 11;
pub(super) const WATCHDOG_TICK_COUNT_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_TICK_RUNNING
// Description : Is the tick generator running?
pub(super) const WATCHDOG_TICK_RUNNING_RESET: &str = "-";
pub(super) const WATCHDOG_TICK_RUNNING_BITS: u32 = 0x00000400;
pub(super) const WATCHDOG_TICK_RUNNING_MSB: i32 = 10;
pub(super) const WATCHDOG_TICK_RUNNING_LSB: i32 = 10;
pub(super) const WATCHDOG_TICK_RUNNING_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_TICK_ENABLE
// Description : start / stop tick generation
pub(super) const WATCHDOG_TICK_ENABLE_RESET: u32 = 0x1;
pub(super) const WATCHDOG_TICK_ENABLE_BITS: u32 = 0x00000200;
pub(super) const WATCHDOG_TICK_ENABLE_MSB: i32 = 9;
pub(super) const WATCHDOG_TICK_ENABLE_LSB: i32 = 9;
pub(super) const WATCHDOG_TICK_ENABLE_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : WATCHDOG_TICK_CYCLES
// Description : Total number of clk_tick cycles before the next tick.
pub(super) const WATCHDOG_TICK_CYCLES_RESET: u32 = 0x000;
pub(super) const WATCHDOG_TICK_CYCLES_BITS: u32 = 0x000001ff;
pub(super) const WATCHDOG_TICK_CYCLES_MSB: i32 = 8;
pub(super) const WATCHDOG_TICK_CYCLES_LSB: i32 = 0;
pub(super) const WATCHDOG_TICK_CYCLES_ACCESS: &str = "RW";
// =============================================================================
