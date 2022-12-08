#![allow(non_upper_case_globals)]
use crate::bindings::{
    mb_port_ser_parity_enum, mb_port_ser_struct, BOOL, CHAR, FALSE, TRUE, UCHAR, ULONG,
};

pub(crate) fn get_serial_interface() -> &'static mut dyn crate::SerialInterface {
    unsafe {
        let p = crate::PORT.as_ref().unwrap();
        &mut *core::ptr::from_raw_parts_mut(p.0, p.1)
    }
}

#[no_mangle]
pub extern "C" fn mb_port_ser_init(
    _inst: *mut mb_port_ser_struct,
    baud: ULONG,
    data_bits: UCHAR,
    parity: mb_port_ser_parity_enum,
) -> BOOL {
    let port = get_serial_interface();
    port.configure(baud, data_bits, parity.into()).into()
}
#[no_mangle]
pub extern "C" fn mb_port_ser_close(_inst: *mut mb_port_ser_struct) -> BOOL {
    let port = get_serial_interface();
    port.close();
    TRUE
}

#[no_mangle]
pub extern "C" fn mb_port_ser_enable(
    _inst: *mut mb_port_ser_struct,
    rx_enable: BOOL,
    tx_enable: BOOL,
) {
    let port = get_serial_interface();
    port.enable(rx_enable == TRUE, tx_enable == TRUE);
}

#[no_mangle]
pub extern "C" fn mb_port_ser_get_byte(
    _inst: *mut mb_port_ser_struct,
    byte_buf: *mut CHAR,
) -> BOOL {
    let port = get_serial_interface();
    if let Some(b) = port.get_byte() {
        unsafe { *byte_buf = b as i8 };
        TRUE
    } else {
        FALSE
    }
}

#[no_mangle]
pub extern "C" fn mb_port_ser_put_byte(_inst: *mut mb_port_ser_struct, byte_va0l: CHAR) -> BOOL {
    let port = get_serial_interface();
    port.put_byte(byte_va0l as u8).into()
}
