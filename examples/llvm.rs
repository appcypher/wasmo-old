use wasmlite_llvm::codegen::generate_module;

use wasmlite_parser::ir::Module;

fn main() {
    println!("\n=== [ llvm_example ] ===\n");

    let empty_wasm_module = Module { sections: vec![] };

    let llvm_module = generate_module(&empty_wasm_module);

    println!("\n=== [ llvm_example ] ===\n");
}
