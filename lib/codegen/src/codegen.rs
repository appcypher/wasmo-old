use wasmlite_parser::ir;
use crate::Module;

pub fn generate_module(wasm_module: &ir::Module) -> Module {
    let sections = &wasm_module.sections;

    Module::create("hello")
}

// TODO: To be moved to their respective folders

// struct
