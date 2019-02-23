use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct StructType {
    type_: Type,
}

impl StructType {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
