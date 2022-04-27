// =============================================================================
// Register block : TIMER
// Version        : 1
// Bus type       : apb
// Description    : Controls time and alarms
//                  time is a 64 bit value indicating the time in usec since
//                  power-on
//                  timeh is the top 32 bits of time & timel is the bottom 32
//                  bits
//                  to change time write to timelw before timehw
//                  to read time read from timelr before timehr
//                  An alarm is set by setting alarm_enable and writing to the
//                  corresponding alarm register
//                  When an alarm is pending, the corresponding alarm_running
//                  signal will be high
//                  An alarm can be cancelled before it has finished by clearing
//                  the alarm_enable
//                  When an alarm fires, the corresponding alarm_irq is set and
//                  alarm_running is cleared
//                  To clear the interrupt write a 1 to the corresponding
//                  alarm_irq
// =============================================================================
// =============================================================================
// Register    : TIMER_TIMEHW
// Description : Write to bits 63:32 of time
//               always write timelw before timehw
pub(super) const TIMER_TIMEHW_OFFSET: u32 = 0x00000000;
pub(super) const TIMER_TIMEHW_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMEHW_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMEHW_MSB: i32 = 31;
pub(super) const TIMER_TIMEHW_LSB: i32 = 0;
pub(super) const TIMER_TIMEHW_ACCESS: &str = "WF";
// =============================================================================
// Register    : TIMER_TIMELW
// Description : Write to bits 31:0 of time
//               writes do not get copied to time until timehw is written
pub(super) const TIMER_TIMELW_OFFSET: u32 = 0x00000004;
pub(super) const TIMER_TIMELW_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMELW_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMELW_MSB: i32 = 31;
pub(super) const TIMER_TIMELW_LSB: i32 = 0;
pub(super) const TIMER_TIMELW_ACCESS: &str = "WF";
// =============================================================================
// Register    : TIMER_TIMEHR
// Description : Read from bits 63:32 of time
//               always read timelr before timehr
pub(super) const TIMER_TIMEHR_OFFSET: u32 = 0x00000008;
pub(super) const TIMER_TIMEHR_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMEHR_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMEHR_MSB: i32 = 31;
pub(super) const TIMER_TIMEHR_LSB: i32 = 0;
pub(super) const TIMER_TIMEHR_ACCESS: &str = "RO";
// =============================================================================
// Register    : TIMER_TIMELR
// Description : Read from bits 31:0 of time
pub(super) const TIMER_TIMELR_OFFSET: u32 = 0x0000000c;
pub(super) const TIMER_TIMELR_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMELR_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMELR_MSB: i32 = 31;
pub(super) const TIMER_TIMELR_LSB: i32 = 0;
pub(super) const TIMER_TIMELR_ACCESS: &str = "RO";
// =============================================================================
// Register    : TIMER_ALARM0
// Description : Arm alarm 0, and configure the time it will fire.
//               Once armed, the alarm fires when TIMER_ALARM0 == TIMELR.
//               The alarm will disarm itself once it fires, and can
//               be disarmed early using the ARMED status register.
pub(super) const TIMER_ALARM0_OFFSET: u32 = 0x00000010;
pub(super) const TIMER_ALARM0_BITS: u32 = 0xffffffff;
pub(super) const TIMER_ALARM0_RESET: u32 = 0x00000000;
pub(super) const TIMER_ALARM0_MSB: i32 = 31;
pub(super) const TIMER_ALARM0_LSB: i32 = 0;
pub(super) const TIMER_ALARM0_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_ALARM1
// Description : Arm alarm 1, and configure the time it will fire.
//               Once armed, the alarm fires when TIMER_ALARM1 == TIMELR.
//               The alarm will disarm itself once it fires, and can
//               be disarmed early using the ARMED status register.
pub(super) const TIMER_ALARM1_OFFSET: u32 = 0x00000014;
pub(super) const TIMER_ALARM1_BITS: u32 = 0xffffffff;
pub(super) const TIMER_ALARM1_RESET: u32 = 0x00000000;
pub(super) const TIMER_ALARM1_MSB: i32 = 31;
pub(super) const TIMER_ALARM1_LSB: i32 = 0;
pub(super) const TIMER_ALARM1_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_ALARM2
// Description : Arm alarm 2, and configure the time it will fire.
//               Once armed, the alarm fires when TIMER_ALARM2 == TIMELR.
//               The alarm will disarm itself once it fires, and can
//               be disarmed early using the ARMED status register.
pub(super) const TIMER_ALARM2_OFFSET: u32 = 0x00000018;
pub(super) const TIMER_ALARM2_BITS: u32 = 0xffffffff;
pub(super) const TIMER_ALARM2_RESET: u32 = 0x00000000;
pub(super) const TIMER_ALARM2_MSB: i32 = 31;
pub(super) const TIMER_ALARM2_LSB: i32 = 0;
pub(super) const TIMER_ALARM2_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_ALARM3
// Description : Arm alarm 3, and configure the time it will fire.
//               Once armed, the alarm fires when TIMER_ALARM3 == TIMELR.
//               The alarm will disarm itself once it fires, and can
//               be disarmed early using the ARMED status register.
pub(super) const TIMER_ALARM3_OFFSET: u32 = 0x0000001c;
pub(super) const TIMER_ALARM3_BITS: u32 = 0xffffffff;
pub(super) const TIMER_ALARM3_RESET: u32 = 0x00000000;
pub(super) const TIMER_ALARM3_MSB: i32 = 31;
pub(super) const TIMER_ALARM3_LSB: i32 = 0;
pub(super) const TIMER_ALARM3_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_ARMED
// Description : Indicates the armed/disarmed status of each alarm.
//               A write to the corresponding ALARMx register arms the alarm.
//               Alarms automatically disarm upon firing, but writing ones here
//               will disarm immediately without waiting to fire.
pub(super) const TIMER_ARMED_OFFSET: u32 = 0x00000020;
pub(super) const TIMER_ARMED_BITS: u32 = 0x0000000f;
pub(super) const TIMER_ARMED_RESET: u32 = 0x00000000;
pub(super) const TIMER_ARMED_MSB: i32 = 3;
pub(super) const TIMER_ARMED_LSB: i32 = 0;
pub(super) const TIMER_ARMED_ACCESS: &str = "WC";
// =============================================================================
// Register    : TIMER_TIMERAWH
// Description : Raw read from bits 63:32 of time (no side effects)
pub(super) const TIMER_TIMERAWH_OFFSET: u32 = 0x00000024;
pub(super) const TIMER_TIMERAWH_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMERAWH_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMERAWH_MSB: i32 = 31;
pub(super) const TIMER_TIMERAWH_LSB: i32 = 0;
pub(super) const TIMER_TIMERAWH_ACCESS: &str = "RO";
// =============================================================================
// Register    : TIMER_TIMERAWL
// Description : Raw read from bits 31:0 of time (no side effects)
pub(super) const TIMER_TIMERAWL_OFFSET: u32 = 0x00000028;
pub(super) const TIMER_TIMERAWL_BITS: u32 = 0xffffffff;
pub(super) const TIMER_TIMERAWL_RESET: u32 = 0x00000000;
pub(super) const TIMER_TIMERAWL_MSB: i32 = 31;
pub(super) const TIMER_TIMERAWL_LSB: i32 = 0;
pub(super) const TIMER_TIMERAWL_ACCESS: &str = "RO";
// =============================================================================
// Register    : TIMER_DBGPAUSE
// Description : Set bits high to enable pause when the corresponding debug
//               ports are active
pub(super) const TIMER_DBGPAUSE_OFFSET: u32 = 0x0000002c;
pub(super) const TIMER_DBGPAUSE_BITS: u32 = 0x00000006;
pub(super) const TIMER_DBGPAUSE_RESET: u32 = 0x00000007;
// -----------------------------------------------------------------------------
// Field       : TIMER_DBGPAUSE_DBG1
// Description : Pause when processor 1 is in debug mode
pub(super) const TIMER_DBGPAUSE_DBG1_RESET: u32 = 0x1;
pub(super) const TIMER_DBGPAUSE_DBG1_BITS: u32 = 0x00000004;
pub(super) const TIMER_DBGPAUSE_DBG1_MSB: i32 = 2;
pub(super) const TIMER_DBGPAUSE_DBG1_LSB: i32 = 2;
pub(super) const TIMER_DBGPAUSE_DBG1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_DBGPAUSE_DBG0
// Description : Pause when processor 0 is in debug mode
pub(super) const TIMER_DBGPAUSE_DBG0_RESET: u32 = 0x1;
pub(super) const TIMER_DBGPAUSE_DBG0_BITS: u32 = 0x00000002;
pub(super) const TIMER_DBGPAUSE_DBG0_MSB: i32 = 1;
pub(super) const TIMER_DBGPAUSE_DBG0_LSB: i32 = 1;
pub(super) const TIMER_DBGPAUSE_DBG0_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_PAUSE
// Description : Set high to pause the timer
pub(super) const TIMER_PAUSE_OFFSET: u32 = 0x00000030;
pub(super) const TIMER_PAUSE_BITS: u32 = 0x00000001;
pub(super) const TIMER_PAUSE_RESET: u32 = 0x00000000;
pub(super) const TIMER_PAUSE_MSB: i32 = 0;
pub(super) const TIMER_PAUSE_LSB: i32 = 0;
pub(super) const TIMER_PAUSE_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_INTR
// Description : Raw Interrupts
pub(super) const TIMER_INTR_OFFSET: u32 = 0x00000034;
pub(super) const TIMER_INTR_BITS: u32 = 0x0000000f;
pub(super) const TIMER_INTR_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : TIMER_INTR_ALARM_3
// Description : None
pub(super) const TIMER_INTR_ALARM_3_RESET: u32 = 0x0;
pub(super) const TIMER_INTR_ALARM_3_BITS: u32 = 0x00000008;
pub(super) const TIMER_INTR_ALARM_3_MSB: i32 = 3;
pub(super) const TIMER_INTR_ALARM_3_LSB: i32 = 3;
pub(super) const TIMER_INTR_ALARM_3_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTR_ALARM_2
// Description : None
pub(super) const TIMER_INTR_ALARM_2_RESET: u32 = 0x0;
pub(super) const TIMER_INTR_ALARM_2_BITS: u32 = 0x00000004;
pub(super) const TIMER_INTR_ALARM_2_MSB: i32 = 2;
pub(super) const TIMER_INTR_ALARM_2_LSB: i32 = 2;
pub(super) const TIMER_INTR_ALARM_2_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTR_ALARM_1
// Description : None
pub(super) const TIMER_INTR_ALARM_1_RESET: u32 = 0x0;
pub(super) const TIMER_INTR_ALARM_1_BITS: u32 = 0x00000002;
pub(super) const TIMER_INTR_ALARM_1_MSB: i32 = 1;
pub(super) const TIMER_INTR_ALARM_1_LSB: i32 = 1;
pub(super) const TIMER_INTR_ALARM_1_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTR_ALARM_0
// Description : None
pub(super) const TIMER_INTR_ALARM_0_RESET: u32 = 0x0;
pub(super) const TIMER_INTR_ALARM_0_BITS: u32 = 0x00000001;
pub(super) const TIMER_INTR_ALARM_0_MSB: i32 = 0;
pub(super) const TIMER_INTR_ALARM_0_LSB: i32 = 0;
pub(super) const TIMER_INTR_ALARM_0_ACCESS: &str = "WC";
// =============================================================================
// Register    : TIMER_INTE
// Description : Interrupt Enable
pub(super) const TIMER_INTE_OFFSET: u32 = 0x00000038;
pub(super) const TIMER_INTE_BITS: u32 = 0x0000000f;
pub(super) const TIMER_INTE_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : TIMER_INTE_ALARM_3
// Description : None
pub(super) const TIMER_INTE_ALARM_3_RESET: u32 = 0x0;
pub(super) const TIMER_INTE_ALARM_3_BITS: u32 = 0x00000008;
pub(super) const TIMER_INTE_ALARM_3_MSB: i32 = 3;
pub(super) const TIMER_INTE_ALARM_3_LSB: i32 = 3;
pub(super) const TIMER_INTE_ALARM_3_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTE_ALARM_2
// Description : None
pub(super) const TIMER_INTE_ALARM_2_RESET: u32 = 0x0;
pub(super) const TIMER_INTE_ALARM_2_BITS: u32 = 0x00000004;
pub(super) const TIMER_INTE_ALARM_2_MSB: i32 = 2;
pub(super) const TIMER_INTE_ALARM_2_LSB: i32 = 2;
pub(super) const TIMER_INTE_ALARM_2_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTE_ALARM_1
// Description : None
pub(super) const TIMER_INTE_ALARM_1_RESET: u32 = 0x0;
pub(super) const TIMER_INTE_ALARM_1_BITS: u32 = 0x00000002;
pub(super) const TIMER_INTE_ALARM_1_MSB: i32 = 1;
pub(super) const TIMER_INTE_ALARM_1_LSB: i32 = 1;
pub(super) const TIMER_INTE_ALARM_1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTE_ALARM_0
// Description : None
pub(super) const TIMER_INTE_ALARM_0_RESET: u32 = 0x0;
pub(super) const TIMER_INTE_ALARM_0_BITS: u32 = 0x00000001;
pub(super) const TIMER_INTE_ALARM_0_MSB: i32 = 0;
pub(super) const TIMER_INTE_ALARM_0_LSB: i32 = 0;
pub(super) const TIMER_INTE_ALARM_0_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_INTF
// Description : Interrupt Force
pub(super) const TIMER_INTF_OFFSET: u32 = 0x0000003c;
pub(super) const TIMER_INTF_BITS: u32 = 0x0000000f;
pub(super) const TIMER_INTF_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : TIMER_INTF_ALARM_3
// Description : None
pub(super) const TIMER_INTF_ALARM_3_RESET: u32 = 0x0;
pub(super) const TIMER_INTF_ALARM_3_BITS: u32 = 0x00000008;
pub(super) const TIMER_INTF_ALARM_3_MSB: i32 = 3;
pub(super) const TIMER_INTF_ALARM_3_LSB: i32 = 3;
pub(super) const TIMER_INTF_ALARM_3_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTF_ALARM_2
// Description : None
pub(super) const TIMER_INTF_ALARM_2_RESET: u32 = 0x0;
pub(super) const TIMER_INTF_ALARM_2_BITS: u32 = 0x00000004;
pub(super) const TIMER_INTF_ALARM_2_MSB: i32 = 2;
pub(super) const TIMER_INTF_ALARM_2_LSB: i32 = 2;
pub(super) const TIMER_INTF_ALARM_2_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTF_ALARM_1
// Description : None
pub(super) const TIMER_INTF_ALARM_1_RESET: u32 = 0x0;
pub(super) const TIMER_INTF_ALARM_1_BITS: u32 = 0x00000002;
pub(super) const TIMER_INTF_ALARM_1_MSB: i32 = 1;
pub(super) const TIMER_INTF_ALARM_1_LSB: i32 = 1;
pub(super) const TIMER_INTF_ALARM_1_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTF_ALARM_0
// Description : None
pub(super) const TIMER_INTF_ALARM_0_RESET: u32 = 0x0;
pub(super) const TIMER_INTF_ALARM_0_BITS: u32 = 0x00000001;
pub(super) const TIMER_INTF_ALARM_0_MSB: i32 = 0;
pub(super) const TIMER_INTF_ALARM_0_LSB: i32 = 0;
pub(super) const TIMER_INTF_ALARM_0_ACCESS: &str = "RW";
// =============================================================================
// Register    : TIMER_INTS
// Description : Interrupt status after masking & forcing
pub(super) const TIMER_INTS_OFFSET: u32 = 0x00000040;
pub(super) const TIMER_INTS_BITS: u32 = 0x0000000f;
pub(super) const TIMER_INTS_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : TIMER_INTS_ALARM_3
// Description : None
pub(super) const TIMER_INTS_ALARM_3_RESET: u32 = 0x0;
pub(super) const TIMER_INTS_ALARM_3_BITS: u32 = 0x00000008;
pub(super) const TIMER_INTS_ALARM_3_MSB: i32 = 3;
pub(super) const TIMER_INTS_ALARM_3_LSB: i32 = 3;
pub(super) const TIMER_INTS_ALARM_3_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTS_ALARM_2
// Description : None
pub(super) const TIMER_INTS_ALARM_2_RESET: u32 = 0x0;
pub(super) const TIMER_INTS_ALARM_2_BITS: u32 = 0x00000004;
pub(super) const TIMER_INTS_ALARM_2_MSB: i32 = 2;
pub(super) const TIMER_INTS_ALARM_2_LSB: i32 = 2;
pub(super) const TIMER_INTS_ALARM_2_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTS_ALARM_1
// Description : None
pub(super) const TIMER_INTS_ALARM_1_RESET: u32 = 0x0;
pub(super) const TIMER_INTS_ALARM_1_BITS: u32 = 0x00000002;
pub(super) const TIMER_INTS_ALARM_1_MSB: i32 = 1;
pub(super) const TIMER_INTS_ALARM_1_LSB: i32 = 1;
pub(super) const TIMER_INTS_ALARM_1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : TIMER_INTS_ALARM_0
// Description : None
pub(super) const TIMER_INTS_ALARM_0_RESET: u32 = 0x0;
pub(super) const TIMER_INTS_ALARM_0_BITS: u32 = 0x00000001;
pub(super) const TIMER_INTS_ALARM_0_MSB: i32 = 0;
pub(super) const TIMER_INTS_ALARM_0_LSB: i32 = 0;
pub(super) const TIMER_INTS_ALARM_0_ACCESS: &str = "RO";
// =============================================================================
