use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

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
}

impl AsTypeRef for FloatType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
