use super::Value;

use llvm_sys::core::{
    LLVMAppendBasicBlockInContext, LLVMCountParams, LLVMGetFirstParam, LLVMGetLastParam,
    LLVMGetParam, LLVMSetLinkage,
};
use llvm_sys::prelude::LLVMValueRef;

use crate::{BasicBlock, Context, Linkage};

use super::{
    super::{errors::GetValue, CompilerError, CompilerResult},
    AsValueRef, BasicValue,
};

use std::ffi::CString;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FunctionValue {
    pub(crate) val: Value,
}

impl FunctionValue {
    ///
    pub(crate) fn new(value: LLVMValueRef) -> Self {
        assert!(!value.is_null());

        Self {
            val: Value::new(value),
        }
    }

    ///
    pub fn set_linkage(&self, linkage: Linkage) {
        unsafe {
            LLVMSetLinkage(self.val.val, linkage.into());
        }
    }

    ///
    pub fn count_params(&self) -> u32 {
        unsafe { LLVMCountParams(self.val.val) }
    }

    ///
    pub fn get_nth_param(&self, nth: u32) -> CompilerResult<BasicValue> {
        let count = self.count_params();

        if count <= nth {
            return Err(CompilerError::GetValue(GetValue::CantGetNthParam(nth)));
        }

        let param = unsafe { LLVMGetParam(self.val.val, nth) };

        Ok(BasicValue::new(param))
    }

    ///
    pub fn get_first_param(&self) -> CompilerResult<BasicValue> {
        let param = unsafe { LLVMGetFirstParam(self.val.val) };

        if param.is_null() {
            return Err(CompilerError::GetValue(GetValue::CantGetFirstParam));
        }

        Ok(BasicValue::new(param))
    }

    ///
    pub fn get_last_param(&self) -> CompilerResult<BasicValue> {
        let param = unsafe { LLVMGetLastParam(self.val.val) };

        if param.is_null() {
            return Err(CompilerError::GetValue(GetValue::CantGetLastParam));
        }

        Ok(BasicValue::new(param))
    }

    ///
    pub fn append_basic_block(&self, name: &str, context: &Context) -> BasicBlock {
        let name = CString::new(name).expect("Conversion to CString failed");

        let basic_block =
            unsafe { LLVMAppendBasicBlockInContext(*context.context, self.val.val, name.as_ptr()) };

        BasicBlock::new(basic_block)
    }
}

impl AsValueRef for FunctionValue {
    fn as_ref(&self) -> LLVMValueRef {
        self.val.val
    }
}
