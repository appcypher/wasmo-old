//! USAGE: cargo run --example codegen --features "verbose"

mod utils;
use wasmo_utils::{file::{read_file_bytes, wat2wasm}, verbose};
use wasmo_codegen::generate_module;
use utils::project_path;


fn main() {
    verbose!("\n=== [ codegen_example ] ===\n");

    let wat_filename = project_path("examples/wat/valid/table-elem.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-data.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-table.wat");
    // let wat_filename = project_path("examples/wat/valid/mem-table-start.wat");
    // let wat_filename = project_path("examples/wat/invalid/start-parameter.wat");
    // let wat_filename = project_path("examples/wat/valid/start.wat");

    let wasm_binary = match wat2wasm(wat_filename.as_str()) {
        Err(error) => panic!("Conversion Error! = {:?}", error),
        Ok(module) => module,
    };

    let module = generate_module(&wasm_binary);

    // Error handing
    match module {
        Err(error) => println!("Parsing Error! = {:?}", error),
        Ok(module) => println!("Compiled module = {:?}", module),
    }

    verbose!("\n=== [ codegen_example ] ===\n");
}

