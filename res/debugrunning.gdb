# Run this script with arm-none-eabi-gdb -command=debugrunning.gdb ./build/minimal.elf
target remote :3333
layout asm
layout regs
focus cmd

define printvt    
  set $vtor=0xe000ed08
  set $ptr=*($vtor as *const u32)
  set $end=$ptr + 0x80
  # The last one is 0x20000000 + (16 + 32) * 4 - 4
  while($ptr<$end)
    x/x $ptr
    set $ptr=$ptr+4
  end
end  

define printtime
  set $tlow=*(0x4005400c as *const u32)
  set $thigh=*(0x40054008 as *const u32)
  p/$arg0 $thigh
  p/$arg0 $tlow
end

# Add here any other breakpoints or configurations needed

define comeon
 set $pc=$pc+2
 x/24x $r1-16
 si
end
