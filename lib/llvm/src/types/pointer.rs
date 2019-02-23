use super::Type;

use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct PointerType {
    type_: Type,
}

impl PointerType {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        assert!(!type_.is_null());

        Self { type_: Type::new(type_) }
    }
}
