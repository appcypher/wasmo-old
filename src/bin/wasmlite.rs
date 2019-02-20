use std::env;

use wasmlite_utils::*;

fn main() {
    println!("\n=== [ wasmlite ] ===\n");

    println!(" -------------------");
    println!("|                   |");
    println!("|    STAY TUNED!    |");
    println!("|                   |");
    println!(" -------------------");

    // Get all arguments.
    let args: Vec<String> = env::args().collect();

    // Get path to running executable.
    let path_to_exe = env::current_exe().unwrap();

    // Get executable path as string.
    let path_to_exe = path_to_exe.into_os_string().into_string().unwrap();

    verbose!("executable path = {:?}\n", path_to_exe);

    verbose!("args = {:#?}\n", args);

    println!("\n=== [ wasmlite ] ===\n");
}
