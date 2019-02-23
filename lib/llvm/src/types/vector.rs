use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct VectorType {
    type_: Type,
}

impl VectorType {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
