use super::{AsTypeRef, Type};

use llvm_sys::prelude::LLVMTypeRef;

use crate::types::PointerType;

use crate::AddressSpace;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VectorType {
    pub(crate) ty: Type,
}

impl VectorType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        self.ty.ptr_type(address_space)
    }
}

impl AsTypeRef for VectorType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
