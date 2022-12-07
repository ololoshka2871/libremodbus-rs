use core::mem::size_of_val;

use crate::bindings::{mb_err_enum, mb_inst_struct, mb_reg_mode_enum, CHAR, USHORT};

fn get_data_interface() -> &'static mut dyn crate::DataInterface {
    unsafe {
        let p = crate::DATA.as_ref().unwrap();
        &mut *core::ptr::from_raw_parts_mut(p.0, p.1)
    }
}

#[cfg(feature = "inputs")]
#[no_mangle]
pub extern "C" fn mb_reg_input_cb(
    _inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    reg_num: USHORT,
) -> mb_err_enum {
    let data_interface = get_data_interface();
    data_interface.read_inputs(
        unsafe {
            core::slice::from_raw_parts_mut(
                reg_buff as *mut u8,
                reg_num as usize * size_of_val(&reg_addr),
            )
        },
        reg_addr - 1,
        reg_num,
    ) as mb_err_enum
}

#[cfg(feature="holdings")]
#[no_mangle]
pub extern "C" fn mb_reg_holding_cb(
    _inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    reg_num: USHORT,
    mode: mb_reg_mode_enum,
) -> mb_err_enum {
    let data_interface = get_data_interface();
    data_interface.rw_holdings(
        unsafe {
            core::slice::from_raw_parts_mut(
                reg_buff as *mut u8,
                reg_num as usize * size_of_val(&reg_addr),
            )
        },
        reg_addr - 1,
        reg_num,
        mode.into(),
    ) as mb_err_enum
}

#[cfg(feature="coils")]
#[no_mangle]
pub extern "C" fn mb_reg_coils_cb(
    _inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    coil_num: USHORT,
    mode: mb_reg_mode_enum,
) -> mb_err_enum {
    let data_interface = get_data_interface();
    data_interface.rw_coils(
        unsafe {
            core::slice::from_raw_parts_mut(
                reg_buff as *mut u8,
                (coil_num as usize / u8::BITS as usize) + 1,
            )
        },
        reg_addr - 1,
        coil_num,
        mode.into(),
    ) as mb_err_enum
}

#[cfg(feature = "d_inputs")]
#[no_mangle]
pub extern "C" fn mb_reg_discrete_cb(
    _inst: *mut mb_inst_struct,
    reg_buff: *mut CHAR,
    reg_addr: USHORT,
    disc_num: USHORT,
) -> mb_err_enum {
    let data_interface = get_data_interface();
    data_interface.read_discretes(
        unsafe {
            core::slice::from_raw_parts_mut(
                reg_buff as *mut u8,
                (disc_num as usize / u8::BITS as usize) + 1,
            )
        },
        reg_addr - 1,
        disc_num,
    ) as mb_err_enum
}
