extern "C" {
    fn __nop();
    fn __sev();
    fn __wfe();
}

#[inline(never)]
pub fn nop() {unsafe{__nop();}}

#[inline(never)]
pub fn sev() {unsafe{__sev();}}

#[inline(never)]
pub fn wfe() {unsafe{__wfe();}}
