use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::core::{LLVMGetElementType, LLVMPointerType};

use crate::types::BasicType;

use crate::types::PointerType;

use crate::AddressSpace;

use super::AsTypeRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Type {
    pub(crate) ty: LLVMTypeRef,
}

impl Type {
    pub(crate) fn new(ty: LLVMTypeRef) -> Self {
        Self { ty }
    }

    pub fn get_element_type(&self) -> BasicType {
        let ptr = unsafe {
            LLVMGetElementType(self.ty)
        };

        BasicType::new(ptr)
    }

    pub fn ptr_type(&self, address_space: &AddressSpace) -> PointerType {
        unsafe { PointerType::new(LLVMPointerType(self.ty, *address_space as _)) }
    }
}

impl AsTypeRef for Type {
    fn as_ref(&self) -> LLVMTypeRef {
        self.ty
    }
}
