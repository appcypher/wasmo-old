
use wasmlite_parser::ir;

use wasmlite_llvm::codegen;

use wasmlite_llvm::{
    types::{fn_type, BasicType},
    values::IntValue,
    Builder, Context, Module,
};


fn main() {
    println!("\n=== [ llvm_example ] ===\n");

    let empty_wasm_module = ir::Module { sections: vec![] };

    // TODO

    println!("\n=== [ llvm_example ] ===\n");
}

