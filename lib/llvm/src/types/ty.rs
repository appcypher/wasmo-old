use llvm_sys::prelude::LLVMTypeRef;

use super::AsTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Type {
    pub(crate) ty: LLVMTypeRef,
}

impl Type {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        Self { ty }
    }
}

impl AsTypeRef for Type {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty
    }
}
