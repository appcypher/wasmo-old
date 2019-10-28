use dotenv;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

pub fn project_path(subpath: &str) -> PathBuf {
    // Set environment variables
    dotenv::dotenv().unwrap();

    // Append project dir to subpath
    PathBuf::from_str(&format!("{}/{}", env::var("project_dir").unwrap(), subpath))
        .expect("Cannot create project path from string")
}
