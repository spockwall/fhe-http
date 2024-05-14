#![allow(dead_code)]
mod apis;
mod configs;
mod fhe_traits;
mod utils;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    // parse the json
    let file_path: &str = "./examples/json_files/user.json";
    let file: File = File::open(file_path)?;
    let contents: String = utils::file_ctl::read_from_stream(file)?;
    let _ = utils::json::parse_json(&contents);

    Ok(())
}
