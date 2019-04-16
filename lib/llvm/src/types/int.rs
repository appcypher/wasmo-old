use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

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
}

impl AsTypeRef for IntType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
