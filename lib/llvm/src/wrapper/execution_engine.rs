#![deprecated(since="0.0.1", note="wasmlite will transition to ORC APIs")]

use std::rc::Rc;

use std::mem::zeroed;

use std::ffi::CString;
use std::marker::PhantomData;

use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine, LLVMExecutionEngineRef,
    LLVMFindFunction, LLVMGetFunctionAddress, LLVMLinkInInterpreter, LLVMLinkInMCJIT,
};

use wasmlite_utils::debug;

use crate::{
    errors::FunctionLookUp, values::FunctionValue, CompilerError, CompilerResult, Context, Module,
};

// Takes ownership of whatever module was passed during the creation of LLVMExecutionEngineRef
// Even though the module is not represented in this structure, `LLVMExecutionEngineRef` indirectly
// holds a reference to it and it will free it when it is done.
pub struct ExecutionEngine {
    execution_engine: LLVMExecutionEngineRef,
    jit_mode: bool,
}

impl ExecutionEngine {
    pub(crate) fn new(execution_engine: LLVMExecutionEngineRef, jit_mode: bool) -> Self {
        assert!(!execution_engine.is_null());

        Self {
            execution_engine,
            jit_mode,
        }
    }

    /// This function needs not be called. It is here because it
    /// references LLVMLinkInMCJIT which prevents its DCE
    pub fn link_in_mc_jit() {
        unsafe { LLVMLinkInMCJIT() }
    }

    /// This function needs not be called. It is here because it
    /// references LLVMLinkInInterpreter which prevents its DCE
    pub fn link_in_interpreter() {
        unsafe {
            LLVMLinkInInterpreter();
        }
    }

    ///
    pub unsafe fn get_function<F>(&self, name: &str) -> CompilerResult<Func<F>> {
        if !self.jit_mode {
            return Err(CompilerError::FunctionLookUp(FunctionLookUp::JITNotEnabled));
        }

        let name = CString::new(name).expect("Conversion to CString failed");

        let address = unsafe { LLVMGetFunctionAddress(self.execution_engine, name.as_ptr()) };

        if address == 0 {
            return Err(CompilerError::FunctionLookUp(
                FunctionLookUp::FunctionNotFound,
            ));
        }

        Ok(Func::create(address))
    }

    /// REVIEW: Can also find nothing if target isn't initialized
    pub fn get_function_value(&self, fn_name: &str) -> CompilerResult<FunctionValue> {
        if !self.jit_mode {
            return Err(CompilerError::FunctionLookUp(FunctionLookUp::JITNotEnabled));
        }

        let fn_name = CString::new(fn_name).expect("Conversion to CString failed");

        let mut function = unsafe { zeroed() };

        let code =
            unsafe { LLVMFindFunction(self.execution_engine, fn_name.as_ptr(), &mut function) };

        if code == 1 {
            return Err(CompilerError::FunctionLookUp(
                FunctionLookUp::FunctionNotFound,
            ));
        };

        Ok(FunctionValue::new(function))
    }
}

///
impl Drop for ExecutionEngine {
    fn drop(&mut self) {
        debug!("ExecutionEngine drop!");
        unsafe {
            LLVMDisposeExecutionEngine(self.execution_engine);
        }
    }
}


/// Represents a jitted function that can be called.
pub struct Func<F> {
    address: F,
}

impl<F> Func<F> {
    pub unsafe fn create(address: u64) -> Func<F> {
        Func {
            address: std::mem::transmute_copy(&address)
        }
    }
}


// Support for 15 arguments for now.
recurse_vararg_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
