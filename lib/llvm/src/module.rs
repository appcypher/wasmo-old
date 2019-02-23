use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{
    LLVMDisposeModule, LLVMModuleCreateWithName, LLVMModuleCreateWithNameInContext,
};

use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};

use wasmlite_utils::debug;

use crate::{
    Context,
    types::FunctionType,
    values::FunctionValue,
};

///
/// TODO:IMPORTANT: Can the Rc be gotten rid of. Does EE own module? 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    module: Rc<LLVMModuleRef>,
    context_ref: Option<Context>,
}

impl Module {
    /// Shares context
    pub fn new(module: LLVMModuleRef, context: Option<&Context>) -> Self {
        assert!(!module.is_null());

        Self {
            module: Rc::new(module),
            context_ref: context.cloned(), // Increments Context.context ref count
        }
    }

    pub fn create(name: &str) -> Self {
        let name = CString::new(name).expect("CString conversion failed");

        let module = unsafe { LLVMModuleCreateWithName(name.as_ptr()) };

        Module::new(module, None)
    }

    /// Consumes type
    pub fn add_function(&self, function_name: &str, type_: FunctionType) -> FunctionValue {
        let name = CString::new(function_name).expect("CString conversion failed");

        unimplemented!()
    }
}

///
impl Drop for Module {
    fn drop(&mut self) {
        debug!(
            "Module drop attempt @ ref count = {:?}",
            Rc::strong_count(&self.module)
        );
        if Rc::strong_count(&self.module) == 1 {
            unsafe {
                LLVMDisposeModule(*self.module);
            }
        }
    }
}
