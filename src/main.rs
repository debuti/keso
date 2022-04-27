#![no_std]
#![no_main]

use core::{
    mem::zeroed,
    panic::PanicInfo,
    ptr::{read, write_volatile},
};

mod app;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;


extern "C" {
    #[cfg(armv6m)]
    fn resetlanding();
}


#[link_section = ".Reset"]
#[no_mangle]
#[cfg(armv6m)]
pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `memory.ld`
        static mut __sbss: u32;  // Start of .bss section
        static mut __ebss: u32;  // End of .bss section
        static mut __sdata: u32; // Start of .data section
        static mut __edata: u32; // End of .data section
        static __sidata: u32;    // Start of .rodata section
    }

    unsafe {
      if app::mcal::sio::Peripheral::new().get_core_num() == 0 {
        // Initialize (Zero) BSS
        /* unsafe */ {
            let mut sbss: *mut u32 = &mut __sbss;
            let ebss: *mut u32 = &mut __ebss;
    
            while sbss < ebss {
                write_volatile(sbss, zeroed());
                sbss = sbss.offset(1);
            }
        }
    
        // Initialize Data
        /* unsafe */ {
            let mut sdata: *mut u32 = &mut __sdata;
            let edata: *mut u32 = &mut __edata;
            let mut sidata: *const u32 = &__sidata;
    
            while sdata < edata {
                write_volatile(sdata, read(sidata));
                sdata = sdata.offset(1);
                sidata = sidata.offset(1);
            }
        }
        
        // Set clocking
        clocks_init();
      }
    }

    // Call OS main function
    app::main()
}

fn clocks_init() {
    // Enable wdg tick 
    let mut wdg = app::mcal::watchdog::Peripheral::new();
    wdg.enable_tick(app::mcal::XOSC_MHZ);

    // Disable resus (clock system wdg) that may be enabled from previous software
    let mut clks = app::mcal::clocks::Peripheral::new();
    clks.disable_resus();

    let mut xosc = app::mcal::xosc::Peripheral::new();
    xosc.init();

    // Before we touch PLLs, switch sys and ref cleanly away from their aux sources.
    clks.reset_source(app::mcal::clocks::ClockIndex::Sys);
    clks.reset_source(app::mcal::clocks::ClockIndex::Ref);

    // Configure PLLs
    //                   REF     FBDIV VCO            POSTDIV
    // PLL SYS: 12 / 1 = 12MHz * 125 = 1500MHZ / 6 / 2 = 125MHz
    // PLL USB: 12 / 1 = 12MHz * 40  = 480 MHz / 5 / 2 =  48MHz

    let mut resets = app::mcal::resets::Peripheral::new();
    resets.disable(app::mcal::resets::ResetDevice::PllSys);
    resets.disable(app::mcal::resets::ResetDevice::PllUsb);
    resets.enable_wait(app::mcal::resets::ResetDevice::PllSys);
    resets.enable_wait(app::mcal::resets::ResetDevice::PllUsb);

    let mut pll_sys = app::mcal::pll::Peripheral::new(app::mcal::pll::Pll::Sys);
    pll_sys.init(1, 1500 * app::mcal::MHZ, 6, 2);
    let mut pll_usb = app::mcal::pll::Peripheral::new(app::mcal::pll::Pll::Usb);
    pll_usb.init(1, 480 * app::mcal::MHZ, 5, 2);


    // Configure clocks
    // CLK_REF = XOSC (12MHz) / 1 = 12MHz
    clks.clock_configure(app::mcal::clocks::ClockIndex::Ref,
        0x2, // xosc_clksrc,
        0, // No aux mux
        12 * app::mcal::MHZ,
        12 * app::mcal::MHZ).expect("Cannot config clock");
        
    // CLK SYS = PLL SYS (125MHz) / 1 = 125MHz
    clks.clock_configure(app::mcal::clocks::ClockIndex::Sys,
        0x1, // clksrc_clk_sys_aux,
        0x0, // clksrc_pll_sys,
        125 * app::mcal::MHZ,
        125 * app::mcal::MHZ).expect("Cannot config clock");

    // CLK USB = PLL USB (48MHz) / 1 = 48MHz
    clks.clock_configure(app::mcal::clocks::ClockIndex::Usb,
            0, // No GLMUX
            0x0, // clksrc_pll_usb
            48 * app::mcal::MHZ,
            48 * app::mcal::MHZ).expect("Cannot config clock");

    // CLK ADC = PLL USB (48MHZ) / 1 = 48MHz
    clks.clock_configure(app::mcal::clocks::ClockIndex::Adc,
            0, // No GLMUX
            0x0, // clksrc_pll_usb
            48 * app::mcal::MHZ,
            48 * app::mcal::MHZ).expect("Cannot config clock");

    // CLK RTC = PLL USB (48MHz) / 1024 = 46875Hz
    clks.clock_configure(app::mcal::clocks::ClockIndex::Rtc,
            0, // No GLMUX
            0x0, // clksrc_pll_usb
            48 * app::mcal::MHZ,
            46875).expect("Cannot config clock");

    // CLK PERI = clk_sys. Used as reference clock for Peripherals. No dividers so just select and enable
    // Normally choose clk_sys or clk_usb
    clks.clock_configure(app::mcal::clocks::ClockIndex::Peri,
            0,
            0x0, // clk_sys
            125 * app::mcal::MHZ,
            125 * app::mcal::MHZ).expect("Cannot config clock");
}

#[no_mangle]
pub extern "C" fn nmi_handler() {
    loop {}
}

#[no_mangle]
pub extern "C" fn hard_fault_handler() {
    // User should look into CFSR 0xE000ED30
    loop {}
}

#[no_mangle]
pub extern "C" fn nohandler() {
    loop {}
}


pub union ISR {
    _handler: unsafe extern "C" fn(),
    _unused: u32,
}

#[link_section = ".vt0"]
#[no_mangle]
pub static __ISRs_C0: [ISR; 47] = [ /* Idx zero is reserved to be populated by the linker */
    /* CM0+ specific */
    ISR { _handler: resetlanding       }, /* Reset */
    ISR { _handler: nmi_handler        }, /* NMI */
    ISR { _handler: hard_fault_handler }, /* HardFault */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _handler: nohandler          }, /* SVCall */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _handler: nohandler          }, /* PendSV */
    ISR { _handler: nohandler          }, /* SysTick */
    /* RP2040 specific */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_0 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_1 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_2 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_3 */
    ISR { _handler: nohandler          }, /* PWM_IRQ_WRAP */
    ISR { _handler: nohandler          }, /* USBCTRL_IRQ */
    ISR { _handler: nohandler          }, /* XIP_IRQ */
    ISR { _handler: nohandler          }, /* PIO0_IRQ_0 */
    ISR { _handler: nohandler          }, /* PIO0_IRQ_1 */
    ISR { _handler: nohandler          }, /* PIO1_IRQ_0 */
    ISR { _handler: nohandler          }, /* PIO1_IRQ_1 */
    ISR { _handler: nohandler          }, /* DMA_IRQ_0 */
    ISR { _handler: nohandler          }, /* DMA_IRQ_1 */
    ISR { _handler: nohandler          }, /* IO_IRQ_BANK0 */
    ISR { _handler: nohandler          }, /* IO_IRQ_QSPI */
    ISR { _handler: nohandler          }, /* SIO_IRQ_PROC0 */
    ISR { _handler: nohandler          }, /* SIO_IRQ_PROC1 */
    ISR { _handler: nohandler          }, /* CLOCKS_IRQ */
    ISR { _handler: nohandler          }, /* SPI0_IRQ */
    ISR { _handler: nohandler          }, /* SPI1_IRQ */
    ISR { _handler: nohandler          }, /* UART0_IRQ */
    ISR { _handler: nohandler          }, /* UART1_IRQ */
    ISR { _handler: nohandler          }, /* ADC_IRQ_FIFO */
    ISR { _handler: nohandler          }, /* I2C0_IRQ */
    ISR { _handler: nohandler          }, /* I2C1_IRQ */
    ISR { _handler: nohandler          }, /* RTC_IRQ */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0xFFFFFF00 },
];

#[link_section = ".vt1"]
#[no_mangle]
pub static __ISRs_C1: [ISR; 47] = [ /* Idx zero is reserved to be populated by the linker */
    /* CM0+ specific */
    ISR { _handler: resetlanding       }, /* Reset */
    ISR { _handler: nmi_handler        }, /* NMI */
    ISR { _handler: hard_fault_handler }, /* HardFault */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _handler: nohandler          }, /* SVCall */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _handler: nohandler          }, /* PendSV */
    ISR { _handler: nohandler          }, /* SysTick */
    /* RP2040 specific */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_0 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_1 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_2 */
    ISR { _handler: nohandler          }, /* TIMER_IRQ_3 */
    ISR { _handler: nohandler          }, /* PWM_IRQ_WRAP */
    ISR { _handler: nohandler          }, /* USBCTRL_IRQ */
    ISR { _handler: nohandler          }, /* XIP_IRQ */
    ISR { _handler: nohandler          }, /* PIO0_IRQ_0 */
    ISR { _handler: nohandler          }, /* PIO0_IRQ_1 */
    ISR { _handler: nohandler          }, /* PIO1_IRQ_0 */
    ISR { _handler: nohandler          }, /* PIO1_IRQ_1 */
    ISR { _handler: nohandler          }, /* DMA_IRQ_0 */
    ISR { _handler: nohandler          }, /* DMA_IRQ_1 */
    ISR { _handler: nohandler          }, /* IO_IRQ_BANK0 */
    ISR { _handler: nohandler          }, /* IO_IRQ_QSPI */
    ISR { _handler: nohandler          }, /* SIO_IRQ_PROC0 */
    ISR { _handler: nohandler          }, /* SIO_IRQ_PROC1 */
    ISR { _handler: nohandler          }, /* CLOCKS_IRQ */
    ISR { _handler: nohandler          }, /* SPI0_IRQ */
    ISR { _handler: nohandler          }, /* SPI1_IRQ */
    ISR { _handler: nohandler          }, /* UART0_IRQ */
    ISR { _handler: nohandler          }, /* UART1_IRQ */
    ISR { _handler: nohandler          }, /* ADC_IRQ_FIFO */
    ISR { _handler: nohandler          }, /* I2C0_IRQ */
    ISR { _handler: nohandler          }, /* I2C1_IRQ */
    ISR { _handler: nohandler          }, /* RTC_IRQ */
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0 },
    ISR { _unused: 0xFFFFFF01 },
];
