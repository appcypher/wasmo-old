use std::env;

use wasmo_utils::verbose;

// Use clap for argument parsing.

fn main() {
    println!("\n === [ wasmo ] ===\n");

    println!("|                   |");
    println!("|    STAY TUNED!    |");
    println!("|                   |");

    // Get all arguments.
    let args: Vec<String> = env::args().collect();

    verbose!("args = {:#?}\n", args);

    println!("\n === [ wasmo ] ===\n");
}
