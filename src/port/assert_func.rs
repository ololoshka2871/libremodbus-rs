#[no_mangle]
pub extern "C" fn __assert_func(
    _file: *const u8,
    _line: i32,
    _func: *const u8,
    _text: *const u8,
) -> ! {
    panic!("Assert in \"C\" code");
}
