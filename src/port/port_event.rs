use crate::bindings::{mb_event_enum, mb_port_ser_struct, BOOL};

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_init(inst: *mut mb_port_ser_struct) -> BOOL {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_post(
    inst: *mut mb_port_ser_struct,
    event: mb_event_enum,
) -> BOOL {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_get(
    inst: *mut mb_port_ser_struct,
    caller: *const core::ffi::c_void,
    event: *mut mb_event_enum,
) -> BOOL {
    todo!()
}
