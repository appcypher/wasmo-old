pub(crate) mod macros;
mod module;
mod instance;
mod compilation;
mod instantiation;
mod descriptions;
mod exports;
mod pointers;
mod imports;
mod func;

pub use module::Module;
pub use instance::Instance;
pub use descriptions::{TableDesc, MemoryDesc, GlobalDesc, FuncDesc};
pub use compilation::compile;
pub use instantiation::instantiate;
pub use pointers::{TablePtr, MemoryPtr, GlobalPtr, FuncPtr};
pub use exports::{Exports};
pub use func::Func;
