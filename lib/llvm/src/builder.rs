use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::prelude::LLVMBuilderRef;

use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildRet, LLVMBuildRetVoid, LLVMCreateBuilder, LLVMDisposeBuilder,
    LLVMPositionBuilder, LLVMPositionBuilderAtEnd, LLVMPositionBuilderBefore,
};

use wasmo_utils::debug;

use crate::{BasicBlock, Context};

use crate::values::{
    AsValueRef, BasicValue, FloatMathValue, InstructionValue, IntMathValue, PointerMathValue,
};

///
pub struct Builder {
    builder: LLVMBuilderRef,
    context_ref: Option<Context>,
}

///
impl Builder {
    /// Shares context
    pub fn new(builder: LLVMBuilderRef, context: Option<&Context>) -> Self {
        assert!(!builder.is_null());

        Self {
            builder: builder,
            context_ref: context.cloned(), // Increments Context.context ref count
        }
    }

    ///
    pub fn create() -> Self {
        let builder = unsafe { LLVMCreateBuilder() };

        Builder::new(builder, None)
    }

    ///
    pub fn position_at(&self, basic_block: &BasicBlock, instruction: &InstructionValue) {
        unsafe {
            LLVMPositionBuilder(self.builder, basic_block.basic_block, instruction.as_ref());
        }
    }

    ///
    pub fn position_before(&self, instruction: &InstructionValue) {
        unsafe { LLVMPositionBuilderBefore(self.builder, instruction.as_ref()) }
    }

    ///
    pub fn position_at_end(&self, basic_block: &BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, basic_block.basic_block);
        }
    }

    ///
    pub fn build_int_add<T: IntMathValue>(&self, rhs: T, lhs: T, name: &str) -> T {
        let name = CString::new(name).expect("Conversion to CString failed");

        let value =
            unsafe { LLVMBuildAdd(self.builder, lhs.as_ref(), rhs.as_ref(), name.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_return(&self, value: Option<impl Into<BasicValue>>) -> InstructionValue {
        let value: Option<BasicValue> = value.map(|x| x.into());

        let value = unsafe {
            value.map_or_else(
                || LLVMBuildRetVoid(self.builder),
                |value| LLVMBuildRet(self.builder, value.as_ref()),
            )
        };

        InstructionValue::new(value)
    }
}

///
impl Drop for Builder {
    fn drop(&mut self) {
        debug!("Builder drop!");
        unsafe {
            LLVMDisposeBuilder(self.builder);
        }
    }
}
