use wasmlite_utils::debug;

use wasmlite_parser::ir;

use crate::{
    Module,
    Context,
    Builder,
    execution_engine,

};

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

pub fn generate_module(_wasm_module: &ir::Module) -> () {
    let context = Context::create();

    let module = context.create_module("Hello LLVM");

    let builder = context.create_builder();

    let f64_type = context.f64_type();

    ()
}


// pub fn jit_compile_sum(context: &Context, module: &Module, builder: Builder) -> JITFunction<SumFunc> {}
