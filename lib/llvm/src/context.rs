
use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{LLVMContextCreate, LLVMContextDispose, LLVMModuleCreateWithNameInContext, LLVMCreateBuilderInContext,
LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMFloatTypeInContext, LLVMDoubleTypeInContext};

use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};

use wasmlite_utils::debug;

use crate::{
    Module,
    Builder,
    types::{I32Type, I64Type, F32Type, F64Type},
};

///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    context: Rc<LLVMContextRef>,
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

    pub(crate) fn as_ptr(&self) -> LLVMContextRef {
        *self.context
    }

    pub fn create_module(&self, name: &str) -> Module {
        let name = CString::new(name).expect("Conversion to CString failed");

        let module = unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), *self.context) };

        Module::new(module, Some(self))
    }

    pub fn create_builder(&self) -> Builder {
        let builder = unsafe { LLVMCreateBuilderInContext(*self.context) };

        Builder::new(builder, Some(self))
    }

    pub fn i32_type(&self) -> I32Type {
        let type_ = unsafe { LLVMInt32TypeInContext(*self.context) };

        I32Type::new(type_)
    }

    pub fn i64_type(&self) -> I64Type {
        let type_ = unsafe { LLVMInt64TypeInContext(*self.context) };

        I64Type::new(type_)
    }

    pub fn f32_type(&self) -> F32Type {
        let type_ = unsafe { LLVMFloatTypeInContext(*self.context) };

        F32Type::new(type_)
    }

    pub fn f64_type(&self) -> F64Type {
        let type_ = unsafe { LLVMDoubleTypeInContext(*self.context) };

        F64Type::new(type_)
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
