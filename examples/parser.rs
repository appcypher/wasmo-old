mod wasm;
use wasm::samples;
use wasmlite_parser::parser::Parser;

fn main() {
    // Parser::new(&samples::valid_module_with_nop_in_function_body()).module().unwrap();
    // Parser::new(&samples::valid_module_with_table_section_and_maximum()).module().unwrap();
    // Parser::new(&samples::valid_module_with_table_section_no_maximum()).module().unwrap();
    Parser::new(&samples::valid_module_with_memory_section_and_maximum()).module().unwrap();
}
