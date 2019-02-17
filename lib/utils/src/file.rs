use std::fs::File;
use std::io::prelude::Read;

/// Gets the bytes of a file as a vector of u8s.
pub fn read_file(filename: &str) -> Vec<u8> {
    // Open file.
    let file = File::open(filename)
        .unwrap_or_else(|_| panic!("Can't open file `{:?}`", filename));

    // Read bytes
    file.bytes()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Can't read file `{:?}`", filename))
}

