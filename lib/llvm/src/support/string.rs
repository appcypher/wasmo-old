use libc::c_char;
use llvm_sys::core::LLVMDisposeMessage;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display, Formatter};
use wasmo_utils::{verbose, debug};

/// A class for managing strings created by the LLVM library.
pub struct LLVMString {
    buffer_ptr: *mut c_char,
}

impl LLVMString {
    pub(crate) fn new(buffer_ptr: *mut c_char) -> Self {
        Self { buffer_ptr }
    }

    /// The string argument becomes owned LLVMstring is allowed to free it.
    /// The string is expected to be from LLVM.
    pub fn from_c_string(c_string: CString) -> Self {
        // Transfer ownership to LLVMString.
        LLVMString::new(c_string.into_raw())
    }

    ///
    pub fn to_string(&self) -> String {
        // Clones the content buffer_ptr points to using `to_string_lossy`.
        unsafe {
            CStr::from_ptr(self.buffer_ptr)
                .to_string_lossy()
                .into_owned()
        }
    }

    ///
    pub fn into_c_string(&mut self) -> CString {
        // Transfer ownership of buffer_ptr underlying data.
        let c_string = unsafe { CString::from_raw(self.buffer_ptr) };

        // Set the buffer_ptr to null
        self.buffer_ptr = std::ptr::null_mut();

        c_string
    }

    ///
    pub fn into_raw(&mut self) -> *mut c_char {
        // Transfer ownership of buffer_ptr underlying data.
        let buffer_ptr = self.buffer_ptr;

        // Set the buffer_ptr to null
        self.buffer_ptr = std::ptr::null_mut();

        buffer_ptr
    }
}

impl Drop for LLVMString {
    fn drop(&mut self) {
        if !self.buffer_ptr.is_null() {
            debug!("LLVMString({}) drop!", self);
            unsafe {
                LLVMDisposeMessage(self.buffer_ptr);
            }
        }
    }
}

impl Debug for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", unsafe { CStr::from_ptr(self.buffer_ptr) })
    }
}

impl Display for LLVMString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", unsafe { CStr::from_ptr(self.buffer_ptr) })
    }
}
