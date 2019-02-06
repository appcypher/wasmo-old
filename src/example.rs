use wasm2llvm_parser::parser::Parser;

use wasm2llvm::wasm_codes::{
    get_import_section,
};

fn main() {
    Parser::new(&get_import_section()).module();
}
