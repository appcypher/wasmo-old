#[macro_use]
use llvm_sys::prelude::LLVMValueRef;

use llvm_sys::LLVMTypeKind;

use llvm_sys::core::{LLVMGetTypeKind, LLVMTypeOf};

use super::{
    ArrayValue, AsValueRef, FloatValue, FunctionValue, IntValue, PointerValue, StructValue,
    VectorValue,
};

enum_impl_def! {
    BasicValue (get_type_first: true, field: val, ref: LLVMValueRef) {
        LLVMHalfTypeKind |
        LLVMFloatTypeKind |
        LLVMDoubleTypeKind |
        LLVMX86_FP80TypeKind |
        LLVMFP128TypeKind |
        LLVMPPC_FP128TypeKind => FloatValue,
        LLVMIntegerTypeKind => IntValue,
        LLVMFunctionTypeKind => FunctionValue,
        LLVMArrayTypeKind => ArrayValue,
        LLVMVectorTypeKind => VectorValue,
        LLVMPointerTypeKind => PointerValue,
        LLVMStructTypeKind => StructValue
    }
}

