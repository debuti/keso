target remote localhost:3333
load
monitor reset init
layout asm
layout regs
focus cmd

# HELP
#  si: step instruction
#  s:  high level step
#  c:  continue
#  q:  quit
#  x/x <addr>: view memory
#  set *(0x20020000 as *mut i32) = 0x0000CAFE: set memory address
#  j *0xd0d0d0d0: jump to address and continue execution
#  set $r1=10: set register
 