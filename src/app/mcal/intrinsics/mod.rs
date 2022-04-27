extern "C" {
    fn __getprimask() -> u32;
    fn __disirq();
    fn __enairq(irq:u32);
    fn __setcontrol(privileged:u32, psp:u32);
    fn __getcontrol() -> u32;
}

pub fn nop() {unsafe {core::arch::asm!("nop");}}

pub fn sev() {unsafe {core::arch::asm!("sev");}}

pub fn wfe() {unsafe {core::arch::asm!("wfe");}}

pub fn dmb() {unsafe {core::arch::asm!("dmb");}}

pub fn isb() {unsafe {core::arch::asm!("isb");}}

pub fn getprimask() -> u32 {unsafe{__getprimask()}}

pub fn disirq() {unsafe{__disirq();}}

pub fn enairq(irq:u32) {unsafe{__enairq(irq);}}

pub fn setcontrol(privileged:bool, psp:bool) {unsafe{__setcontrol( if privileged {0} else {1},
                                                                   if psp {1} else {0});}}

pub fn getcontrol() -> (bool, bool) {
    unsafe {
        let control = __getcontrol();
        ( if control & 0x01 == 0x01 {false} else {true},
          if control & 0x02 == 0x02 {true} else {false} )
    }
}
