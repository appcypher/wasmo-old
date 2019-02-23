use llvm_sys::prelude::{LLVMValueRef};

///
pub struct Value {
    value: LLVMValueRef
}

impl Value {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        Self { value }
    }
}
