//  bindgen Rust -> C
// cbindgen C -> Rust
//      cxx C++ <-> Rust
//
// c2rust
// corrode
//
// neon
// wasm


use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_longlong},
};

#[no_mangle]
pub extern "C" fn ccacl_eval(expr: *const c_char, res: *mut c_longlong) -> c_int {
    let expr = unsafe {
        let cstr = CStr::from_ptr(expr);
        match cstr.to_str() {
            Ok(it) => it,
            Err(_) => return -1,
        }
    };

    let value = match calc::eval_str(expr) {
        Ok(it) => it,
        Err(_) => return -1,
    };

    unsafe {
        *res = value as c_longlong;
    }
    0
}
