# Context switch concept

## Exceptions
In ARM Cortex M microprocessors, and more especifically, in the ARMv6m architecture (M0 and M0+), the exceptions are handled as if they were functions. When an exception (1-15) or an interrupt (16-) is called (mind that interrupts are handled through the NVIC and you need to activate them if you want them to fire properly), the processor performs the following tasks:
 * Aligns (to 8) and pushes to the current stack, in this order, xPSR, PC+1, LR, R12, R3, R2, R1 and R0
  ** M0+ resets to MSP stack and privileged mode
 * Saves EXC_RETURN into LR, the specific value is defined in the following paragraph
 * Enters handler (and thus privileged) mode. Handler makes use of MSP always
 * Retrieves the handler address from VTOR[nb] and jumps to it

The specific value of EXC_RETURN defines the situation at the moment of the interrupt, and instructs the future exception return
 * 0xFFFFFFF1 Return to Handler Mode. Exception return gets state from the Main stack. On return execution uses the Main Stack.
 * 0xFFFFFFF9 Return to Thread Mode. Exception return gets state from the Main stack. On return execution uses the Main Stack.
 * 0xFFFFFFFD Return to Thread Mode. Exception return gets state from the Process stack. On return execution uses the Process Stack.

An exception return occurs when the processor is in Handler mode and one of the following instructions loads a value of 0xFXXXXXXX into the PC:
 * POP that includes loading the PC.
 * BX with any register.

Thanks to the EXC_RETURN value and the handler mode state, the core knows how to undo the exception state into the code that was executing prior to the exception. The specific steps are:
 * Determines which stack pointer (MSP or PSP) will be used to restore the processor context in the next steps
 * Does some integrity checks
 * Restores the context from the stack and resumes the execution in the saved PC+1

This is architected to allow plain functions work as exception handlers without any special treatment by the compiler. By saving the EXC_RETURN "flag" the core knows what to do.

Each exception has a priority (the lower the priority number, the higher the precedence the exception has), a number and a associated handler

The exceptions defined for v6m are
 1. Reset
 2. NMI (Non-maskable interrupt)
 3. Hardfault
 4-10. Reserved
 11. SVCall (Supervisor call)
 12-13. Reserved
 14. PendSV
 15. Systick
 
The numbers over that are associated to external interrupts and they depend on the architecture of the implementation. In the RP2040 are:
 16. TIMER_IRQ_0
 17. TIMER_IRQ_1
 18. TIMER_IRQ_2
 19. TIMER_IRQ_3
 20. PWM_IRQ_WRAP
 21. USBCTRL_IRQ
 22. XIP_IRQ 
 23. PIO0_IRQ_0
 24. PIO0_IRQ_1
 25. PIO1_IRQ_0
 26. PIO1_IRQ_1
 27. DMA_IRQ_0
 28. DMA_IRQ_1
 29. IO_IRQ_BANK0
 30. IO_IRQ_QSPI
 31. SIO_IRQ_PROC0
 32. SIO_IRQ_PROC1
 33. CLOCKS_IRQ
 34. SPI0_IRQ
 35. SPI1_IRQ
 36. UART0_IRQ
 37. UART1_IRQ
 38. ADC_IRQ_FIFO
 39. I2C0_IRQ
 40. I2C1_IRQ
 41. RTC_IRQ

## Stack handling
We will use PSP for thread mode (tasks), and MSP for handler mode (OS). To achieve this, the software will start the OS and once there it will switch to user mode (PSP + unprivileged) with 
 CONTROL = 0x03 /* Unpriv Thread Mode w/ PSP */
 ISB

before that the OS will have to prepare each task stack 
 PSP = task.sp+64 /* Set PSP to the top of the first task stack */

the scheduler will launch each task in place, so the OS needs to populate the initial stack appropiately, as if the task was interrupted by a context switch at the very beginning of its execution

## Concept
1. Tasks are created before releasing the scheduler
1.1. The task stack is created as a local array to the OS and its top is initialized
1.2. The task body is prepared as a divergent function
2. Schedule tables are created with expiry points and period, each expiry point will refer to a task
3. Scheduler is started, the alarm for the second expirypoint is set. The kernel does a reconfiguration to use PSP and user mode, it jumps to the first task.
4. When the exception handler fires, it will perform the context switch

# Refs
 * ARMv6-M Architecture Reference Manual
 * https://www.embeddedrelated.com/showarticle/912.php
 * https://interrupt.memfault.com/blog/arm-cortex-m-exceptions-and-nvic
 * https://github.com/adamheinrich/os.h/blob/master/src/os.c