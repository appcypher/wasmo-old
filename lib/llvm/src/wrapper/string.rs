// pub struct LLVMString {}
use std::ffi::CString;

pub unsafe fn to_rust_string(c_string: *mut i8) -> String {
    CString::from_raw(c_string)
        .into_string()
        .expect("Conversion from c_string failed")
}
