use wasmlite_parser::parser::Parser;

use wasmlite::wasm_codes::function_with_locals_only;

fn main() {
    Parser::new(&function_with_locals_only()).module().unwrap();
}
