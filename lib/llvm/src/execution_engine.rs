use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::execution_engine::{LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMExecutionEngineRef};

use wasmlite_utils::debug;

use crate::{Context, Module};

/// An ExecutionEngine is JIT compiler that is used to generate code for an LLVM module.
pub struct ExecutionEngine {
    builder: LLVMExecutionEngineRef,
    module_ref: Option<Module>,
}

impl ExecutionEngine {
    /// Shares module
    pub fn new(builder: LLVMExecutionEngineRef, module: Option<&Module>) -> Self {
        assert!(!builder.is_null());

        Self {
            builder: builder,
            module_ref: module.cloned(), // Increments Module.module ref count
        }
    }
}

///
impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.builder);
        }
    }
}
