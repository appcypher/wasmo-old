use llvm_sys::prelude::LLVMValueRef;

use super::{FloatValue, IntValue, PointerValue, VectorValue};

pub trait AsValueRef {
    fn as_ref(&self) -> LLVMValueRef;
}

pub trait IntMathValue: AsValueRef {
    fn new(value: LLVMValueRef) -> Self;
}

pub trait FloatMathValue: AsValueRef {
    fn new(value: LLVMValueRef) -> Self;
}

pub trait PointerMathValue: AsValueRef {
    fn new(value: LLVMValueRef) -> Self;
}

impl IntMathValue for IntValue {
    fn new(value: LLVMValueRef) -> Self {
        IntValue::new(value)
    }
}

impl IntMathValue for VectorValue {
    fn new(value: LLVMValueRef) -> Self {
        VectorValue::new(value)
    }
}

impl FloatMathValue for IntValue {
    fn new(value: LLVMValueRef) -> Self {
        IntValue::new(value)
    }
}

impl FloatMathValue for VectorValue {
    fn new(value: LLVMValueRef) -> Self {
        VectorValue::new(value)
    }
}

impl PointerMathValue for IntValue {
    fn new(value: LLVMValueRef) -> Self {
        IntValue::new(value)
    }
}

impl PointerMathValue for VectorValue {
    fn new(value: LLVMValueRef) -> Self {
        VectorValue::new(value)
    }
}
