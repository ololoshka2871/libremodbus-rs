use crate::bindings::{mb_port_ser_struct, BOOL, USHORT};

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_init(
    inst: *mut mb_port_ser_struct,
    timeout_50us: USHORT,
) -> BOOL {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_close(inst: *mut mb_port_ser_struct) {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_enable(inst: *mut mb_port_ser_struct) {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_disable(inst: *mut mb_port_ser_struct) {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_tmr_delay(inst: *mut mb_port_ser_struct, timeout_50us: USHORT) {
    todo!()
}
