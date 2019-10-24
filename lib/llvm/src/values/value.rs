use llvm_sys::prelude::{LLVMValueRef, LLVMTypeRef};
use llvm_sys::core::LLVMTypeOf;
use crate::types::BasicType;

use super::AsValueRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Value {
    pub(crate) val: LLVMValueRef,
}

impl Value {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        Self { val: value }
    }

    pub fn get_type(&self) -> BasicType {
        let ty = unsafe {
            LLVMTypeOf(self.val)
        };

        BasicType::new(ty)
    }
}

impl AsValueRef for Value {
    fn as_ref(&self) -> LLVMValueRef {
        self.val
    }
}
