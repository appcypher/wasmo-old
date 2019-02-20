use std::fs::File;
use std::io::prelude::Read;

/// Gets the bytes of a file as a vector of u8s.
pub fn read_file_bytes(filename: &str) -> Vec<u8> {
    // Open file.
    let file = File::open(filename).unwrap_or_else(|_| panic!("Can't open file `{:?}`", filename));

    // Read bytes
    file.bytes()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Can't read file `{:?}`", filename))
}

/// Checks if a file ia a wasm file.
pub fn is_wasm_file(filename: &str) -> bool {
    // Open file.
    let file = File::open(filename).unwrap_or_else(|_| panic!("Can't open file `{:?}`", filename));

    // Read bytes
    let bytes = file
        .bytes()
        .take(4)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Can't read file `{:?}`", filename));

    // Check if the file starts with bytes "\0asm"
    bytes.starts_with(b"\0asm")
}
