use super::Type;

use super::AsTypeRef;

use crate::types::BasicType;

use crate::AddressSpace;

use llvm_sys::prelude::LLVMTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PointerType {
    pub(crate) ty: Type,
}

impl PointerType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> Self {
        self.ty.ptr_type(address_space)
    }
}

impl AsTypeRef for PointerType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
