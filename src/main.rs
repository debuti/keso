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


// 1 Reset
// 2 NMI
// 3 HardFault
// 4 MemManage
// 5 BusFault
// 6 UsageFault
// 7-10 Reserved
// 11 SVCall
// 12 DebugMonitor
// 13 Reserved
// 14 PendSV
// 15 SysTick
extern "C" {
    #[cfg(armv6m)]
    fn PreResetTrampoline() -> !;
}

#[link_section = ".vector_table.resetv"]
#[no_mangle]
#[cfg(armv6m)]
pub static __RESET: unsafe extern "C" fn() -> ! = PreResetTrampoline;

#[link_section = ".Reset"]
#[no_mangle]
#[cfg(armv6m)]
pub fn reset_handler() -> ! {
    extern "C" {
        // These symbols come from `memory.ld`
        static mut __sbss: u32; // Start of .bss section
        static mut __ebss: u32; // End of .bss section
        static mut __sdata: u32; // Start of .data section
        static mut __edata: u32; // End of .data section
        static __sidata: u32; // Start of .rodata section
        static mut _vector_table_0: u32; // Start of vector table core0
        static mut _vector_table_1: u32; // Start of vector table core1
    }

    unsafe {
      let sio = app::mcal::sio::Peripheral::new();
      if sio.get_core_num() == 0 {
        // Initialize (Zero) BSS
        unsafe {
            let mut sbss: *mut u32 = &mut __sbss;
            let ebss: *mut u32 = &mut __ebss;
    
            while sbss < ebss {
                write_volatile(sbss, zeroed());
                sbss = sbss.offset(1);
            }
        }
    
        // Initialize Data
        unsafe {
            let mut sdata: *mut u32 = &mut __sdata;
            let edata: *mut u32 = &mut __edata;
            let mut sidata: *const u32 = &__sidata;
    
            while sdata < edata {
                write_volatile(sdata, read(sidata));
                sdata = sdata.offset(1);
                sidata = sidata.offset(1);
            }
        }
    
        // Setup vector table for core 1
        unsafe {
            let mut ptr0: *mut u32 = &mut _vector_table_0;
            let end: *mut u32 = &mut _vector_table_1;
            let mut ptr1: *mut u32 = &mut _vector_table_1;
    
            while ptr0 < end {
                ptr0 = ptr0.offset(1);
                ptr1 = ptr1.offset(1);
                write_volatile(ptr1, read(ptr0));
            }
        }
    
        // Set clocking
        clocks_init();
      }
    }
    
    // Call user's main function
    app::main()
}

fn clocks_init() {
    unsafe {
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
}

#[link_section = ".vector_table.nmi"]
#[no_mangle]
pub static __NMI: fn() -> ! = nmi_handler;
pub fn nmi_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.hard_fault"]
#[no_mangle]
pub static __HARD_FAULT: fn() -> ! = hard_fault_handler;
pub fn hard_fault_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.mem_manage"]
#[no_mangle]
pub static __MEM_MANAGE: fn() -> ! = mem_manage_handler;
pub fn mem_manage_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.bus_fault"]
#[no_mangle]
pub static __BUS_FAULT: fn() -> ! = bus_fault_handler;
pub fn bus_fault_handler() -> ! {
    loop {}
}

#[link_section = ".vector_table.usage_fault"]
#[no_mangle]
pub static __USAGE_FAULT: fn() -> ! = usage_fault_handler;
pub fn usage_fault_handler() -> ! {
    loop {}
}
