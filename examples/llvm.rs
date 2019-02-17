use wasmlite_llvm::codegen::LLVMCodegen;

fn main() {
    println!("\n=== [ llvm_example ] ===\n");
    let llvm_codegen = LLVMCodegen::new("my_wasm_file");
    println!("\n=== [ llvm_example ] ===\n");
}
