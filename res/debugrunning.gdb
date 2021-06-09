# Run this script with arm-none-eabi-gdb -command=debug.gdb ./build/minimal.elf
target remote :3333
layout asm
layout regs
focus cmd

# Add here any other breakpoints or configurations needed
