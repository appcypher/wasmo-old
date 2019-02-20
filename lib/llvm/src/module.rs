use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{
    LLVMDisposeModule, LLVMModuleCreateWithName, LLVMModuleCreateWithNameInContext,
};

use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};

use wasmlite_utils::*;

use crate::context::Context;

///
#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub(crate) module: Rc<LLVMModuleRef>,
    context_ref: Option<Context>,
}

///
impl Module {
    pub fn new(module: LLVMModuleRef, context: Option<&Context>) -> Self {
        assert!(!module.is_null());

        Self {
            module: Rc::new(module),
            context_ref: context.cloned(), // Increments Context.context ref count
        }
    }

    pub fn create(name: &str) -> Module {
        let name = CString::new(name).expect("Conversion to CString failed");

        let module = unsafe { LLVMModuleCreateWithName(name.as_ptr()) };

        Module::new(module, None)
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
