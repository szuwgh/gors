use crate::app::Builder;

use std::os::raw::c_void;
use std::slice;

#[no_mangle]
pub unsafe extern "C" fn new_builder() -> *mut c_void {
    Box::into_raw(Box::new(Builder::new())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn add_key(arg: *mut c_void, key: *const u8, len: u32) {
    let b: &mut Builder = &mut *(arg as *mut Builder);
    let k = slice::from_raw_parts(key, len as usize);
    b.add(k);
}

#[no_mangle]
pub unsafe extern "C" fn get(arg: *mut c_void) -> *const u8 {
    let b: &mut Builder = &mut *(arg as *mut Builder);
    b.get().as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn len(arg: *mut c_void) -> u32 {
    let b: &mut Builder = &mut *(arg as *mut Builder);
    b.len()
}
