use super::Value;

use llvm_sys::prelude::{LLVMValueRef};

///
pub struct FunctionValue {
    value: Value
}

impl FunctionValue {
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        Self { value: Value::new(value) }
    }
}
