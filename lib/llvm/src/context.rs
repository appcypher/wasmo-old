use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDoubleTypeInContext,
    LLVMFloatTypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext,
    LLVMModuleCreateWithNameInContext,
};

use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};

use wasmlite_utils::debug;

use crate::{
    types::{FloatType, IntType},
    Builder, Module,
};

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

    pub fn create_builder(&self) -> Builder {
        let builder = unsafe { LLVMCreateBuilderInContext(*self.context) };

        Builder::new(builder, Some(self))
    }

    pub fn i32_type(&self) -> IntType {
        let ty = unsafe { LLVMInt32TypeInContext(*self.context) };

        IntType::new(ty)
    }

    pub fn i64_type(&self) -> IntType {
        let ty = unsafe { LLVMInt64TypeInContext(*self.context) };

        IntType::new(ty)
    }

    pub fn f32_type(&self) -> FloatType {
        let ty = unsafe { LLVMFloatTypeInContext(*self.context) };

        FloatType::new(ty)
    }

    pub fn f64_type(&self) -> FloatType {
        let ty = unsafe { LLVMDoubleTypeInContext(*self.context) };

        FloatType::new(ty)
    }

    pub fn rc(&self) -> usize {
        Rc::strong_count(&(self.context))
    }
}

///
impl Drop for Context {
    fn drop(&mut self) {
        if Rc::strong_count(&self.context) == 1 {
            unsafe {
                LLVMContextDispose(*self.context);
            }
        }
        debug!(
            "Context drop! to rc({:?})",
            Rc::strong_count(&self.context) - 1
        );
    }
}
