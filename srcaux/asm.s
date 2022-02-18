.cfi_sections .debug_frame

# ARMv6-M leaves LR in an unknown state on Reset
# this trampoline sets LR before it's pushed onto the stack by Reset
.section .PreResetTrampoline, "ax"

.global PreResetTrampoline
.type PreResetTrampoline,%function
.thumb_func
.cfi_startproc
PreResetTrampoline:
  # set LR to the initial value used by the ARMv7-M (0xFFFF_FFFF)
  mov r0, #0
  sub r0, #1
  mov lr, r0
  b reset_handler
.cfi_endproc
.size PreResetTrampoline, . - PreResetTrampoline

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

.global __nop
.type __nop,%function
.thumb_func
__nop:
  push {lr}
  nop
  pop {pc}

.global __sev
.type __sev,%function
.thumb_func
__sev:
  push {lr}
  sev
  pop {pc}

.global __wfe
.type __wfe,%function
.thumb_func
__wfe:
  push {lr}
  wfe
  pop {pc}

.global __dmb
.type __dmb,%function
.thumb_func
__dmb:
  push {lr}
  dmb
  pop {pc}

.global __isb
.type __isb,%function
.thumb_func
__isb:
  push {lr}
  isb
  pop {pc}

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
  
.global __setcontrol
.type __setcontrol,%function
.thumb_func
__setcontrol:
  push {r2, lr}
  lsl r2, r1, #1
  orr r2, r0
  msr CONTROL, r2
  pop {r2, pc}
  
.global __getcontrol
.type __getcontrol,%function
.thumb_func
__getcontrol:
  push {lr}
  mrs r0, CONTROL
  pop {pc}
  
.global __getpsp
.type __getpsp,%function
.thumb_func
__getpsp:
  push {lr}
  mrs r0, PSP
  pop {pc}
  
.global __setpsp
.type __setpsp,%function
.thumb_func
__setpsp:
  push {lr}
  msr PSP, r0
  pop {pc}

.global __getrx
.type __getrx,%function
.thumb_func
__getrx:
  push {lr}
__getrx_r0:
  cmp r0, #0
  bne __getrx_r1
  mov r0, r0
  pop {pc}
__getrx_r1:
  cmp r0, #1
  bne __getrx_r2
  mov r0, r1
  pop {pc}
__getrx_r2:
  cmp r0, #2
  bne __getrx_r3
  mov r0, r2
  pop {pc}
__getrx_r3:
  cmp r0, #3
  bne __getrx_r4
  mov r0, r3
  pop {pc}
__getrx_r4:
  cmp r0, #4
  bne __getrx_r5
  mov r0, r4
  pop {pc}
__getrx_r5:
  cmp r0, #5
  bne __getrx_r6
  mov r0, r5
  pop {pc}
__getrx_r6:
  cmp r0, #6
  bne __getrx_r7
  mov r0, r6
  pop {pc}
__getrx_r7:
  cmp r0, #7
  bne __getrx_r8
  mov r0, r7
  pop {pc}
__getrx_r8:
  cmp r0, #8
  bne __getrx_r9
  mov r0, r8
  pop {pc}
__getrx_r9:
  cmp r0, #9
  bne __getrx_r10
  mov r0, r9
  pop {pc}
__getrx_r10:
  cmp r0, #10
  bne __getrx_r11
  mov r0, r10
  pop {pc}
__getrx_r11:
  cmp r0, #11
  bne __getrx_r12
  mov r0, r11
  pop {pc}
__getrx_r12:
  cmp r0, #12
  bne __getrx_r13
  mov r0, r12
  pop {pc}
__getrx_r13:
  cmp r0, #13
  bne __getrx_r14
  mov r0, r13
  pop {pc}
__getrx_r14:
  cmp r0, #14
  bne __getrx_r15
  mov r0, r14
  pop {pc}
__getrx_r15:
  cmp r0, #15
  bne __getrx_error
  mov r0, r15
  pop {pc}
__getrx_error:
  mov r0, #0
  pop {pc}
  
.global __setrx
.type __setrx,%function
.thumb_func
__setrx:
  push {lr}
__setrx_r0:
  cmp r0, #0
  bne __setrx_r1
  mov r0, r1
  pop {pc}
__setrx_r1:
  cmp r0, #1
  bne __setrx_r2
  mov r1, r1
  pop {pc}
__setrx_r2:
  cmp r0, #2
  bne __setrx_r3
  mov r2, r1
  pop {pc}
__setrx_r3:
  cmp r0, #3
  bne __setrx_r4
  mov r3, r1
  pop {pc}
__setrx_r4:
  cmp r0, #4
  bne __setrx_r5
  mov r4, r1
  pop {pc}
__setrx_r5:
  cmp r0, #5
  bne __setrx_r6
  mov r5, r1
  pop {pc}
__setrx_r6:
  cmp r0, #6
  bne __setrx_r7
  mov r6, r1
  pop {pc}
__setrx_r7:
  cmp r0, #7
  bne __setrx_r8
  mov r7, r1
  pop {pc}
__setrx_r8:
  cmp r0, #8
  bne __setrx_r9
  mov r8, r1
  pop {pc}
__setrx_r9:
  cmp r0, #9
  bne __setrx_r10
  mov r9, r1
  pop {pc}
__setrx_r10:
  cmp r0, #10
  bne __setrx_r11
  mov r10, r1
  pop {pc}
__setrx_r11:
  cmp r0, #11
  bne __setrx_r12
  mov r11, r1
  pop {pc}
__setrx_r12:
  cmp r0, #12
  bne __setrx_r13
  mov r12, r1
  pop {pc}
__setrx_r13:
  cmp r0, #13
  bne __setrx_r14
  mov r13, r1
  pop {pc}
__setrx_r14:
  cmp r0, #14
  bne __setrx_r15
  mov r14, r1
  pop {pc}
__setrx_r15:
  cmp r0, #15
  bne __setrx_error
  mov r15, r1
__setrx_error:
  pop {pc}

.global __launch
.type __launch,%function
.thumb_func
__launch:
  push {lr}
  msr PSP, r1
  mov r1, #0x3
  msr CONTROL, r1
  isb
  bx r0
  /* will never return */
  udf 0xA
