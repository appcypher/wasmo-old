use super::Value;

use llvm_sys::prelude::LLVMValueRef;

use super::AsValueRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntValue {
    pub(crate) val: Value,
}

impl IntValue {
    pub(crate) fn new(val: LLVMValueRef) -> Self {
        assert!(!val.is_null());

        Self {
            val: Value::new(val),
        }
    }
}

impl AsValueRef for IntValue {
    fn as_ref(&self) -> LLVMValueRef {
        self.val.val
    }
}
