use crate::descriptions::ModuleDesc;
use crate::func::Func;
use crate::types::{FuncPtr, GlobalPtr, MemoryPtr, TablePtr};
use wasmo_llvm::Module as LLVMModule;

use std::marker::PhantomData;

/// GenericModule<T> is a type that can either be a Module or an Instance because they have a similar structure,
/// and some similar functions. They also have a few specialized functions
#[repr(C)]
pub struct GenericModule<T> {
    ctx: Option<InstanceContext>,
    desc: ModuleDesc,
    ir: Option<LLVMModule>,
    is_instance: bool,
    phantom: PhantomData<T>,
}

#[repr(C)]
pub struct InstanceContext {
    tables: *mut TablePtr,
    memories: *mut MemoryPtr,
    globals: *mut GlobalPtr,
    functions: *mut FuncPtr,
}

/// GenericModuleType is a trait describing the different types of Module.
/// e.g. ModuleType and InstanceType
pub trait GenericModuleType {}

pub struct ModuleType {}
pub struct InstanceType {}

impl GenericModuleType for ModuleType {}
impl GenericModuleType for InstanceType {}

// Aliases
pub type Module = GenericModule<ModuleType>;
pub type Instance = GenericModule<InstanceType>;

/// Implementation for all GenericModule<T>'s where T is a ModType
impl<T: GenericModuleType> GenericModule<T> {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn get_func<'a>() -> Func<'a> {
        unimplemented!()
    }
}

/// Implementation for GenericModule<ModuleType>
impl GenericModule<ModuleType> {
    pub fn instantiate() -> Module {
        unimplemented!()
    }
}

/// Implementation for GenericModule<InstanceType>
impl GenericModule<InstanceType> {
    // TODO?
}
