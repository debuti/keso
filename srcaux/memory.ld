MEMORY
{
    BOOT_LOADER(rx): ORIGIN = 0x10000000, LENGTH = 0x100
    FLASH(rx)      : ORIGIN = 0x10000100, LENGTH = 2048k - 0x100
    RAM(rwx)       : ORIGIN = 0x20000000, LENGTH = 256k
    STACK0(rwx)    : ORIGIN = 0x20040000, LENGTH = 4k
    STACK1(rwx)    : ORIGIN = 0x20041000, LENGTH = 4k
}

SECTIONS
{
  PROVIDE(_stack0_start = ORIGIN(STACK0) + LENGTH(STACK0));
  PROVIDE(_stack1_start = ORIGIN(STACK1) + LENGTH(STACK1));

  /* ## Sections in FLASH */
  /* ### Boot loader */
  .boot2 ORIGIN(BOOT_LOADER) : {
      __boot2_start__ = .;
      KEEP (*(.boot2))
      __boot2_end__ = .;
  } > BOOT_LOADER

  ASSERT(__boot2_end__ - __boot2_start__ == 256,
         "ERROR: Pico second stage bootloader must be 256 bytes in size")


  /* ## Sections in RAM */
  /* ### Vector table for core 0 */
  .vector_table_0 :
  {
    LONG(_stack0_start);                /* Initial Stack Pointer */
    KEEP(*(.vt0));  

    /* Placeholder for other unused interrupts in the vector table. */
    . = ADDR(.vector_table_0) + (48*4);
  } > RAM AT > FLASH

  PROVIDE(_vector_table_0 = ADDR(.vector_table_0));
  PROVIDE(_reset_0 = ADDR(.vector_table_0) + 0x4);

  /* ### Vector table for core 1 */
  .vector_table_1 :
  {
    LONG(_stack1_start);
    KEEP(*(.vt1));  

    . = ADDR(.vector_table_1) + (48*4);
  } > RAM AT > FLASH

  PROVIDE(_vector_table_1 = ADDR(.vector_table_1));
  PROVIDE(_reset_1 = ADDR(.vector_table_1) + 0x4);

  
  PROVIDE(__stext = ADDR(.vector_table_1) + SIZEOF(.vector_table_1));

  /* ### .text */
  .text __stext :
  {
    *(.Reset);
    *(.text.isrs);
    *(.text .text.*);

    . = ALIGN(4);
    __etext = .;
  } > RAM AT > FLASH

  /* ### .rodata */
  .rodata :
  {
    __srodata = .;
    *(.rodata .rodata.*);

    /* 4-byte align the end (VMA) of this section.
       This is required by LLD to ensure the LMA of the following .data
       section will have the correct alignment. */
    . = ALIGN(4);
    __erodata = .;
  } > RAM AT > FLASH

  /* ### .data */
  .data :
  {
    __sdata = .;
    *(.data .data.*);
    
    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __edata = .;
  } > RAM AT > FLASH

  /* LMA of .data */
  __sidata = LOADADDR(.data);

  /* ### .bss */
  .bss :
  {
    __sbss = .;
    *(.bss .bss.*);

    . = ALIGN(4); /* 4-byte align the end (VMA) of this section */
    __ebss = .;
  } > RAM AT > FLASH

  /* ### .uninit */
  .uninit (NOLOAD) :
  {
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM AT > FLASH

  /* ### heap */
  . = ALIGN(16);
  __sheap = .;
  . = ORIGIN(RAM) + LENGTH(RAM);
  __eheap = .;

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

