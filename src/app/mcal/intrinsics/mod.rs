extern "C" {
    fn __nop();
    fn __sev();
    fn __wfe();
    fn __dmb();
    fn __getprimask() -> u32;
    fn __disirq();
    fn __enairq(irq:u32);
}

pub fn nop() {unsafe{__nop();}}

pub fn sev() {unsafe{__sev();}}

pub fn wfe() {unsafe{__wfe();}}

pub fn dmb() {unsafe{__dmb();}}

pub fn getprimask() -> u32 {unsafe{__getprimask()}}

pub fn disirq() {unsafe{__disirq();}}

pub fn enairq(irq:u32) {unsafe{__enairq(irq);}}
