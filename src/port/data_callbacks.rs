use crate::bindings::{mb_err_enum, mb_inst_struct, mb_reg_mode_enum, BOOL, CHAR, UCHAR, USHORT};

#[no_mangle]
pub extern "C" fn mb_reg_input_cb(
    inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    reg_num: USHORT,
) -> mb_err_enum {
    todo!()
}
#[no_mangle]
pub extern "C" fn mb_reg_holding_cb(
    inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    reg_num: USHORT,
    mode: mb_reg_mode_enum,
) -> mb_err_enum {
    todo!()
}
#[no_mangle]
pub extern "C" fn mb_reg_coils_cb(
    inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    coil_num: USHORT,
    mode: mb_reg_mode_enum,
) -> mb_err_enum {
    todo!()
}
#[no_mangle]
pub extern "C" fn mb_reg_discrete_cb(
    inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    disc_num: USHORT,
) -> mb_err_enum {
    todo!()
}
