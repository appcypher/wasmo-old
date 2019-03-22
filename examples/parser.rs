mod utils;

use utils::project_path;
use wasmlite_parser::Parser;
use wasmlite_utils::{debug, file::wat2wasm, verbose};

fn main() {
    // Convert wat file to wasm bytes.
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_export_section.wat").as_str());
    let wasm_bytes = wat2wasm(project_path("examples/wat/valid_add_operations.wat").as_str());

    // Parse wasm bytes
    let wasm_module = Parser::new(&wasm_bytes).module();

    match wasm_module {
        Ok(module) => verbose!("wasm_module = {:#?}", module),
        Err(error) => debug!("ERROR!\n\n{:#?}", error),
    }
}
