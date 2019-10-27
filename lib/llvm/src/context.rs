use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDoubleTypeInContext,
    LLVMFloatTypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext, LLVMInt8TypeInContext,
    LLVMModuleCreateWithNameInContext, LLVMStructCreateNamed, LLVMStructSetBody,
    LLVMStructTypeInContext, LLVMVoidTypeInContext,
};

use llvm_sys::target::{LLVMIntPtrTypeForASInContext, LLVMIntPtrTypeInContext};

use llvm_sys::prelude::{LLVMContextRef, LLVMTypeRef};

use wasmo_utils::debug;

use crate::{
    types::{BasicType, FloatType, IntType, StructType, VoidType},
    AddressSpace, Builder, Module,
};

use crate::target::TargetData;

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

    pub fn i8_type(&self) -> IntType {
        let ty = unsafe { LLVMInt8TypeInContext(*self.context) };

        IntType::new(ty)
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

    pub fn void_type(&self) -> VoidType {
        let ty = unsafe { LLVMVoidTypeInContext(*self.context) };

        VoidType::new(ty)
    }

    pub fn struct_type(&self, types: &[BasicType], is_packed: bool) -> StructType {
        let ty = unsafe {
            LLVMStructTypeInContext(
                *self.context,
                types
                    .iter()
                    .map(|ty| ty.as_ref())
                    .collect::<Vec<LLVMTypeRef>>()
                    .as_mut_ptr(),
                types.len() as _,
                is_packed as _,
            )
        };

        StructType::new(ty)
    }

    pub fn struct_type_with_name(
        &self,
        struct_name: &str,
        types: &[BasicType],
        is_packed: bool,
    ) -> StructType {
        let c_string = CString::new(struct_name)
            .expect("Conversion of struct name string to c_string failed unexpectedly");

        let mut ty = unsafe { LLVMStructCreateNamed(*self.context, c_string.as_ptr()) };

        unsafe {
            LLVMStructSetBody(
                ty,
                types
                    .iter()
                    .map(|ty| ty.as_ref())
                    .collect::<Vec<LLVMTypeRef>>()
                    .as_mut_ptr(),
                types.len() as _,
                is_packed as _,
            )
        }

        StructType::new(ty)
    }

    pub fn machine_int_type(
        &self,
        target_data: &TargetData,
        address_space: Option<AddressSpace>,
    ) -> IntType {
        let ty = match address_space {
            Some(address_space) => unsafe {
                LLVMIntPtrTypeForASInContext(*self.context, target_data.data, address_space as _)
            },
            None => unsafe { LLVMIntPtrTypeInContext(*self.context, target_data.data) },
        };

        IntType::new(ty)
    }

    pub fn rc(&self) -> usize {
        Rc::strong_count(&(self.context))
    }
}

///
impl Drop for Context {
    fn drop(&mut self) {
        if Rc::strong_count(&self.context) == 1 {
            debug!("Context drop!");
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
