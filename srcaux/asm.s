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
  