use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct ArrayType {
    type_: Type
}

impl ArrayType {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
