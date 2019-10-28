//! USAGE: cargo run --example codegen --features "verbose"

use wasmo_codegen::generator::ModuleGenerator;
use wasmo_codegen::options::CodegenOptions;
use wasmo_utils::file::{convert_wat_to_wasm};
use wasmo_utils::path::project_path;
use wasmo_utils::verbose;

fn main() {
    verbose!("\n=== [ codegen_example ] ===\n");

    let wat_filename = project_path("examples/wat/valid/func-body.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-data.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-table.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-table-start.wat");
    // let wat_filename = project_path("examples/wat/invalid/start-parameter.wat");
    // let wat_filename = project_path("examples/wat/valid/start.wat");

    let wasm_binary = match convert_wat_to_wasm(&wat_filename) {
        Err(error) => panic!("Conversion Error! = {:?}", error),
        Ok(binary) => binary,
    };

    let options = &CodegenOptions::default();

    let result = ModuleGenerator::new(&wasm_binary, options).generate_module();

    // Error handing
    let result = match result {
        Err(error) => panic!("Parsing Error! = {:?}", error),
        Ok(result) => {
            verbose!("LLVM Module generated! = {:?}", result.0);
            verbose!("Runtime Module Data generated! = {:?}", result.1);
            result
        }
    };

    verbose!("\n=== [ codegen_example ] ===\n");
}
