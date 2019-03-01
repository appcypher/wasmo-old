use super::{AsTypeRef, BasicType, Type};

use llvm_sys::core::LLVMFunctionType;
use llvm_sys::prelude::LLVMTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FunctionType {
    pub(crate) ty: Type,
}

impl FunctionType {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        assert!(!ty.is_null());

        Self { ty: Type::new(ty) }
    }
}

impl AsTypeRef for FunctionType {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty.ty
    }
}

pub fn fn_type(
    param_types: &[BasicType],
    return_type: BasicType,
    is_varargs: bool,
) -> FunctionType {
    let mut param_types: Vec<LLVMTypeRef> = param_types.iter().map(|ty| ty.as_ref()).collect();
    let return_type = return_type.as_ref();
    let ty = unsafe {
        LLVMFunctionType(
            return_type,
            param_types.as_mut_ptr(),
            param_types.len() as _,
            is_varargs as _,
        )
    };
    FunctionType::new(ty)
}
