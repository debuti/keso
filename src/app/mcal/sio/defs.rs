// =============================================================================
// Register block : SIO
// Version        : 1
// Bus type       : apb
// Description    : Single-cycle IO block
//                  Provides core-local and inter-core hardware for the two
//                  processors, with single-cycle access.
// =============================================================================
// =============================================================================
// Register    : SIO_CPUID
// Description : Processor core identifier
//               Value is 0 when read from processor core 0, and 1 when read
//               from processor core 1.
pub(super) const SIO_CPUID_OFFSET: u32 = 0x00000000;
pub(super) const SIO_CPUID_BITS: u32 = 0xffffffff;
pub(super) const SIO_CPUID_RESET: &str = "-";
pub(super) const SIO_CPUID_MSB: i32 = 31;
pub(super) const SIO_CPUID_LSB: i32 = 0;
pub(super) const SIO_CPUID_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_GPIO_IN
// Description : Input value for GPIO pins
//               Input value for GPIO0...29
pub(super) const SIO_GPIO_IN_OFFSET: u32 = 0x00000004;
pub(super) const SIO_GPIO_IN_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_IN_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_IN_MSB: i32 = 29;
pub(super) const SIO_GPIO_IN_LSB: i32 = 0;
pub(super) const SIO_GPIO_IN_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_GPIO_HI_IN
// Description : Input value for QSPI pins
//               Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2,
//               SD3
pub(super) const SIO_GPIO_HI_IN_OFFSET: u32 = 0x00000008;
pub(super) const SIO_GPIO_HI_IN_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_IN_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_IN_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_IN_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_IN_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_GPIO_OUT
// Description : GPIO output value
//               Set output level (1/0 -> high/low) for GPIO0...29.
//               Reading back gives the last value written, NOT the input value
//               from the pins.
//               If core 0 and core 1 both write to GPIO_OUT simultaneously (or
//               to a SET/CLR/XOR alias),
//               the result is as though the write from core 0 took place first,
//               and the write from core 1 was then applied to that intermediate
//               result.
pub(super) const SIO_GPIO_OUT_OFFSET: u32 = 0x00000010;
pub(super) const SIO_GPIO_OUT_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OUT_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OUT_MSB: i32 = 29;
pub(super) const SIO_GPIO_OUT_LSB: i32 = 0;
pub(super) const SIO_GPIO_OUT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OUT_SET
// Description : GPIO output value set
//               Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`
pub(super) const SIO_GPIO_OUT_SET_OFFSET: u32 = 0x00000014;
pub(super) const SIO_GPIO_OUT_SET_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OUT_SET_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OUT_SET_MSB: i32 = 29;
pub(super) const SIO_GPIO_OUT_SET_LSB: i32 = 0;
pub(super) const SIO_GPIO_OUT_SET_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OUT_CLR
// Description : GPIO output value clear
//               Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &=
//               ~wdata`
pub(super) const SIO_GPIO_OUT_CLR_OFFSET: u32 = 0x00000018;
pub(super) const SIO_GPIO_OUT_CLR_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OUT_CLR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OUT_CLR_MSB: i32 = 29;
pub(super) const SIO_GPIO_OUT_CLR_LSB: i32 = 0;
pub(super) const SIO_GPIO_OUT_CLR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OUT_XOR
// Description : GPIO output value XOR
//               Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^=
//               wdata`
pub(super) const SIO_GPIO_OUT_XOR_OFFSET: u32 = 0x0000001c;
pub(super) const SIO_GPIO_OUT_XOR_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OUT_XOR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OUT_XOR_MSB: i32 = 29;
pub(super) const SIO_GPIO_OUT_XOR_LSB: i32 = 0;
pub(super) const SIO_GPIO_OUT_XOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OE
// Description : GPIO output enable
//               Set output enable (1/0 -> output/input) for GPIO0...29.
//               Reading back gives the last value written.
//               If core 0 and core 1 both write to GPIO_OE simultaneously (or
//               to a SET/CLR/XOR alias),
//               the result is as though the write from core 0 took place first,
//               and the write from core 1 was then applied to that intermediate
//               result.
pub(super) const SIO_GPIO_OE_OFFSET: u32 = 0x00000020;
pub(super) const SIO_GPIO_OE_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OE_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OE_MSB: i32 = 29;
pub(super) const SIO_GPIO_OE_LSB: i32 = 0;
pub(super) const SIO_GPIO_OE_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OE_SET
// Description : GPIO output enable set
//               Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`
pub(super) const SIO_GPIO_OE_SET_OFFSET: u32 = 0x00000024;
pub(super) const SIO_GPIO_OE_SET_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OE_SET_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OE_SET_MSB: i32 = 29;
pub(super) const SIO_GPIO_OE_SET_LSB: i32 = 0;
pub(super) const SIO_GPIO_OE_SET_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OE_CLR
// Description : GPIO output enable clear
//               Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &=
//               ~wdata`
pub(super) const SIO_GPIO_OE_CLR_OFFSET: u32 = 0x00000028;
pub(super) const SIO_GPIO_OE_CLR_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OE_CLR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OE_CLR_MSB: i32 = 29;
pub(super) const SIO_GPIO_OE_CLR_LSB: i32 = 0;
pub(super) const SIO_GPIO_OE_CLR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_OE_XOR
// Description : GPIO output enable XOR
//               Perform an atomic bitwise XOR on GPIO_OE, i.e. `GPIO_OE ^=
//               wdata`
pub(super) const SIO_GPIO_OE_XOR_OFFSET: u32 = 0x0000002c;
pub(super) const SIO_GPIO_OE_XOR_BITS: u32 = 0x3fffffff;
pub(super) const SIO_GPIO_OE_XOR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_OE_XOR_MSB: i32 = 29;
pub(super) const SIO_GPIO_OE_XOR_LSB: i32 = 0;
pub(super) const SIO_GPIO_OE_XOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OUT
// Description : QSPI output value
//               Set output level (1/0 -> high/low) for QSPI IO0...5.
//               Reading back gives the last value written, NOT the input value
//               from the pins.
//               If core 0 and core 1 both write to GPIO_HI_OUT simultaneously
//               (or to a SET/CLR/XOR alias),
//               the result is as though the write from core 0 took place first,
//               and the write from core 1 was then applied to that intermediate
//               result.
pub(super) const SIO_GPIO_HI_OUT_OFFSET: u32 = 0x00000030;
pub(super) const SIO_GPIO_HI_OUT_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OUT_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OUT_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OUT_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OUT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OUT_SET
// Description : QSPI output value set
//               Perform an atomic bit-set on GPIO_HI_OUT, i.e. `GPIO_HI_OUT |=
//               wdata`
pub(super) const SIO_GPIO_HI_OUT_SET_OFFSET: u32 = 0x00000034;
pub(super) const SIO_GPIO_HI_OUT_SET_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OUT_SET_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OUT_SET_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OUT_SET_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OUT_SET_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OUT_CLR
// Description : QSPI output value clear
//               Perform an atomic bit-clear on GPIO_HI_OUT, i.e. `GPIO_HI_OUT
//               &= ~wdata`
pub(super) const SIO_GPIO_HI_OUT_CLR_OFFSET: u32 = 0x00000038;
pub(super) const SIO_GPIO_HI_OUT_CLR_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OUT_CLR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OUT_CLR_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OUT_CLR_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OUT_CLR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OUT_XOR
// Description : QSPI output value XOR
//               Perform an atomic bitwise XOR on GPIO_HI_OUT, i.e. `GPIO_HI_OUT
//               ^= wdata`
pub(super) const SIO_GPIO_HI_OUT_XOR_OFFSET: u32 = 0x0000003c;
pub(super) const SIO_GPIO_HI_OUT_XOR_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OUT_XOR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OUT_XOR_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OUT_XOR_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OUT_XOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OE
// Description : QSPI output enable
//               Set output enable (1/0 -> output/input) for QSPI IO0...5.
//               Reading back gives the last value written.
//               If core 0 and core 1 both write to GPIO_HI_OE simultaneously
//               (or to a SET/CLR/XOR alias),
//               the result is as though the write from core 0 took place first,
//               and the write from core 1 was then applied to that intermediate
//               result.
pub(super) const SIO_GPIO_HI_OE_OFFSET: u32 = 0x00000040;
pub(super) const SIO_GPIO_HI_OE_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OE_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OE_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OE_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OE_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OE_SET
// Description : QSPI output enable set
//               Perform an atomic bit-set on GPIO_HI_OE, i.e. `GPIO_HI_OE |=
//               wdata`
pub(super) const SIO_GPIO_HI_OE_SET_OFFSET: u32 = 0x00000044;
pub(super) const SIO_GPIO_HI_OE_SET_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OE_SET_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OE_SET_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OE_SET_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OE_SET_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OE_CLR
// Description : QSPI output enable clear
//               Perform an atomic bit-clear on GPIO_HI_OE, i.e. `GPIO_HI_OE &=
//               ~wdata`
pub(super) const SIO_GPIO_HI_OE_CLR_OFFSET: u32 = 0x00000048;
pub(super) const SIO_GPIO_HI_OE_CLR_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OE_CLR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OE_CLR_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OE_CLR_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OE_CLR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_GPIO_HI_OE_XOR
// Description : QSPI output enable XOR
//               Perform an atomic bitwise XOR on GPIO_HI_OE, i.e. `GPIO_HI_OE
//               ^= wdata`
pub(super) const SIO_GPIO_HI_OE_XOR_OFFSET: u32 = 0x0000004c;
pub(super) const SIO_GPIO_HI_OE_XOR_BITS: u32 = 0x0000003f;
pub(super) const SIO_GPIO_HI_OE_XOR_RESET: u32 = 0x00000000;
pub(super) const SIO_GPIO_HI_OE_XOR_MSB: i32 = 5;
pub(super) const SIO_GPIO_HI_OE_XOR_LSB: i32 = 0;
pub(super) const SIO_GPIO_HI_OE_XOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_FIFO_ST
// Description : Status register for inter-core FIFOs (mailboxes).
//               There is one FIFO in the core 0 -> core 1 direction, and one
//               core 1 -> core 0. Both are 32 bits wide and 8 words deep.
//               Core 0 can see the read side of the 1->0 FIFO (RX), and the
//               write side of 0->1 FIFO (TX).
//               Core 1 can see the read side of the 0->1 FIFO (RX), and the
//               write side of 1->0 FIFO (TX).
//               The SIO IRQ for each core is the logical OR of the VLD, WOF and
//               ROE fields of its FIFO_ST register.
pub(super) const SIO_FIFO_ST_OFFSET: u32 = 0x00000050;
pub(super) const SIO_FIFO_ST_BITS: u32 = 0x0000000f;
pub(super) const SIO_FIFO_ST_RESET: u32 = 0x00000002;
// -----------------------------------------------------------------------------
// Field       : SIO_FIFO_ST_ROE
// Description : Sticky flag indicating the RX FIFO was read when empty. This
//               read was ignored by the FIFO.
pub(super) const SIO_FIFO_ST_ROE_RESET: u32 = 0x0;
pub(super) const SIO_FIFO_ST_ROE_BITS: u32 = 0x00000008;
pub(super) const SIO_FIFO_ST_ROE_MSB: i32 = 3;
pub(super) const SIO_FIFO_ST_ROE_LSB: i32 = 3;
pub(super) const SIO_FIFO_ST_ROE_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : SIO_FIFO_ST_WOF
// Description : Sticky flag indicating the TX FIFO was written when full. This
//               write was ignored by the FIFO.
pub(super) const SIO_FIFO_ST_WOF_RESET: u32 = 0x0;
pub(super) const SIO_FIFO_ST_WOF_BITS: u32 = 0x00000004;
pub(super) const SIO_FIFO_ST_WOF_MSB: i32 = 2;
pub(super) const SIO_FIFO_ST_WOF_LSB: i32 = 2;
pub(super) const SIO_FIFO_ST_WOF_ACCESS: &str = "WC";
// -----------------------------------------------------------------------------
// Field       : SIO_FIFO_ST_RDY
// Description : Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR
//               is ready for more data)
pub(super) const SIO_FIFO_ST_RDY_RESET: u32 = 0x1;
pub(super) const SIO_FIFO_ST_RDY_BITS: u32 = 0x00000002;
pub(super) const SIO_FIFO_ST_RDY_MSB: i32 = 1;
pub(super) const SIO_FIFO_ST_RDY_LSB: i32 = 1;
pub(super) const SIO_FIFO_ST_RDY_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_FIFO_ST_VLD
// Description : Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD
//               is valid)
pub(super) const SIO_FIFO_ST_VLD_RESET: u32 = 0x0;
pub(super) const SIO_FIFO_ST_VLD_BITS: u32 = 0x00000001;
pub(super) const SIO_FIFO_ST_VLD_MSB: i32 = 0;
pub(super) const SIO_FIFO_ST_VLD_LSB: i32 = 0;
pub(super) const SIO_FIFO_ST_VLD_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_FIFO_WR
// Description : Write access to this core's TX FIFO
pub(super) const SIO_FIFO_WR_OFFSET: u32 = 0x00000054;
pub(super) const SIO_FIFO_WR_BITS: u32 = 0xffffffff;
pub(super) const SIO_FIFO_WR_RESET: u32 = 0x00000000;
pub(super) const SIO_FIFO_WR_MSB: i32 = 31;
pub(super) const SIO_FIFO_WR_LSB: i32 = 0;
pub(super) const SIO_FIFO_WR_ACCESS: &str = "WF";
// =============================================================================
// Register    : SIO_FIFO_RD
// Description : Read access to this core's RX FIFO
pub(super) const SIO_FIFO_RD_OFFSET: u32 = 0x00000058;
pub(super) const SIO_FIFO_RD_BITS: u32 = 0xffffffff;
pub(super) const SIO_FIFO_RD_RESET: &str = "-";
pub(super) const SIO_FIFO_RD_MSB: i32 = 31;
pub(super) const SIO_FIFO_RD_LSB: i32 = 0;
pub(super) const SIO_FIFO_RD_ACCESS: &str = "RF";
// =============================================================================
// Register    : SIO_SPINLOCK_ST
// Description : Spinlock state
//               A bitmap containing the state of all 32 spinlocks (1=locked).
//               Mainly intended for debugging.
pub(super) const SIO_SPINLOCK_ST_OFFSET: u32 = 0x0000005c;
pub(super) const SIO_SPINLOCK_ST_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK_ST_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK_ST_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK_ST_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK_ST_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_DIV_UDIVIDEND
// Description : Divider unsigned dividend
//               Write to the DIVIDEND operand of the divider, i.e. the p in `p
//               / q`.
//               Any operand write starts a new calculation. The results appear
//               in QUOTIENT, REMAINDER.
//               UDIVIDEND/SDIVIDEND are aliases of the same internal register.
//               The U alias starts an
//               unsigned calculation, and the S alias starts a signed
//               calculation.
pub(super) const SIO_DIV_UDIVIDEND_OFFSET: u32 = 0x00000060;
pub(super) const SIO_DIV_UDIVIDEND_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_UDIVIDEND_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_UDIVIDEND_MSB: i32 = 31;
pub(super) const SIO_DIV_UDIVIDEND_LSB: i32 = 0;
pub(super) const SIO_DIV_UDIVIDEND_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_UDIVISOR
// Description : Divider unsigned divisor
//               Write to the DIVISOR operand of the divider, i.e. the q in `p /
//               q`.
//               Any operand write starts a new calculation. The results appear
//               in QUOTIENT, REMAINDER.
//               UDIVIDEND/SDIVIDEND are aliases of the same internal register.
//               The U alias starts an
//               unsigned calculation, and the S alias starts a signed
//               calculation.
pub(super) const SIO_DIV_UDIVISOR_OFFSET: u32 = 0x00000064;
pub(super) const SIO_DIV_UDIVISOR_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_UDIVISOR_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_UDIVISOR_MSB: i32 = 31;
pub(super) const SIO_DIV_UDIVISOR_LSB: i32 = 0;
pub(super) const SIO_DIV_UDIVISOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_SDIVIDEND
// Description : Divider signed dividend
//               The same as UDIVIDEND, but starts a signed calculation, rather
//               than unsigned.
pub(super) const SIO_DIV_SDIVIDEND_OFFSET: u32 = 0x00000068;
pub(super) const SIO_DIV_SDIVIDEND_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_SDIVIDEND_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_SDIVIDEND_MSB: i32 = 31;
pub(super) const SIO_DIV_SDIVIDEND_LSB: i32 = 0;
pub(super) const SIO_DIV_SDIVIDEND_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_SDIVISOR
// Description : Divider signed divisor
//               The same as UDIVISOR, but starts a signed calculation, rather
//               than unsigned.
pub(super) const SIO_DIV_SDIVISOR_OFFSET: u32 = 0x0000006c;
pub(super) const SIO_DIV_SDIVISOR_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_SDIVISOR_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_SDIVISOR_MSB: i32 = 31;
pub(super) const SIO_DIV_SDIVISOR_LSB: i32 = 0;
pub(super) const SIO_DIV_SDIVISOR_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_QUOTIENT
// Description : Divider result quotient
//               The result of `DIVIDEND / DIVISOR` (division). Contents
//               undefined while CSR_READY is low.
//               For signed calculations, QUOTIENT is negative when the signs of
//               DIVIDEND and DIVISOR differ.
//               This register can be written to directly, for context
//               save/restore purposes. This halts any
//               in-progress calculation and sets the CSR_READY and CSR_DIRTY
//               flags.
//               Reading from QUOTIENT clears the CSR_DIRTY flag, so should read
//               results in the order
//               REMAINDER, QUOTIENT if CSR_DIRTY is used.
pub(super) const SIO_DIV_QUOTIENT_OFFSET: u32 = 0x00000070;
pub(super) const SIO_DIV_QUOTIENT_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_QUOTIENT_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_QUOTIENT_MSB: i32 = 31;
pub(super) const SIO_DIV_QUOTIENT_LSB: i32 = 0;
pub(super) const SIO_DIV_QUOTIENT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_REMAINDER
// Description : Divider result remainder
//               The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined
//               while CSR_READY is low.
//               For signed calculations, REMAINDER is negative only when
//               DIVIDEND is negative.
//               This register can be written to directly, for context
//               save/restore purposes. This halts any
//               in-progress calculation and sets the CSR_READY and CSR_DIRTY
//               flags.
pub(super) const SIO_DIV_REMAINDER_OFFSET: u32 = 0x00000074;
pub(super) const SIO_DIV_REMAINDER_BITS: u32 = 0xffffffff;
pub(super) const SIO_DIV_REMAINDER_RESET: u32 = 0x00000000;
pub(super) const SIO_DIV_REMAINDER_MSB: i32 = 31;
pub(super) const SIO_DIV_REMAINDER_LSB: i32 = 0;
pub(super) const SIO_DIV_REMAINDER_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_DIV_CSR
// Description : Control and status register for divider.
pub(super) const SIO_DIV_CSR_OFFSET: u32 = 0x00000078;
pub(super) const SIO_DIV_CSR_BITS: u32 = 0x00000003;
pub(super) const SIO_DIV_CSR_RESET: u32 = 0x00000001;
// -----------------------------------------------------------------------------
// Field       : SIO_DIV_CSR_DIRTY
// Description : Changes to 1 when any register is written, and back to 0 when
//               QUOTIENT is read.
//               Software can use this flag to make save/restore more efficient
//               (skip if not DIRTY).
//               If the flag is used in this way, it's recommended to either
//               read QUOTIENT only,
//               or REMAINDER and then QUOTIENT, to prevent data loss on context
//               switch.
pub(super) const SIO_DIV_CSR_DIRTY_RESET: u32 = 0x0;
pub(super) const SIO_DIV_CSR_DIRTY_BITS: u32 = 0x00000002;
pub(super) const SIO_DIV_CSR_DIRTY_MSB: i32 = 1;
pub(super) const SIO_DIV_CSR_DIRTY_LSB: i32 = 1;
pub(super) const SIO_DIV_CSR_DIRTY_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_DIV_CSR_READY
// Description : Reads as 0 when a calculation is in progress, 1 otherwise.
//               Writing an operand (xDIVIDEND, xDIVISOR) will immediately start
//               a new calculation, no
//               matter if one is already in progress.
//               Writing to a result register will immediately terminate any
//               in-progress calculation
//               and set the READY and DIRTY flags.
pub(super) const SIO_DIV_CSR_READY_RESET: u32 = 0x1;
pub(super) const SIO_DIV_CSR_READY_BITS: u32 = 0x00000001;
pub(super) const SIO_DIV_CSR_READY_MSB: i32 = 0;
pub(super) const SIO_DIV_CSR_READY_LSB: i32 = 0;
pub(super) const SIO_DIV_CSR_READY_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_ACCUM0
// Description : Read/write access to accumulator 0
pub(super) const SIO_INTERP0_ACCUM0_OFFSET: u32 = 0x00000080;
pub(super) const SIO_INTERP0_ACCUM0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_ACCUM0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_ACCUM0_MSB: i32 = 31;
pub(super) const SIO_INTERP0_ACCUM0_LSB: i32 = 0;
pub(super) const SIO_INTERP0_ACCUM0_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_ACCUM1
// Description : Read/write access to accumulator 1
pub(super) const SIO_INTERP0_ACCUM1_OFFSET: u32 = 0x00000084;
pub(super) const SIO_INTERP0_ACCUM1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_ACCUM1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_ACCUM1_MSB: i32 = 31;
pub(super) const SIO_INTERP0_ACCUM1_LSB: i32 = 0;
pub(super) const SIO_INTERP0_ACCUM1_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_BASE0
// Description : Read/write access to BASE0 register.
pub(super) const SIO_INTERP0_BASE0_OFFSET: u32 = 0x00000088;
pub(super) const SIO_INTERP0_BASE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_BASE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_BASE0_MSB: i32 = 31;
pub(super) const SIO_INTERP0_BASE0_LSB: i32 = 0;
pub(super) const SIO_INTERP0_BASE0_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_BASE1
// Description : Read/write access to BASE1 register.
pub(super) const SIO_INTERP0_BASE1_OFFSET: u32 = 0x0000008c;
pub(super) const SIO_INTERP0_BASE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_BASE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_BASE1_MSB: i32 = 31;
pub(super) const SIO_INTERP0_BASE1_LSB: i32 = 0;
pub(super) const SIO_INTERP0_BASE1_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_BASE2
// Description : Read/write access to BASE2 register.
pub(super) const SIO_INTERP0_BASE2_OFFSET: u32 = 0x00000090;
pub(super) const SIO_INTERP0_BASE2_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_BASE2_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_BASE2_MSB: i32 = 31;
pub(super) const SIO_INTERP0_BASE2_LSB: i32 = 0;
pub(super) const SIO_INTERP0_BASE2_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_POP_LANE0
// Description : Read LANE0 result, and simultaneously write lane results to
//               both accumulators (POP).
pub(super) const SIO_INTERP0_POP_LANE0_OFFSET: u32 = 0x00000094;
pub(super) const SIO_INTERP0_POP_LANE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_POP_LANE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_POP_LANE0_MSB: i32 = 31;
pub(super) const SIO_INTERP0_POP_LANE0_LSB: i32 = 0;
pub(super) const SIO_INTERP0_POP_LANE0_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_POP_LANE1
// Description : Read LANE1 result, and simultaneously write lane results to
//               both accumulators (POP).
pub(super) const SIO_INTERP0_POP_LANE1_OFFSET: u32 = 0x00000098;
pub(super) const SIO_INTERP0_POP_LANE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_POP_LANE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_POP_LANE1_MSB: i32 = 31;
pub(super) const SIO_INTERP0_POP_LANE1_LSB: i32 = 0;
pub(super) const SIO_INTERP0_POP_LANE1_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_POP_FULL
// Description : Read FULL result, and simultaneously write lane results to both
//               accumulators (POP).
pub(super) const SIO_INTERP0_POP_FULL_OFFSET: u32 = 0x0000009c;
pub(super) const SIO_INTERP0_POP_FULL_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_POP_FULL_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_POP_FULL_MSB: i32 = 31;
pub(super) const SIO_INTERP0_POP_FULL_LSB: i32 = 0;
pub(super) const SIO_INTERP0_POP_FULL_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_PEEK_LANE0
// Description : Read LANE0 result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP0_PEEK_LANE0_OFFSET: u32 = 0x000000a0;
pub(super) const SIO_INTERP0_PEEK_LANE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_PEEK_LANE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_PEEK_LANE0_MSB: i32 = 31;
pub(super) const SIO_INTERP0_PEEK_LANE0_LSB: i32 = 0;
pub(super) const SIO_INTERP0_PEEK_LANE0_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_PEEK_LANE1
// Description : Read LANE1 result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP0_PEEK_LANE1_OFFSET: u32 = 0x000000a4;
pub(super) const SIO_INTERP0_PEEK_LANE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_PEEK_LANE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_PEEK_LANE1_MSB: i32 = 31;
pub(super) const SIO_INTERP0_PEEK_LANE1_LSB: i32 = 0;
pub(super) const SIO_INTERP0_PEEK_LANE1_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_PEEK_FULL
// Description : Read FULL result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP0_PEEK_FULL_OFFSET: u32 = 0x000000a8;
pub(super) const SIO_INTERP0_PEEK_FULL_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_PEEK_FULL_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_PEEK_FULL_MSB: i32 = 31;
pub(super) const SIO_INTERP0_PEEK_FULL_LSB: i32 = 0;
pub(super) const SIO_INTERP0_PEEK_FULL_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP0_CTRL_LANE0
// Description : Control register for lane 0
pub(super) const SIO_INTERP0_CTRL_LANE0_OFFSET: u32 = 0x000000ac;
pub(super) const SIO_INTERP0_CTRL_LANE0_BITS: u32 = 0x03bfffff;
pub(super) const SIO_INTERP0_CTRL_LANE0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_OVERF
// Description : Set if either OVERF0 or OVERF1 is set.
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF_BITS: u32 = 0x02000000;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF_MSB: i32 = 25;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF_LSB: i32 = 25;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_OVERF1
// Description : Indicates if any masked-off MSBs in ACCUM1 are set.
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF1_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF1_BITS: u32 = 0x01000000;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF1_MSB: i32 = 24;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF1_LSB: i32 = 24;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_OVERF0
// Description : Indicates if any masked-off MSBs in ACCUM0 are set.
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF0_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF0_BITS: u32 = 0x00800000;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF0_MSB: i32 = 23;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF0_LSB: i32 = 23;
pub(super) const SIO_INTERP0_CTRL_LANE0_OVERF0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_BLEND
// Description : Only present on INTERP0 on each core. If BLEND mode is enabled:
//               - LANE1 result is a linear interpolation between BASE0 and
//               BASE1, controlled
//               by the 8 LSBs of lane 1 shift and mask value (a fractional
//               number between
//               0 and 255/256ths)
//               - LANE0 result does not have BASE0 added (yields only the 8
//               LSBs of lane 1 shift+mask value)
//               - FULL result does not have lane 1 shift+mask value added
//               (BASE2 + lane 0 shift+mask)
//               LANE1 SIGNED flag controls whether the interpolation is signed
//               or unsigned.
pub(super) const SIO_INTERP0_CTRL_LANE0_BLEND_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_BLEND_BITS: u32 = 0x00200000;
pub(super) const SIO_INTERP0_CTRL_LANE0_BLEND_MSB: i32 = 21;
pub(super) const SIO_INTERP0_CTRL_LANE0_BLEND_LSB: i32 = 21;
pub(super) const SIO_INTERP0_CTRL_LANE0_BLEND_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_FORCE_MSB
// Description : ORed into bits 29:28 of the lane result presented to the
//               processor on the bus.
//               No effect on the internal 32-bit datapath. Handy for using a
//               lane to generate sequence
//               of pointers into flash or SRAM.
pub(super) const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_BITS: u32 = 0x00180000;
pub(super) const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_MSB: i32 = 20;
pub(super) const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_LSB: i32 = 19;
pub(super) const SIO_INTERP0_CTRL_LANE0_FORCE_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_ADD_RAW
// Description : If 1, mask + shift is bypassed for LANE0 result. This does not
//               affect FULL result.
pub(super) const SIO_INTERP0_CTRL_LANE0_ADD_RAW_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_ADD_RAW_BITS: u32 = 0x00040000;
pub(super) const SIO_INTERP0_CTRL_LANE0_ADD_RAW_MSB: i32 = 18;
pub(super) const SIO_INTERP0_CTRL_LANE0_ADD_RAW_LSB: i32 = 18;
pub(super) const SIO_INTERP0_CTRL_LANE0_ADD_RAW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_CROSS_RESULT
// Description : If 1, feed the opposite lane's result into this lane's
//               accumulator on POP.
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_BITS: u32 = 0x00020000;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_MSB: i32 = 17;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_LSB: i32 = 17;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_RESULT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_CROSS_INPUT
// Description : If 1, feed the opposite lane's accumulator into this lane's
//               shift + mask hardware.
//               Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is
//               before the shift+mask bypass)
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_BITS: u32 = 0x00010000;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_MSB: i32 = 16;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_LSB: i32 = 16;
pub(super) const SIO_INTERP0_CTRL_LANE0_CROSS_INPUT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_SIGNED
// Description : If SIGNED is set, the shifted and masked accumulator value is
//               sign-extended to 32 bits
//               before adding to BASE0, and LANE0 PEEK/POP appear extended to
//               32 bits when read by processor.
pub(super) const SIO_INTERP0_CTRL_LANE0_SIGNED_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE0_SIGNED_BITS: u32 = 0x00008000;
pub(super) const SIO_INTERP0_CTRL_LANE0_SIGNED_MSB: i32 = 15;
pub(super) const SIO_INTERP0_CTRL_LANE0_SIGNED_LSB: i32 = 15;
pub(super) const SIO_INTERP0_CTRL_LANE0_SIGNED_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_MASK_MSB
// Description : The most-significant bit allowed to pass by the mask
//               (inclusive)
//               Setting MSB < LSB may cause chip to turn inside-out
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_MSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_MSB_BITS: u32 = 0x00007c00;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_MSB_MSB: i32 = 14;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_MSB_LSB: i32 = 10;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_MASK_LSB
// Description : The least-significant bit allowed to pass by the mask
//               (inclusive)
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_LSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_LSB_BITS: u32 = 0x000003e0;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_LSB_MSB: i32 = 9;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_LSB_LSB: i32 = 5;
pub(super) const SIO_INTERP0_CTRL_LANE0_MASK_LSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE0_SHIFT
// Description : Logical right-shift applied to accumulator before masking
pub(super) const SIO_INTERP0_CTRL_LANE0_SHIFT_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE0_SHIFT_BITS: u32 = 0x0000001f;
pub(super) const SIO_INTERP0_CTRL_LANE0_SHIFT_MSB: i32 = 4;
pub(super) const SIO_INTERP0_CTRL_LANE0_SHIFT_LSB: i32 = 0;
pub(super) const SIO_INTERP0_CTRL_LANE0_SHIFT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_CTRL_LANE1
// Description : Control register for lane 1
pub(super) const SIO_INTERP0_CTRL_LANE1_OFFSET: u32 = 0x000000b0;
pub(super) const SIO_INTERP0_CTRL_LANE1_BITS: u32 = 0x001fffff;
pub(super) const SIO_INTERP0_CTRL_LANE1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_FORCE_MSB
// Description : ORed into bits 29:28 of the lane result presented to the
//               processor on the bus.
//               No effect on the internal 32-bit datapath. Handy for using a
//               lane to generate sequence
//               of pointers into flash or SRAM.
pub(super) const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_BITS: u32 = 0x00180000;
pub(super) const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_MSB: i32 = 20;
pub(super) const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_LSB: i32 = 19;
pub(super) const SIO_INTERP0_CTRL_LANE1_FORCE_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_ADD_RAW
// Description : If 1, mask + shift is bypassed for LANE1 result. This does not
//               affect FULL result.
pub(super) const SIO_INTERP0_CTRL_LANE1_ADD_RAW_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE1_ADD_RAW_BITS: u32 = 0x00040000;
pub(super) const SIO_INTERP0_CTRL_LANE1_ADD_RAW_MSB: i32 = 18;
pub(super) const SIO_INTERP0_CTRL_LANE1_ADD_RAW_LSB: i32 = 18;
pub(super) const SIO_INTERP0_CTRL_LANE1_ADD_RAW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_CROSS_RESULT
// Description : If 1, feed the opposite lane's result into this lane's
//               accumulator on POP.
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_BITS: u32 = 0x00020000;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_MSB: i32 = 17;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_LSB: i32 = 17;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_RESULT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_CROSS_INPUT
// Description : If 1, feed the opposite lane's accumulator into this lane's
//               shift + mask hardware.
//               Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is
//               before the shift+mask bypass)
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_BITS: u32 = 0x00010000;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_MSB: i32 = 16;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_LSB: i32 = 16;
pub(super) const SIO_INTERP0_CTRL_LANE1_CROSS_INPUT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_SIGNED
// Description : If SIGNED is set, the shifted and masked accumulator value is
//               sign-extended to 32 bits
//               before adding to BASE1, and LANE1 PEEK/POP appear extended to
//               32 bits when read by processor.
pub(super) const SIO_INTERP0_CTRL_LANE1_SIGNED_RESET: u32 = 0x0;
pub(super) const SIO_INTERP0_CTRL_LANE1_SIGNED_BITS: u32 = 0x00008000;
pub(super) const SIO_INTERP0_CTRL_LANE1_SIGNED_MSB: i32 = 15;
pub(super) const SIO_INTERP0_CTRL_LANE1_SIGNED_LSB: i32 = 15;
pub(super) const SIO_INTERP0_CTRL_LANE1_SIGNED_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_MASK_MSB
// Description : The most-significant bit allowed to pass by the mask
//               (inclusive)
//               Setting MSB < LSB may cause chip to turn inside-out
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_MSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_MSB_BITS: u32 = 0x00007c00;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_MSB_MSB: i32 = 14;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_MSB_LSB: i32 = 10;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_MASK_LSB
// Description : The least-significant bit allowed to pass by the mask
//               (inclusive)
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_LSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_LSB_BITS: u32 = 0x000003e0;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_LSB_MSB: i32 = 9;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_LSB_LSB: i32 = 5;
pub(super) const SIO_INTERP0_CTRL_LANE1_MASK_LSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP0_CTRL_LANE1_SHIFT
// Description : Logical right-shift applied to accumulator before masking
pub(super) const SIO_INTERP0_CTRL_LANE1_SHIFT_RESET: u32 = 0x00;
pub(super) const SIO_INTERP0_CTRL_LANE1_SHIFT_BITS: u32 = 0x0000001f;
pub(super) const SIO_INTERP0_CTRL_LANE1_SHIFT_MSB: i32 = 4;
pub(super) const SIO_INTERP0_CTRL_LANE1_SHIFT_LSB: i32 = 0;
pub(super) const SIO_INTERP0_CTRL_LANE1_SHIFT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_ACCUM0_ADD
// Description : Values written here are atomically added to ACCUM0
//               Reading yields lane 0's raw shift and mask value (BASE0 not
//               added).
pub(super) const SIO_INTERP0_ACCUM0_ADD_OFFSET: u32 = 0x000000b4;
pub(super) const SIO_INTERP0_ACCUM0_ADD_BITS: u32 = 0x00ffffff;
pub(super) const SIO_INTERP0_ACCUM0_ADD_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_ACCUM0_ADD_MSB: i32 = 23;
pub(super) const SIO_INTERP0_ACCUM0_ADD_LSB: i32 = 0;
pub(super) const SIO_INTERP0_ACCUM0_ADD_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_ACCUM1_ADD
// Description : Values written here are atomically added to ACCUM1
//               Reading yields lane 1's raw shift and mask value (BASE1 not
//               added).
pub(super) const SIO_INTERP0_ACCUM1_ADD_OFFSET: u32 = 0x000000b8;
pub(super) const SIO_INTERP0_ACCUM1_ADD_BITS: u32 = 0x00ffffff;
pub(super) const SIO_INTERP0_ACCUM1_ADD_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_ACCUM1_ADD_MSB: i32 = 23;
pub(super) const SIO_INTERP0_ACCUM1_ADD_LSB: i32 = 0;
pub(super) const SIO_INTERP0_ACCUM1_ADD_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP0_BASE_1AND0
// Description : On write, the lower 16 bits go to BASE0, upper bits to BASE1
//               simultaneously.
//               Each half is sign-extended to 32 bits if that lane's SIGNED
//               flag is set.
pub(super) const SIO_INTERP0_BASE_1AND0_OFFSET: u32 = 0x000000bc;
pub(super) const SIO_INTERP0_BASE_1AND0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP0_BASE_1AND0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP0_BASE_1AND0_MSB: i32 = 31;
pub(super) const SIO_INTERP0_BASE_1AND0_LSB: i32 = 0;
pub(super) const SIO_INTERP0_BASE_1AND0_ACCESS: &str = "WO";
// =============================================================================
// Register    : SIO_INTERP1_ACCUM0
// Description : Read/write access to accumulator 0
pub(super) const SIO_INTERP1_ACCUM0_OFFSET: u32 = 0x000000c0;
pub(super) const SIO_INTERP1_ACCUM0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_ACCUM0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_ACCUM0_MSB: i32 = 31;
pub(super) const SIO_INTERP1_ACCUM0_LSB: i32 = 0;
pub(super) const SIO_INTERP1_ACCUM0_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_ACCUM1
// Description : Read/write access to accumulator 1
pub(super) const SIO_INTERP1_ACCUM1_OFFSET: u32 = 0x000000c4;
pub(super) const SIO_INTERP1_ACCUM1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_ACCUM1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_ACCUM1_MSB: i32 = 31;
pub(super) const SIO_INTERP1_ACCUM1_LSB: i32 = 0;
pub(super) const SIO_INTERP1_ACCUM1_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_BASE0
// Description : Read/write access to BASE0 register.
pub(super) const SIO_INTERP1_BASE0_OFFSET: u32 = 0x000000c8;
pub(super) const SIO_INTERP1_BASE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_BASE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_BASE0_MSB: i32 = 31;
pub(super) const SIO_INTERP1_BASE0_LSB: i32 = 0;
pub(super) const SIO_INTERP1_BASE0_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_BASE1
// Description : Read/write access to BASE1 register.
pub(super) const SIO_INTERP1_BASE1_OFFSET: u32 = 0x000000cc;
pub(super) const SIO_INTERP1_BASE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_BASE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_BASE1_MSB: i32 = 31;
pub(super) const SIO_INTERP1_BASE1_LSB: i32 = 0;
pub(super) const SIO_INTERP1_BASE1_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_BASE2
// Description : Read/write access to BASE2 register.
pub(super) const SIO_INTERP1_BASE2_OFFSET: u32 = 0x000000d0;
pub(super) const SIO_INTERP1_BASE2_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_BASE2_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_BASE2_MSB: i32 = 31;
pub(super) const SIO_INTERP1_BASE2_LSB: i32 = 0;
pub(super) const SIO_INTERP1_BASE2_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_POP_LANE0
// Description : Read LANE0 result, and simultaneously write lane results to
//               both accumulators (POP).
pub(super) const SIO_INTERP1_POP_LANE0_OFFSET: u32 = 0x000000d4;
pub(super) const SIO_INTERP1_POP_LANE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_POP_LANE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_POP_LANE0_MSB: i32 = 31;
pub(super) const SIO_INTERP1_POP_LANE0_LSB: i32 = 0;
pub(super) const SIO_INTERP1_POP_LANE0_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_POP_LANE1
// Description : Read LANE1 result, and simultaneously write lane results to
//               both accumulators (POP).
pub(super) const SIO_INTERP1_POP_LANE1_OFFSET: u32 = 0x000000d8;
pub(super) const SIO_INTERP1_POP_LANE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_POP_LANE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_POP_LANE1_MSB: i32 = 31;
pub(super) const SIO_INTERP1_POP_LANE1_LSB: i32 = 0;
pub(super) const SIO_INTERP1_POP_LANE1_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_POP_FULL
// Description : Read FULL result, and simultaneously write lane results to both
//               accumulators (POP).
pub(super) const SIO_INTERP1_POP_FULL_OFFSET: u32 = 0x000000dc;
pub(super) const SIO_INTERP1_POP_FULL_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_POP_FULL_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_POP_FULL_MSB: i32 = 31;
pub(super) const SIO_INTERP1_POP_FULL_LSB: i32 = 0;
pub(super) const SIO_INTERP1_POP_FULL_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_PEEK_LANE0
// Description : Read LANE0 result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP1_PEEK_LANE0_OFFSET: u32 = 0x000000e0;
pub(super) const SIO_INTERP1_PEEK_LANE0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_PEEK_LANE0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_PEEK_LANE0_MSB: i32 = 31;
pub(super) const SIO_INTERP1_PEEK_LANE0_LSB: i32 = 0;
pub(super) const SIO_INTERP1_PEEK_LANE0_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_PEEK_LANE1
// Description : Read LANE1 result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP1_PEEK_LANE1_OFFSET: u32 = 0x000000e4;
pub(super) const SIO_INTERP1_PEEK_LANE1_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_PEEK_LANE1_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_PEEK_LANE1_MSB: i32 = 31;
pub(super) const SIO_INTERP1_PEEK_LANE1_LSB: i32 = 0;
pub(super) const SIO_INTERP1_PEEK_LANE1_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_PEEK_FULL
// Description : Read FULL result, without altering any internal state (PEEK).
pub(super) const SIO_INTERP1_PEEK_FULL_OFFSET: u32 = 0x000000e8;
pub(super) const SIO_INTERP1_PEEK_FULL_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_PEEK_FULL_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_PEEK_FULL_MSB: i32 = 31;
pub(super) const SIO_INTERP1_PEEK_FULL_LSB: i32 = 0;
pub(super) const SIO_INTERP1_PEEK_FULL_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_INTERP1_CTRL_LANE0
// Description : Control register for lane 0
pub(super) const SIO_INTERP1_CTRL_LANE0_OFFSET: u32 = 0x000000ec;
pub(super) const SIO_INTERP1_CTRL_LANE0_BITS: u32 = 0x03dfffff;
pub(super) const SIO_INTERP1_CTRL_LANE0_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_OVERF
// Description : Set if either OVERF0 or OVERF1 is set.
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF_BITS: u32 = 0x02000000;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF_MSB: i32 = 25;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF_LSB: i32 = 25;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_OVERF1
// Description : Indicates if any masked-off MSBs in ACCUM1 are set.
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF1_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF1_BITS: u32 = 0x01000000;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF1_MSB: i32 = 24;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF1_LSB: i32 = 24;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF1_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_OVERF0
// Description : Indicates if any masked-off MSBs in ACCUM0 are set.
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF0_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF0_BITS: u32 = 0x00800000;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF0_MSB: i32 = 23;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF0_LSB: i32 = 23;
pub(super) const SIO_INTERP1_CTRL_LANE0_OVERF0_ACCESS: &str = "RO";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_CLAMP
// Description : Only present on INTERP1 on each core. If CLAMP mode is enabled:
//               - LANE0 result is shifted and masked ACCUM0, clamped by a lower
//               bound of
//               BASE0 and an upper bound of BASE1.
//               - Signedness of these comparisons is determined by
//               LANE0_CTRL_SIGNED
pub(super) const SIO_INTERP1_CTRL_LANE0_CLAMP_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_CLAMP_BITS: u32 = 0x00400000;
pub(super) const SIO_INTERP1_CTRL_LANE0_CLAMP_MSB: i32 = 22;
pub(super) const SIO_INTERP1_CTRL_LANE0_CLAMP_LSB: i32 = 22;
pub(super) const SIO_INTERP1_CTRL_LANE0_CLAMP_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_FORCE_MSB
// Description : ORed into bits 29:28 of the lane result presented to the
//               processor on the bus.
//               No effect on the internal 32-bit datapath. Handy for using a
//               lane to generate sequence
//               of pointers into flash or SRAM.
pub(super) const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_BITS: u32 = 0x00180000;
pub(super) const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_MSB: i32 = 20;
pub(super) const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_LSB: i32 = 19;
pub(super) const SIO_INTERP1_CTRL_LANE0_FORCE_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_ADD_RAW
// Description : If 1, mask + shift is bypassed for LANE0 result. This does not
//               affect FULL result.
pub(super) const SIO_INTERP1_CTRL_LANE0_ADD_RAW_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_ADD_RAW_BITS: u32 = 0x00040000;
pub(super) const SIO_INTERP1_CTRL_LANE0_ADD_RAW_MSB: i32 = 18;
pub(super) const SIO_INTERP1_CTRL_LANE0_ADD_RAW_LSB: i32 = 18;
pub(super) const SIO_INTERP1_CTRL_LANE0_ADD_RAW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_CROSS_RESULT
// Description : If 1, feed the opposite lane's result into this lane's
//               accumulator on POP.
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_BITS: u32 = 0x00020000;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_MSB: i32 = 17;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_LSB: i32 = 17;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_RESULT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_CROSS_INPUT
// Description : If 1, feed the opposite lane's accumulator into this lane's
//               shift + mask hardware.
//               Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is
//               before the shift+mask bypass)
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_BITS: u32 = 0x00010000;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_MSB: i32 = 16;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_LSB: i32 = 16;
pub(super) const SIO_INTERP1_CTRL_LANE0_CROSS_INPUT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_SIGNED
// Description : If SIGNED is set, the shifted and masked accumulator value is
//               sign-extended to 32 bits
//               before adding to BASE0, and LANE0 PEEK/POP appear extended to
//               32 bits when read by processor.
pub(super) const SIO_INTERP1_CTRL_LANE0_SIGNED_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE0_SIGNED_BITS: u32 = 0x00008000;
pub(super) const SIO_INTERP1_CTRL_LANE0_SIGNED_MSB: i32 = 15;
pub(super) const SIO_INTERP1_CTRL_LANE0_SIGNED_LSB: i32 = 15;
pub(super) const SIO_INTERP1_CTRL_LANE0_SIGNED_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_MASK_MSB
// Description : The most-significant bit allowed to pass by the mask
//               (inclusive)
//               Setting MSB < LSB may cause chip to turn inside-out
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_MSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_MSB_BITS: u32 = 0x00007c00;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_MSB_MSB: i32 = 14;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_MSB_LSB: i32 = 10;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_MASK_LSB
// Description : The least-significant bit allowed to pass by the mask
//               (inclusive)
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_LSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_LSB_BITS: u32 = 0x000003e0;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_LSB_MSB: i32 = 9;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_LSB_LSB: i32 = 5;
pub(super) const SIO_INTERP1_CTRL_LANE0_MASK_LSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE0_SHIFT
// Description : Logical right-shift applied to accumulator before masking
pub(super) const SIO_INTERP1_CTRL_LANE0_SHIFT_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE0_SHIFT_BITS: u32 = 0x0000001f;
pub(super) const SIO_INTERP1_CTRL_LANE0_SHIFT_MSB: i32 = 4;
pub(super) const SIO_INTERP1_CTRL_LANE0_SHIFT_LSB: i32 = 0;
pub(super) const SIO_INTERP1_CTRL_LANE0_SHIFT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_CTRL_LANE1
// Description : Control register for lane 1
pub(super) const SIO_INTERP1_CTRL_LANE1_OFFSET: u32 = 0x000000f0;
pub(super) const SIO_INTERP1_CTRL_LANE1_BITS: u32 = 0x001fffff;
pub(super) const SIO_INTERP1_CTRL_LANE1_RESET: u32 = 0x00000000;
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_FORCE_MSB
// Description : ORed into bits 29:28 of the lane result presented to the
//               processor on the bus.
//               No effect on the internal 32-bit datapath. Handy for using a
//               lane to generate sequence
//               of pointers into flash or SRAM.
pub(super) const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_BITS: u32 = 0x00180000;
pub(super) const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_MSB: i32 = 20;
pub(super) const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_LSB: i32 = 19;
pub(super) const SIO_INTERP1_CTRL_LANE1_FORCE_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_ADD_RAW
// Description : If 1, mask + shift is bypassed for LANE1 result. This does not
//               affect FULL result.
pub(super) const SIO_INTERP1_CTRL_LANE1_ADD_RAW_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE1_ADD_RAW_BITS: u32 = 0x00040000;
pub(super) const SIO_INTERP1_CTRL_LANE1_ADD_RAW_MSB: i32 = 18;
pub(super) const SIO_INTERP1_CTRL_LANE1_ADD_RAW_LSB: i32 = 18;
pub(super) const SIO_INTERP1_CTRL_LANE1_ADD_RAW_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_CROSS_RESULT
// Description : If 1, feed the opposite lane's result into this lane's
//               accumulator on POP.
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_BITS: u32 = 0x00020000;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_MSB: i32 = 17;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_LSB: i32 = 17;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_RESULT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_CROSS_INPUT
// Description : If 1, feed the opposite lane's accumulator into this lane's
//               shift + mask hardware.
//               Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is
//               before the shift+mask bypass)
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_BITS: u32 = 0x00010000;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_MSB: i32 = 16;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_LSB: i32 = 16;
pub(super) const SIO_INTERP1_CTRL_LANE1_CROSS_INPUT_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_SIGNED
// Description : If SIGNED is set, the shifted and masked accumulator value is
//               sign-extended to 32 bits
//               before adding to BASE1, and LANE1 PEEK/POP appear extended to
//               32 bits when read by processor.
pub(super) const SIO_INTERP1_CTRL_LANE1_SIGNED_RESET: u32 = 0x0;
pub(super) const SIO_INTERP1_CTRL_LANE1_SIGNED_BITS: u32 = 0x00008000;
pub(super) const SIO_INTERP1_CTRL_LANE1_SIGNED_MSB: i32 = 15;
pub(super) const SIO_INTERP1_CTRL_LANE1_SIGNED_LSB: i32 = 15;
pub(super) const SIO_INTERP1_CTRL_LANE1_SIGNED_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_MASK_MSB
// Description : The most-significant bit allowed to pass by the mask
//               (inclusive)
//               Setting MSB < LSB may cause chip to turn inside-out
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_MSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_MSB_BITS: u32 = 0x00007c00;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_MSB_MSB: i32 = 14;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_MSB_LSB: i32 = 10;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_MSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_MASK_LSB
// Description : The least-significant bit allowed to pass by the mask
//               (inclusive)
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_LSB_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_LSB_BITS: u32 = 0x000003e0;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_LSB_MSB: i32 = 9;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_LSB_LSB: i32 = 5;
pub(super) const SIO_INTERP1_CTRL_LANE1_MASK_LSB_ACCESS: &str = "RW";
// -----------------------------------------------------------------------------
// Field       : SIO_INTERP1_CTRL_LANE1_SHIFT
// Description : Logical right-shift applied to accumulator before masking
pub(super) const SIO_INTERP1_CTRL_LANE1_SHIFT_RESET: u32 = 0x00;
pub(super) const SIO_INTERP1_CTRL_LANE1_SHIFT_BITS: u32 = 0x0000001f;
pub(super) const SIO_INTERP1_CTRL_LANE1_SHIFT_MSB: i32 = 4;
pub(super) const SIO_INTERP1_CTRL_LANE1_SHIFT_LSB: i32 = 0;
pub(super) const SIO_INTERP1_CTRL_LANE1_SHIFT_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_ACCUM0_ADD
// Description : Values written here are atomically added to ACCUM0
//               Reading yields lane 0's raw shift and mask value (BASE0 not
//               added).
pub(super) const SIO_INTERP1_ACCUM0_ADD_OFFSET: u32 = 0x000000f4;
pub(super) const SIO_INTERP1_ACCUM0_ADD_BITS: u32 = 0x00ffffff;
pub(super) const SIO_INTERP1_ACCUM0_ADD_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_ACCUM0_ADD_MSB: i32 = 23;
pub(super) const SIO_INTERP1_ACCUM0_ADD_LSB: i32 = 0;
pub(super) const SIO_INTERP1_ACCUM0_ADD_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_ACCUM1_ADD
// Description : Values written here are atomically added to ACCUM1
//               Reading yields lane 1's raw shift and mask value (BASE1 not
//               added).
pub(super) const SIO_INTERP1_ACCUM1_ADD_OFFSET: u32 = 0x000000f8;
pub(super) const SIO_INTERP1_ACCUM1_ADD_BITS: u32 = 0x00ffffff;
pub(super) const SIO_INTERP1_ACCUM1_ADD_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_ACCUM1_ADD_MSB: i32 = 23;
pub(super) const SIO_INTERP1_ACCUM1_ADD_LSB: i32 = 0;
pub(super) const SIO_INTERP1_ACCUM1_ADD_ACCESS: &str = "RW";
// =============================================================================
// Register    : SIO_INTERP1_BASE_1AND0
// Description : On write, the lower 16 bits go to BASE0, upper bits to BASE1
//               simultaneously.
//               Each half is sign-extended to 32 bits if that lane's SIGNED
//               flag is set.
pub(super) const SIO_INTERP1_BASE_1AND0_OFFSET: u32 = 0x000000fc;
pub(super) const SIO_INTERP1_BASE_1AND0_BITS: u32 = 0xffffffff;
pub(super) const SIO_INTERP1_BASE_1AND0_RESET: u32 = 0x00000000;
pub(super) const SIO_INTERP1_BASE_1AND0_MSB: i32 = 31;
pub(super) const SIO_INTERP1_BASE_1AND0_LSB: i32 = 0;
pub(super) const SIO_INTERP1_BASE_1AND0_ACCESS: &str = "WO";
// =============================================================================
// Register    : SIO_SPINLOCK0
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK0_OFFSET: u32 = 0x00000100;
pub(super) const SIO_SPINLOCK0_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK0_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK0_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK0_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK0_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK1
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK1_OFFSET: u32 = 0x00000104;
pub(super) const SIO_SPINLOCK1_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK1_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK1_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK1_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK1_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK2
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK2_OFFSET: u32 = 0x00000108;
pub(super) const SIO_SPINLOCK2_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK2_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK2_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK2_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK2_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK3
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK3_OFFSET: u32 = 0x0000010c;
pub(super) const SIO_SPINLOCK3_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK3_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK3_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK3_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK3_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK4
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK4_OFFSET: u32 = 0x00000110;
pub(super) const SIO_SPINLOCK4_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK4_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK4_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK4_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK4_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK5
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK5_OFFSET: u32 = 0x00000114;
pub(super) const SIO_SPINLOCK5_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK5_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK5_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK5_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK5_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK6
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK6_OFFSET: u32 = 0x00000118;
pub(super) const SIO_SPINLOCK6_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK6_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK6_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK6_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK6_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK7
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK7_OFFSET: u32 = 0x0000011c;
pub(super) const SIO_SPINLOCK7_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK7_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK7_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK7_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK7_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK8
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK8_OFFSET: u32 = 0x00000120;
pub(super) const SIO_SPINLOCK8_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK8_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK8_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK8_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK8_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK9
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK9_OFFSET: u32 = 0x00000124;
pub(super) const SIO_SPINLOCK9_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK9_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK9_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK9_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK9_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK10
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK10_OFFSET: u32 = 0x00000128;
pub(super) const SIO_SPINLOCK10_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK10_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK10_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK10_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK10_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK11
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK11_OFFSET: u32 = 0x0000012c;
pub(super) const SIO_SPINLOCK11_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK11_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK11_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK11_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK11_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK12
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK12_OFFSET: u32 = 0x00000130;
pub(super) const SIO_SPINLOCK12_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK12_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK12_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK12_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK12_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK13
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK13_OFFSET: u32 = 0x00000134;
pub(super) const SIO_SPINLOCK13_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK13_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK13_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK13_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK13_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK14
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK14_OFFSET: u32 = 0x00000138;
pub(super) const SIO_SPINLOCK14_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK14_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK14_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK14_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK14_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK15
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK15_OFFSET: u32 = 0x0000013c;
pub(super) const SIO_SPINLOCK15_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK15_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK15_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK15_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK15_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK16
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK16_OFFSET: u32 = 0x00000140;
pub(super) const SIO_SPINLOCK16_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK16_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK16_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK16_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK16_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK17
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK17_OFFSET: u32 = 0x00000144;
pub(super) const SIO_SPINLOCK17_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK17_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK17_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK17_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK17_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK18
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK18_OFFSET: u32 = 0x00000148;
pub(super) const SIO_SPINLOCK18_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK18_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK18_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK18_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK18_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK19
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK19_OFFSET: u32 = 0x0000014c;
pub(super) const SIO_SPINLOCK19_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK19_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK19_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK19_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK19_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK20
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK20_OFFSET: u32 = 0x00000150;
pub(super) const SIO_SPINLOCK20_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK20_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK20_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK20_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK20_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK21
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK21_OFFSET: u32 = 0x00000154;
pub(super) const SIO_SPINLOCK21_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK21_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK21_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK21_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK21_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK22
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK22_OFFSET: u32 = 0x00000158;
pub(super) const SIO_SPINLOCK22_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK22_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK22_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK22_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK22_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK23
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK23_OFFSET: u32 = 0x0000015c;
pub(super) const SIO_SPINLOCK23_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK23_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK23_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK23_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK23_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK24
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK24_OFFSET: u32 = 0x00000160;
pub(super) const SIO_SPINLOCK24_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK24_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK24_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK24_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK24_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK25
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK25_OFFSET: u32 = 0x00000164;
pub(super) const SIO_SPINLOCK25_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK25_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK25_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK25_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK25_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK26
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK26_OFFSET: u32 = 0x00000168;
pub(super) const SIO_SPINLOCK26_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK26_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK26_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK26_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK26_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK27
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK27_OFFSET: u32 = 0x0000016c;
pub(super) const SIO_SPINLOCK27_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK27_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK27_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK27_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK27_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK28
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK28_OFFSET: u32 = 0x00000170;
pub(super) const SIO_SPINLOCK28_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK28_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK28_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK28_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK28_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK29
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK29_OFFSET: u32 = 0x00000174;
pub(super) const SIO_SPINLOCK29_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK29_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK29_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK29_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK29_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK30
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK30_OFFSET: u32 = 0x00000178;
pub(super) const SIO_SPINLOCK30_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK30_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK30_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK30_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK30_ACCESS: &str = "RO";
// =============================================================================
// Register    : SIO_SPINLOCK31
// Description : Reading from a spinlock address will:
//               - Return 0 if lock is already locked
//               - Otherwise return nonzero, and simultaneously claim the lock
//
//               Writing (any value) releases the lock.
//               If core 0 and core 1 attempt to claim the same lock
//               simultaneously, core 0 wins.
//               The value returned on success is 0x1 << lock number.
pub(super) const SIO_SPINLOCK31_OFFSET: u32 = 0x0000017c;
pub(super) const SIO_SPINLOCK31_BITS: u32 = 0xffffffff;
pub(super) const SIO_SPINLOCK31_RESET: u32 = 0x00000000;
pub(super) const SIO_SPINLOCK31_MSB: i32 = 31;
pub(super) const SIO_SPINLOCK31_LSB: i32 = 0;
pub(super) const SIO_SPINLOCK31_ACCESS: &str = "RO";
// =============================================================================
