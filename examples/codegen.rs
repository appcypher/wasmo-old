//! USAGE: cargo run --example codegen --features "verbose"
mod utils;

use wasmo_codegen::generate_module;
use wasmo_utils::read_file_bytes;
use utils::project_path;


fn main() {
    verbose!("\n=== [ codegen_example ] ===\n");

    let wasm_filename = project_path("examples/wat/valid_export_section.wat");

    let wasm_binary = read_file_bytes(wasm_filename.as_str());

    generate_module(&wasm_binary);

    verbose!("\n=== [ codegen_example ] ===\n");
}

