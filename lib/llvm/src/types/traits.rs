use llvm_sys::prelude::LLVMTypeRef;

pub trait AsTypeRef {
    fn as_ref(&self) -> LLVMTypeRef;
}
