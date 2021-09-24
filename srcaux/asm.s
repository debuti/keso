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
  ldr r0,=0xffffffff
  mov lr,r0
  b reset_handler
.cfi_endproc
.size PreResetTrampoline, . - PreResetTrampoline

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

.global __setrx
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
