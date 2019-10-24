use crate::data::ModuleData;
use crate::func::Func;
use crate::types::{FuncPtr, GlobalPtr, MemoryPtr, TablePtr};
use wasmo_llvm::Module as LLVMModule;

use std::sync::{Arc, RwLock};
use std::marker::PhantomData;

#[repr(C)]
#[derive(Debug)]
pub struct InstanceContext {
    memories: *mut MemoryPtr,
    tables: *mut TablePtr,
    globals: *mut GlobalPtr,
    functions: *mut FuncPtr,
}

/// GenericModule<T> is a type that can be of any module type because the different module types
/// all have a similar structure and similar functions. Module types can also have a unique functions.
///
/// NOTE:
///
///     `ctx`'s InstanceContext is not optional (i.e. Option<InstanceContext>) here, even though it is not
///     needed at codegen phase, because we may need a stable way of determining the offsets of
///     `ctx` and `data` in the Module<T>
///
///     `ir` is optional because it is not always needed. JIT Eager and AOT discards it after instantiation.
///     `ir` is Arc because LLVMModule is shared between instances and since they can read/write to it RwLock is
///     required. Instances live on seperate execution threads BTW.
#[repr(C)]
#[derive(Debug)]
pub struct GenericModule<T> {
    ctx: InstanceContext,
    data: ModuleData,
    ir: Option<Arc<RwLock<LLVMModule>>>,
    phantom: PhantomData<T>,
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

///
impl InstanceContext {
    fn new() -> Self {
        let null: *const () = std::ptr::null();

        Self {
            memories: null as _,
            tables: null as _,
            globals: null as _,
            functions: null as _,
        }
    }
}

/// Implementation for all GenericModule<T>'s where T is a ModType
impl<T: GenericModuleType> GenericModule<T> {
    pub fn new() -> Self {
        // TODO
        Self {
            ctx: InstanceContext::new(),
            data: ModuleData::new(),
            ir: None,
            phantom: PhantomData,
        }
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

