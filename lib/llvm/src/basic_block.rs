use llvm_sys::prelude::{LLVMBasicBlockRef, LLVMValueRef};

///
pub struct BasicBlock {
    pub(crate) basic_block: LLVMBasicBlockRef,
}

impl BasicBlock {
    pub(crate) fn new(basic_block: LLVMBasicBlockRef) -> Self {
        assert!(!basic_block.is_null());

        Self { basic_block }
    }
}
