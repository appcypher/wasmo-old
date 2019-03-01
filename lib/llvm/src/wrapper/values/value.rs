use llvm_sys::prelude::LLVMValueRef;

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
}

impl AsValueRef for Value {
    fn as_ref(&self) -> LLVMValueRef {
        self.val
    }
}
