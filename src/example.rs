use wasmlite_parser::parser::Parser;

use wasmlite::wasm_codes::{
    get_import_section,
};

fn main() {
    Parser::new(&get_import_section()).module();
}
