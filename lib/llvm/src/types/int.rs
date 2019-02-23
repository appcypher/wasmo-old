use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct I32Type {
    type_: Type,
}

impl I32Type {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}

///
pub struct I64Type {
    type_: Type,
}

impl I64Type {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
