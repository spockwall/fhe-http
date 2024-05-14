use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("tfhe_version.rs");

    // Run `cargo metadata` and capture the output
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    let metadata = String::from_utf8(output.stdout).expect("cargo metadata output not valid UTF-8");

    // Find the version of the tfhe crate
    let mut tfhe_version = "unknown";
    for package in metadata.split("{").skip(1) {
        if package.contains("\"name\":\"tfhe\"") {
            for line in package.split(",") {
                if line.contains("\"version\":\"") {
                    tfhe_version = line.split("\"").nth(3).unwrap();
                    break;
                }
            }
        }
    }

    // Write the version information to a file
    fs::write(
        dest_path,
        format!("pub const TFHE_VERSION: &str = \"{}\";\n", tfhe_version),
    )
    .expect("Unable to write tfhe_version.rs");
}
