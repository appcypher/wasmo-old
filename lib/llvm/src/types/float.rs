use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

use llvm_sys::core::LLVMConstReal;

use crate::values::FloatValue;

use crate::types::PointerType;

use crate::AddressSpace;

use super::AsTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FloatType {
    pub(crate) ty: Type,
}

impl FloatType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }

    pub fn const_float(&self, value: f64) -> FloatValue {
        unsafe { FloatValue::new(LLVMConstReal(self.ty.ty, value)) }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        self.ty.ptr_type(address_space)
    }

    pub fn zero(&self) -> FloatValue {
        self.const_float(0.0)
    }
}

impl AsTypeRef for FloatType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
