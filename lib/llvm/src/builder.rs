use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::prelude::{LLVMBuilderRef};

use llvm_sys::core::{LLVMDisposeBuilder, LLVMCreateBuilder};

use wasmlite_utils::debug;

use crate::context::Context;

///
pub struct Builder {
    builder: LLVMBuilderRef,
    context_ref: Option<Context>,
}

///
impl Builder {
    /// Shares context
    pub fn new(builder: LLVMBuilderRef, context: Option<&Context>) -> Self {
        assert!(!builder.is_null());

        Self {
            builder: builder,
            context_ref: context.cloned(), // Increments Context.context ref count
        }
    }

    pub fn create() -> Self {
        let builder = unsafe { LLVMCreateBuilder() };

        Builder::new(builder, None)
    }
}

///
impl Drop for Builder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}
