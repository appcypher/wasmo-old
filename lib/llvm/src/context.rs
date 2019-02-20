use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{LLVMContextCreate, LLVMContextDispose, LLVMModuleCreateWithNameInContext};

use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef};

use wasmlite_utils::*;

use crate::module::Module;

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    pub(crate) context: Rc<LLVMContextRef>,
}

///
impl Context {
    pub fn create() -> Self {
        let context = unsafe { LLVMContextCreate() };

        assert!(!context.is_null());

        Self {
            context: Rc::new(context),
        }
    }

    pub fn create_module(&self, name: &str) -> Module {
        let name = CString::new(name).expect("Conversion to CString failed");

        let module = unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), *self.context) };

        Module::new(module, Some(self))
    }

    pub(crate) fn as_ptr(&self) -> LLVMContextRef {
        *self.context
    }

    pub fn create_builder() -> () {
        ()
    }

    pub fn type_i32() -> () {
        ()
    }

    pub fn type_i64() -> () {
        ()
    }

    pub fn type_f32() -> () {
        ()
    }

    pub fn type_f64() -> () {
        ()
    }
}

///
impl Drop for Context {
    fn drop(&mut self) {
        debug!(
            "Context drop attempt @ ref count = {:?}",
            Rc::strong_count(&self.context)
        );
        if Rc::strong_count(&self.context) == 1 {
            unsafe {
                LLVMContextDispose(*self.context);
            }
        }
    }
}
