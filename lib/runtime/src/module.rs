use wasmlite_llvm as llvm;
use wasmlite_parser::ir;

use crate::ModuleDesc;

///
/// Compilation modes:
/// - Normal - compiles module-local code section
pub struct Module {
    module: Option<llvm::Module>,
    exports: Exports,
    descs: ModuleDesc, // Or Details?
}

impl Module {
    ///
    pub fn compile(wasm: &[u8]) -> Self { unimplemented!() }

    ///
    pub fn compile_ir(wasm: &ir::Module) -> Self { unimplemented!() }
}
