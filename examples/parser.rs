mod utils;

use utils::project_path;
use wasmo_parser::Parser;
use wasmo_utils::{debug, file::wat2wasm, verbose};

fn main() {
    // Convert wat file to wasm bytes.
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_export.wat").as_str());
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_add.wat").as_str());
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_block.wat").as_str());
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_more_on_stack.wat").as_str());
    // let wasm_bytes = wat2wasm(project_path("examples/wat/valid_multiple_block.wat").as_str());
    let wasm_bytes = wat2wasm(project_path("examples/wat/valid_empty_block.wat").as_str());

    // Parse wasm bytes
    let wasm_module = Parser::new(&wasm_bytes).module();

    match wasm_module {
        Ok(module) => verbose!("wasm_module = {:#?}", module),
        Err(error) => debug!("ERROR!\n\n{:#?}", error),
    }
}
