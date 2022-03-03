use crate::app;

#[no_mangle]
pub extern "C" fn simple_rust_func_called_from_go(arg1: u8, arg2: u16, arg3: u32) -> usize {
    app::my_app_simple_rust_func_called_from_go(arg1, arg2, arg3) as usize
}
