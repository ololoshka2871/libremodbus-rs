use crate::bindings::{mb_port_ser_struct, BOOL, USHORT};
use fugit_timer::ExtU32;

fn get_timer_interface() -> &'static mut dyn crate::TimerInterface {
    unsafe {
        let p = crate::TIMER.as_ref().unwrap();
        &mut *core::ptr::from_raw_parts_mut(p.0, p.1)
    }
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_init(
    _inst: *mut mb_port_ser_struct,
    timeout_50us: USHORT,
) -> BOOL {
    let timer = get_timer_interface();
    timer
        .set_timeout((timeout_50us as u32 * 50).micros())
        .into()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_close(_inst: *mut mb_port_ser_struct) {
    let timer = get_timer_interface();
    timer.close();
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_enable(_inst: *mut mb_port_ser_struct) {
    let timer = get_timer_interface();
    timer.start();
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_disable(_inst: *mut mb_port_ser_struct) {
    let timer = get_timer_interface();
    timer.stop();
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_delay(_inst: *mut mb_port_ser_struct, timeout_50us: USHORT) {
    let timer = get_timer_interface();

    timer.set_timeout((timeout_50us as u32 * 50).micros());
    timer.start();
    timer.wait();
}
