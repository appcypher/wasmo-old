//! USAGE: cargo run --example codegen --features "verbose"

use wasmo_codegen::generator::ModuleGenerator;
use wasmo_codegen::options::CodegenOptions;
use wasmo_runtime::module::{Module, ModuleAOT};
use wasmo_runtime::options::Options;
use wasmo_utils::file::convert_wat_to_wasm;
use wasmo_utils::path::project_path;
use wasmo_utils::verbose;

fn main() {
    verbose!("\n=== [ codegen_example ] ===\n");

    let wat_file_path = project_path("examples/wat/valid/func-body.wat");

    let wasm_binary = match convert_wat_to_wasm(&wat_file_path) {
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

    let options = &Options::default();

    let module: ModuleAOT = Module::create_aot_with_llvm_module(result.0, options);

    verbose!("Runtime Module generated! = {:?}", module);

    verbose!("\n=== [ codegen_example ] ===\n");
}
