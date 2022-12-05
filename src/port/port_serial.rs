use crate::bindings::{mb_port_ser_parity_enum, mb_port_ser_struct, BOOL, CHAR, UCHAR, ULONG, TRUE};

#[no_mangle]
pub extern "C" fn mb_port_ser_init(
    inst: *mut mb_port_ser_struct,
    baud: ULONG,
    data_bits: UCHAR,
    parity: mb_port_ser_parity_enum,
) -> BOOL {
    // empty
    TRUE
}
#[no_mangle]
pub extern "C" fn mb_port_ser_close(inst: *mut mb_port_ser_struct) -> BOOL {
    todo!()
}
#[no_mangle]
pub extern "C" fn mb_port_ser_enable(
    inst: *mut mb_port_ser_struct,
    rx_enable: BOOL,
    tx_enable: BOOL,
) {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_get_byte(inst: *mut mb_port_ser_struct, byte_buf: *mut CHAR) -> BOOL {
    todo!()
}

#[no_mangle]
pub extern "C" fn mb_port_ser_put_byte(inst: *mut mb_port_ser_struct, byte_va0l: CHAR) -> BOOL {
    todo!()
}
