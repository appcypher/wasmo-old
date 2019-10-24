use super::{AsTypeRef, BasicType, Type};

use llvm_sys::core::{LLVMCountParamTypes, LLVMFunctionType, LLVMGetParamTypes, LLVMGetReturnType};
use llvm_sys::prelude::LLVMTypeRef;

use crate::types::PointerType;

use crate::AddressSpace;

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

    pub fn count_param_types(&self) -> u32 {
        unsafe { LLVMCountParamTypes(self.ty.ty) }
    }

    pub fn get_param_types(&self) -> Vec<BasicType> {
        let count = self.count_param_types();
        let mut raw_vec: Vec<LLVMTypeRef> = Vec::with_capacity(count as usize);
        let ptr = raw_vec.as_mut_ptr();

        let new_vec = unsafe {
            LLVMGetParamTypes(self.ty.ty, ptr);
            std::slice::from_raw_parts(ptr, count as usize).to_vec()
        };

        new_vec.iter().map(|ty| BasicType::new(*ty)).collect()
    }

    pub fn get_return_type(&self) -> BasicType {
        unsafe {
            BasicType::new(LLVMGetReturnType(self.ty.ty))
        }
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        self.ty.ptr_type(address_space)
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
