#![allow(unused)]

#[no_mangle]
pub extern "C" fn rust_enter_critical() {
    cortex_m::interrupt::disable();
}

#[no_mangle]
pub unsafe extern "C" fn rust_exit_critical() {
    cortex_m::interrupt::enable();
}