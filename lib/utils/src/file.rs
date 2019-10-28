use std::fs::File;
use std::io;
use std::io::prelude::Read;
use std::path::Path;
use std::fmt::Debug;
use wabt;

/// Gets the bytes of a file as a vector of u8s.
pub fn get_file_bytes<P: AsRef<Path> + Debug + Copy>(file_path: P) -> Result<Vec<u8>, String> {
    // Open file.
    let file = File::open(file_path)
        .map_err(|_| format!("Unable to open file: {:?}", file_path))?;

    // Read bytes
    file.bytes().collect::<Result<Vec<_>, _>>().map_err(|_| format!("Unable to read file: {:?}", file_path))
}

/// Checks if a file ia a wasm file.
pub fn is_wasm_file<P: AsRef<Path> + Debug + Copy>(file_path: P) -> Result<bool, String> {
    // Open file.
    let file = File::open(file_path)
        .map_err(|_| format!("Unable to open file: {:?}", file_path))?;

    // Read bytes
    let bytes = file
        .bytes()
        .take(4)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| format!("Unable to read file: {:?}", file_path))?;

    // Check if the file starts with bytes "\0asm"
    Ok(bytes.starts_with(b"\0asm"))
}

pub fn convert_wat_to_wasm<P: AsRef<Path> + Debug + Copy>(file_path: P) -> Result<Vec<u8>, String> {
    let mut file = File::open(file_path).expect("Unable to open the file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|_| format!("Unable to read file: {:?}", file_path))?;

    wabt::wat2wasm(contents).map_err(|e| format!("Conversion error: {:?}", e))
}
