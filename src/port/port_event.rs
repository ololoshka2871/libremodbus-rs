use core::cell::Cell;

use cortex_m::interrupt::Mutex;

use crate::bindings::{
    mb_event_enum, mb_port_ser_struct, BOOL, FALSE, TRUE,
};

static mut EVENT: Mutex<Cell<Option<mb_event_enum>>> = Mutex::new(Cell::new(None));

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_init(_inst: *mut mb_port_ser_struct) -> BOOL {
    TRUE
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_post(
    _inst: *mut mb_port_ser_struct,
    event: mb_event_enum,
) -> BOOL {
    cortex_m::interrupt::free(|cs| unsafe {
        if let Some(_) = EVENT.borrow(cs).get() {
            FALSE
        } else {
            EVENT.borrow(cs).set(Some(event));
            TRUE
        }
    })
}

#[no_mangle]
pub extern "C" fn mb_port_ser_evt_get(
    _inst: *mut mb_port_ser_struct,
    _caller: *const core::ffi::c_void,
    event: *mut mb_event_enum,
) -> BOOL {
    cortex_m::interrupt::free(|cs| unsafe {
        if let Some(ev) = EVENT.borrow(cs).get() {
            *event = ev;
            EVENT.borrow(cs).set(None);
            TRUE
        } else {
            FALSE
        }
    })
}
