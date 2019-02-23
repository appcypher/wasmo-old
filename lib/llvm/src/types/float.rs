use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct F32Type {
    type_: Type,
}

impl F32Type {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}

///
pub struct F64Type {
    type_: Type,
}

impl F64Type {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
