use core::cell::Cell;

use cortex_m::interrupt::Mutex;

use crate::bindings::{mb_event_enum, mb_event_enum_EV_READY, mb_port_ser_struct, BOOL, TRUE};

static mut EVENT: Mutex<Cell<mb_event_enum>> = Mutex::new(Cell::new(mb_event_enum_EV_READY));

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_init(_inst: *mut mb_port_ser_struct) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_post(
    _inst: *mut mb_port_ser_struct,
    event: mb_event_enum,
) -> BOOL {
    cortex_m::interrupt::free(|cs| {
        unsafe { EVENT.borrow(cs).set(event) };
    });
    TRUE
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_get(
    _inst: *mut mb_port_ser_struct,
    _caller: *const core::ffi::c_void,
    event: *mut mb_event_enum,
) -> BOOL {
    cortex_m::interrupt::free(|cs| {
        unsafe { *event = EVENT.borrow(cs).get() };
    });
    TRUE
}
