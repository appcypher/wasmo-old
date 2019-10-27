use crate::data::ModuleData;
use crate::func::Func;
use wasmo_llvm::Module as LLVMModule;
use crate::context::InstanceContext;
use crate::options::Options;

use std::sync::{Arc, RwLock};
use std::marker::PhantomData;

/// Container<T> is a type that can either be Instance or Module. Container<T> represents a shared structural
/// representation between these two types. With traits and generics, a specific type of container,
/// e.g. Container<Instance<AOT>>, can have it's own specific implementation.
///
/// #### NOTE
/// `context`'s InstanceContext is not optional (i.e. Option<InstanceContext>) here, even though it is not
/// needed at codegen phase, because we may need a stable way of determining the offsets of `ctx` and `data`
/// in the Module<T>.
///
/// `module` is optional because it is not always needed. JIT Eager and AOT discards it after instantiation.
/// `module` is Arc because LLVMModule is shared between instances and since they can read/write to it RwLock is
/// required. Instances live on seperate execution threads BTW.
#[repr(C)]
#[derive(Debug)]
pub struct Container<T> {
    context: InstanceContext,
    data: ModuleData,
    module: Option<Arc<RwLock<LLVMModule>>>,
    phantom: PhantomData<T>,
}

/// Implementation for all Container<T>'s where T is a ContainerType
impl<T: ContainerType> Container<T> {
    pub fn new() -> Self {
        Self {
            context: InstanceContext::new(),
            data: ModuleData::new(),
            module: None,
            phantom: PhantomData,
        }
    }

    pub fn from_llvm_module(module: LLVMModule) -> Self {
        Self {
            context: InstanceContext::new(),
            data: ModuleData::new(),
            module: Some(Arc::new(RwLock::new(module))),
            phantom: PhantomData,
        }
    }
}


impl Container<Module<AOT>> {
    pub fn instantiate() -> Module<AOT> {
        unimplemented!()
    }
}

impl Container<Instance<AOT>> {
    pub fn instantiate() -> Module<AOT> {
        unimplemented!()
    }
}

impl Container<Module<JITEager>> {
    pub fn create_aot_with_llvm_module(module: LLVMModule, options: &Options) -> Container<Module<AOT>> {
        Container::from_llvm_module(module)
    }

    pub fn instantiate() -> Module<AOT> {
        unimplemented!()
    }
}

impl Container<Instance<JITEager>> {
    pub fn get_func<'a>() -> Func<'a> {
        unimplemented!()
    }
}

// Traits
pub trait ContainerType {}
pub trait CompileType {}

#[derive(Debug)]
pub struct AOT();
#[derive(Debug)]
pub struct JITEager();

impl CompileType for AOT {}
impl CompileType for JITEager {}

#[derive(Debug)]
pub struct Module<T>(PhantomData<T>);
#[derive(Debug)]
pub struct Instance<T>(PhantomData<T>);

impl<T: CompileType> ContainerType for Module<T> {}
impl<T: CompileType> ContainerType for Instance<T> {}


/// This module is the public interface of this file.
pub mod module {
    pub use super::{JITEager, AOT, Container};

    pub type ModuleAOT = Container<super::Module<AOT>>;
    pub type InstanceAOT = Container<super::Instance<AOT>>;
    pub type Module = Container<super::Module<JITEager>>;
    pub type Instance = Container<super::Instance<JITEager>>;
}
