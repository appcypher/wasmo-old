use llvm_sys::prelude::{LLVMTypeRef};

///
pub struct Type {
    type_: LLVMTypeRef,
}

impl Type {
    pub(crate) fn new(type_: LLVMTypeRef) -> Self {
        Self { type_ }
    }
}
