//! USAGE: cargo run --example llvm --features "verbose"

mod utils;

use wasmlite_utils::{verbose, debug, file::wat2wasm};
use wasmlite_llvm::codegen;
use wasmlite_parser::Parser;
use utils::project_path;

fn main() {
    verbose!("\n=== [ llvm_example ] ===\n");

    let wasm_file = project_path("examples/wat/valid_export_section.wat");

    let wasm_module = Parser::new(&wat2wasm(wasm_file.as_str())).module().unwrap();

    verbose!("wasm_module = {:#?}", wasm_module);

    let llvm_module = codegen::generate_module(&wasm_module);

    verbose!("llvm_module = {:?}", llvm_module);

    verbose!("\n=== [ llvm_example ] ===\n");
}

