use super::Type;

use llvm_sys::prelude::LLVMTypeRef;

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
}

impl AsTypeRef for ArrayType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}
