use wasmlite_utils::*;

use wasmlite_parser::ir;

use crate::module::Module;

use crate::context::Context;

pub fn generate_module(_wasm_module: &ir::Module) -> () {
    let context = Context::create();

    debug!("context = {:#?}\n", context);

    let module = context.create_module("Hello LLVM");

    debug!("module = {:#?}\n", module);

    ()
}
