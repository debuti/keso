.cfi_sections .debug_frame

# ARMv6-M leaves LR in an unknown state on Reset
# this trampoline sets LR before it's pushed onto the stack by Reset
.section .text.isrs, "ax"

.global resetlanding
.type resetlanding,%function
.thumb_func
.cfi_startproc
resetlanding:
  # set LR to the initial value used by the ARMv7-M (0xFFFF_FFFF)
  mov r0, #0
  sub r0, #1
  mov lr, r0
  b reset_handler
.cfi_endproc
.size resetlanding, . - resetlanding

.global svclanding
.type svclanding,%function
.thumb_func
.cfi_startproc
svclanding:
  push {lr}
	/* There is no need to disable interrupts as this is the most prio interrupt handler (SHPR2 defaults to 0) */
  mrs r1, PSP  /* All syscalls are made from userland */
  ldr r0, [r1, #24] 
  sub r0, #2
  ldrh r0, [r0]
  mov r1, #0xFF
  and r0, r1
  bl svchandler
  pop {pc}
.cfi_endproc
.size svclanding, . - svclanding

.global ctxtswtr
.type ctxtswtr,%function
.thumb_func
.cfi_startproc
ctxtswtr:
	cpsid	i

  # Identify if we come from kernel or user mode
  mov r3, #0x0
  sub r3, r3, #0xF   /* 0xFFFF_FFF1 */
  mov r2, LR
  sub r2, r2, r3

  cmp r2, #0x0       /* Nested exception */
  beq .              /* unimplemented!() */
  cmp r2, #0x8       /* CPU was running kernel code */
  bne . + 0xA
  mov r2, #0
  mrs r0, MSP
  b . + 0xC
  cmp r2, #0xC       /* CPU was running user code */
  bne .              /* unreachable!() */
  mov r2, #1
  mrs r0, PSP

  # Advance the stacked PC here:
  # * If PSP
  #   * if the task didnt finish there is no problem, the execution will halt shortly after, in the ctxtswtr
  #   * if the task did finish great, thats what we want
  # * If MSP the flow will never return to it so no problem (but if it passes somehow, Rust has prepared a panic because SchedTable::start is divergent)
  ldr r1, [r0, #24]  /* Indirect to PC */
  add r1, #2
  str r1, [r0, #24]

  # Save (the rest of) the context to the stack
  sub r0, #32
  stmia r0!, {r4-r7}
	mov	r4, r8
	mov	r5, r9
	mov	r6, r10
	mov	r7, r11
  stmia r0!, {r4-r7}
  /*
          |  ...  | ^- 0xFFFFFFFF
          | xPSR  |
          | PC+1  |
          |  LR   |
          |  R12  |
          |  R3   |
          |  R2   |
          |  R1   |
          |  R0   | <- Until here was pushed by the architecture
          |  R11  |
          |  R10  |
          |  R9   |
          |  R8   |
          |  R7   |
          |  R6   |
          |  R5   |
    SP -> |  R4   | <- Until here was pushed by us
  */

  # Save SP to R0
  mov r1, r0
  sub r1, #32

  # Identify running exception from xPSR
  mrs r0, xPSR
  mov r4, #0xFF
  and r0, r4
  sub r0, #0x10 /* R0 now holds the interrupt number */

  # Call high level code to setup the next timer and return next task SP
  bl alarmhandler /* R0 will hold the new PSP value */

  # Restore PSP to next task SP
  mov r1, r0
  add r1, #16
  ldmia	r1!, {r4-r7}
	mov	r8, r4
	mov	r9, r5
	mov	r10, r6
	mov	r11, r7
  msr PSP, r1
  ldmia	r0!, {r4-r7}

  # Remove privileges!
  //mov r0, #0x3 /* unpriv + PSP */
  //mov r0, #0x2 /* priv + PSP */
  //msr CONTROL, r0

  # Return from exception
  mov r0, #0
  sub r0, r0, #3
  mov lr, r0     /* Place 0xFFFFFFFD on LR */
	cpsie	i
  bx lr
.cfi_endproc
.size ctxtswtr, . - ctxtswtr

.global __getprimask
.type __getprimask,%function
.thumb_func
__getprimask:
  push {lr}
  mrs r0, PRIMASK
  pop {pc}

.global __disirq
.type __disirq,%function
.thumb_func
__disirq:
  push {lr}
  cpsid i
  pop {pc}

.global __enairq
.type __enairq,%function
.thumb_func
__enairq:
  push {lr}
  msr PRIMASK, r0
  pop {pc}
