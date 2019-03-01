#[macro_use]
use llvm_sys::prelude::LLVMTypeRef;

use llvm_sys::LLVMTypeKind;

use llvm_sys::core::LLVMGetTypeKind;

use super::{
    ArrayType, AsTypeRef, FloatType, FunctionType, IntType, PointerType, StructType, VectorType,
};

enum_impl_def! {
    BasicType (get_type_first: false, field: ty, ref: LLVMTypeRef) {
        LLVMHalfTypeKind |
        LLVMFloatTypeKind |
        LLVMDoubleTypeKind |
        LLVMX86_FP80TypeKind |
        LLVMFP128TypeKind |
        LLVMPPC_FP128TypeKind => FloatType,
        LLVMIntegerTypeKind => IntType,
        LLVMFunctionTypeKind => FunctionType,
        LLVMArrayTypeKind => ArrayType,
        LLVMVectorTypeKind => VectorType,
        LLVMPointerTypeKind => PointerType,
        LLVMStructTypeKind => StructType
    }
}
