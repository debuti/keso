extern "C" {
    fn __nop();
    fn __sev();
    fn __wfe();
    fn __dmb();
    fn __isb();
    fn __getprimask() -> u32;
    fn __disirq();
    fn __enairq(irq:u32);
    fn __setcontrol(privileged:u32, psp:u32);
    fn __getcontrol() -> u32;
    fn __getpsp() -> u32;
    fn __setpsp(value:u32);
    fn __getrx(idx:u8) -> u32;
    fn __setrx(idx:u8, value:u32);
    fn __launch(body:u32,psp:u32);
}

pub fn nop() {unsafe{__nop();}}

pub fn sev() {unsafe{__sev();}}

pub fn wfe() {unsafe{__wfe();}}

pub fn dmb() {unsafe{__dmb();}}

pub fn isb() {unsafe{__isb();}}

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

pub fn getpsp() -> usize {unsafe{__getpsp() as usize}}

pub fn setpsp(value:usize) {unsafe{__setpsp(value as u32)}}

pub fn getrx(idx:u8) -> u32 {unsafe{__getrx(idx) as u32}}

pub fn setrx(idx:u8, value:u32) {unsafe{__setrx(idx, value as u32)}}

pub fn launch(body:fn(), psp:usize) {unsafe{__launch(body as u32, psp as u32)}}
