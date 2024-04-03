use std::io::{Read, Result};
use tfhe::{prelude::*, ClientKey, FheUint8};

pub fn read_from_stream<T: Read>(mut stream: T) -> Result<String> {
    let mut contents: String = String::new();
    stream.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_http_packet(packet: &str) -> (String, String) {
    // split the packet into header and body
    let res: Vec<&str> = packet.split("\r\n\r\n").collect();
    if res.len() == 2 {
        return (res[0].to_string(), res[1].to_string());
    }
    return ("".to_string(), "".to_string());
}

pub fn encrypt_string(input: &str, client_key: &ClientKey) -> Vec<FheUint8> {
    // divide input into 8-bit chunks
    let mut encrypted_chunks: Vec<FheUint8> = Vec::new();
    for byte in input.bytes() {
        let res = FheUint8::try_encrypt(byte, client_key);
        let temp = res.unwrap();
        encrypted_chunks.push(temp);
    }

    // turn encrypted_chunks into a string

    return encrypted_chunks;
}

pub fn decrypt_chunks(input: Vec<FheUint8>, client_key: &ClientKey) -> String {
    // decrypt chunks is a empty vector of bytes at the begining
    let mut decrypted_chunks: Vec<u8> = Vec::new();
    for chunk in input {
        let res: u8 = chunk.decrypt(client_key);
        decrypted_chunks.push(res);
    }

    // turn decrypted_chunks into a string
    let decrypted_string: String = String::from_utf8(decrypted_chunks).unwrap();
    return decrypted_string;
}
