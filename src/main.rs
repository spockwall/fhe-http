use std::fs::File;
use std::io::Result;
mod utils;
use tfhe::{generate_keys, ConfigBuilder};

fn main() -> Result<()> {
    // create a fhe config builder
    let config: tfhe::Config = ConfigBuilder::default().build();
    let (client_key, _) = generate_keys(config);

    // read text from file
    let file_path: &str = "./examples/request.txt";
    let file: File = File::open(file_path)?;
    let contents: String = utils::file_ctl::read_from_stream(file)?;

    //// turn content from String to &str
    let encoded: String = utils::base64::encode(contents.as_str());
    let decoded: String = utils::base64::decode(encoded.as_str());

    println!("Encode: {}\n", encoded);
    println!("Decode: {}", decoded);

    // encrypt & decrypt the text
    let res: Vec<tfhe::FheUint<tfhe::FheUint8Id>> =
        utils::file_ctl::encrypt_string(&encoded, &client_key);
    let decrypted: String = utils::file_ctl::decrypt_chunks(res, &client_key);
    println!("Decrypted: {}", decrypted);

    // extract the header and body from the packet
    let (header, body) = utils::file_ctl::parse_http_packet(&contents);
    print!("Header:\n {}\n\nBody:\n {}", header, body);
    Ok(())
}
