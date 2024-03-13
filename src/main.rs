use std::fs::File;
use std::io::Result;
mod utils;

fn main() -> Result<()> {
    let file_path: &str = "./examples/request.txt";
    let file: File = File::open(file_path)?;
    let contents: String = utils::file_ctl::read_from_stream(file)?;

    // turn content from String to &str
    let encoded = utils::base64::encode(contents.as_str());
    let decoded = utils::base64::decode(encoded.as_str());

    println!("Encode: {}\n", encoded);
    println!("Decode: {}", decoded);
    Ok(())
}
