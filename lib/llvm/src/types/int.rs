use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

use llvm_sys::core::LLVMConstInt;

use crate::values::IntValue;

use crate::types::PointerType;

use crate::AddressSpace;

use super::AsTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntType {
    pub(crate) ty: Type,
}

impl IntType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }

    pub fn const_int(&self, value: u64, sign_extend: bool) -> IntValue {
        unsafe { IntValue::new(LLVMConstInt(self.ty.ty, value, sign_extend as _)) }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        self.ty.ptr_type(address_space)
    }

    pub fn zero(&self, sign_extend: bool) -> IntValue {
        self.const_int(0, sign_extend)
    }
}

impl AsTypeRef for IntType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
