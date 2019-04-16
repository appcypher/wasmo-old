use super::Type;

use super::AsTypeRef;

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
}

impl AsTypeRef for PointerType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
