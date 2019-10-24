use dotenv;
use std::env;

pub fn project_path(subpath: &str) -> String {
    // Set environment variables
    dotenv::dotenv().unwrap();

    // Append project dir to subpath
    format!("{}/{}", env::var("project_dir").unwrap(), subpath)
}
