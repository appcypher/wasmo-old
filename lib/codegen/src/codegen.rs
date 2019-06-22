use wasmparser::Parser;

pub fn generate_module(wasm_binary: &[u8]) -> () {
    let parser = Parser::new(wasm_binary);

    // Imports
    // Tables & Elements (Initializer & Imported)
    // Memories & Data (Initializer & Imported)
    // Globals & Values (Initializer & Imported)
    // Start
    // Functions & Code
    // Exports

    println!("hello there!");
}

