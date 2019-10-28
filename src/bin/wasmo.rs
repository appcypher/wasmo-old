#[macro_use]
extern crate wasmo_utils;
mod args;

use args::ArgumentsHandler;

fn main() -> Result<(), String> {
    ArgumentsHandler::new().setup()?;
    Ok(())
}
