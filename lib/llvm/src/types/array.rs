use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

use crate::types::PointerType;

use crate::AddressSpace;

use super::AsTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ArrayType {
    pub(crate) ty: Type,
}

impl ArrayType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        self.ty.ptr_type(address_space)
    }
}

impl AsTypeRef for ArrayType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
