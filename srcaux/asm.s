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
  