use std::mem::zeroed;

use std::cell::RefCell;

use std::ffi::CString;

use llvm_sys::core::{LLVMAddFunction, LLVMDisposeModule, LLVMModuleCreateWithName};

use llvm_sys::execution_engine::{
    LLVMCreateExecutionEngineForModule, LLVMCreateInterpreterForModule,
    LLVMCreateJITCompilerForModule, LLVMExecutionEngineRef,
};

use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};

use wasmlite_utils::debug;

use super::{
    string::to_rust_string,
    types::{AsTypeRef, FunctionType},
    values::FunctionValue,
    CompilerError, CompilerResult, Context, ExecutionEngine, InitializationConfig, Linkage,
    OptimizationLevel, Target,
};

///
/// TODO:IMPORTANT: Can the Rc be gotten rid of. Does EE own module?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub(crate) module: LLVMModuleRef,
    context_ref: Option<Context>,
    owned: RefCell<bool>, // RefCell is a workaround for mutating `owned` field without passing module mutably during execution engine creation.
}

impl Module {
    /// Shares context
    pub fn new(module: LLVMModuleRef, context: Option<&Context>) -> Self {
        assert!(!module.is_null());

        Self {
            module: module,
            context_ref: context.cloned(), // Increments Context.context ref count
            owned: RefCell::new(false),
        }
    }

    ///
    pub(crate) fn is_owned(&self) -> bool {
        *self.owned.borrow_mut()
    }

    ///
    pub fn create(name: &str) -> Self {
        let name = CString::new(name).expect("CString conversion failed");

        let module = unsafe { LLVMModuleCreateWithName(name.as_ptr()) };

        Module::new(module, None)
    }

    ///
    pub fn create_interpreter_execution_engine(&self) -> CompilerResult<ExecutionEngine> {
        Target::initialize_native(&InitializationConfig::default())?;

        // Check if module is owned by another execution engine.
        if self.is_owned() {
            return Err(CompilerError::ExecutionEngine(
                "Module is owned by another Execution Engine".into(),
            ));
        }

        // Set module as owned so it will be disposed by this execution engine.
        *self.owned.borrow_mut() = true;

        let mut execution_engine = unsafe { zeroed() };
        let mut error_string = unsafe { zeroed() };

        let code = unsafe {
            LLVMCreateInterpreterForModule(
                &mut execution_engine,
                self.module, // Takes ownership of module
                &mut error_string,
            )
        };

        if code == 1 {
            return Err(CompilerError::ExecutionEngine(unsafe {
                to_rust_string(error_string)
            }));
        }

        Ok(ExecutionEngine::new(execution_engine, false))
    }

    ///
    pub fn create_jit_execution_engine(
        &self,
        opt_level: OptimizationLevel,
    ) -> CompilerResult<ExecutionEngine> {
        Target::initialize_native(&InitializationConfig::default())?;

        // Check if module is owned by another execution engine.
        if self.is_owned() {
            return Err(CompilerError::ExecutionEngine(
                "Module is owned by another Execution Engine".into(),
            ));
        }

        // Set module as owned so it will be disposed by this execution engine.
        *self.owned.borrow_mut() = true;

        let mut execution_engine = unsafe { zeroed() };
        let mut error_string = unsafe { zeroed() };

        let code = unsafe {
            LLVMCreateJITCompilerForModule(
                &mut execution_engine,
                self.module, // Takes ownership of module
                opt_level as u32,
                &mut error_string,
            )
        };

        if code == 1 {
            return Err(CompilerError::ExecutionEngine(unsafe {
                to_rust_string(error_string)
            }));
        }

        Ok(ExecutionEngine::new(execution_engine, true))
    }

    /// Consumes type
    pub fn add_function(
        &self,
        function_name: &str,
        func_type: FunctionType,
        linkage: Option<Linkage>,
    ) -> FunctionValue {
        let name = CString::new(function_name).expect("CString conversion failed");

        let value = unsafe { LLVMAddFunction(self.module, name.as_ptr(), func_type.as_ref()) };

        let fn_value = FunctionValue::new(value);

        if let Some(linkage) = linkage {
            fn_value.set_linkage(linkage)
        }

        fn_value
    }
}

///
impl Drop for Module {
    fn drop(&mut self) {
        debug!("Module drop!");
        // Checks if module is owned by a in which case, it will be automatically deallocated
        // when the ExcecutionEngine is dropped.
        if !*self.owned.borrow_mut() {
            unsafe {
                LLVMDisposeModule(self.module);
            }
        }
    }
}
