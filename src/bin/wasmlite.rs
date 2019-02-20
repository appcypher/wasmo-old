use std::env;

use wasmlite_utils::*;


fn main() {
    debug!("\n=== [ wasmlite ] ===\n");

    // Get all arguments.
    let args: Vec<String> = env::args().collect();

    // Get path to running executable.
    let path_to_exe = env::current_exe().unwrap();

    let path_to_exe = path_to_exe
        .into_os_string()
        .into_string()
        .unwrap();

    debug!("executable path = {:#?}\n", path_to_exe);

    debug!("args = {:#?}\n", args);

    debug!("\n=== [ wasmlite ] ===\n");
}
