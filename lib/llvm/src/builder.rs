use std::rc::Rc;

use std::ffi::CString;

use llvm_sys::prelude::LLVMBuilderRef;

use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildFAdd, LLVMBuildRet, LLVMBuildRetVoid,
    LLVMCreateBuilder, LLVMDisposeBuilder, LLVMPositionBuilder, LLVMPositionBuilderAtEnd,
    LLVMPositionBuilderBefore, LLVMBuildFSub, LLVMBuildSub, LLVMBuildFMul, LLVMBuildMul
};

use wasmo_utils::debug;

use crate::{BasicBlock, Context};

use crate::values::{
    AsValueRef, BasicValue, FloatMathValue, InstructionValue, IntMathValue, PointerMathValue,
};

///
#[derive(Debug)]
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
        let name = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildAdd(self.builder, lhs.as_ref(), rhs.as_ref(), name.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_float_add<T: FloatMathValue>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_string = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildFAdd(self.builder, lhs.as_ref(), rhs.as_ref(), c_string.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_int_sub<T: IntMathValue>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_string = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildSub(self.builder, lhs.as_ref(), rhs.as_ref(), c_string.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_float_sub<T: FloatMathValue>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_string = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildFSub(self.builder, lhs.as_ref(), rhs.as_ref(), c_string.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_int_mul<T: IntMathValue>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_string = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildMul(self.builder, lhs.as_ref(), rhs.as_ref(), c_string.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_float_mul<T: FloatMathValue>(&self, lhs: T, rhs: T, name: &str) -> T {
        let c_string = CString::new(name).expect("Conversion of name string to cstring failed");

        let value =
            unsafe { LLVMBuildFMul(self.builder, lhs.as_ref(), rhs.as_ref(), c_string.as_ptr()) };

        T::new(value)
    }

    ///
    pub fn build_return(&self, value: Option<BasicValue>) -> InstructionValue {
        let value = unsafe {
            match value {
                Some(val) => LLVMBuildRet(self.builder, val.as_ref()),
                None => LLVMBuildRetVoid(self.builder),
            }
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
