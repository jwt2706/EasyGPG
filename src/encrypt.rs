use dialoguer::Input;
use std::path::Path;
use std::process::Command;
use std::fs::write;

use crate::keys::select;

pub fn encrypt(input: &str, key: &str, input_type: &str) -> Vec<u8> {
    let output = match input_type {
        "text" => {
            Command::new("gpg")
                .args(&["--encrypt", "--armor", "--recipient", key, "--batch", "--yes", "--passphrase", input])
                .output()
                .expect("Failed to execute gpg command")
        },
        _ => { // input_type == "file"
            let output_file = format!("{}.gpg", input);
            Command::new("gpg")
                .args(&["--encrypt", "--recipient", key, "--output", &output_file, input])
                .output()
                .expect("Failed to execute gpg command")
        }
    };

    if !output.status.success() {
        eprintln!("Encryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    output.stdout
}

pub fn main() {
    println!("Let's encrypt!");

    let user_input: String = Input::new()
        .with_prompt("Please paste your message, or file path/name")
        .interact()
        .unwrap();

    let input_type = if Path::new(&user_input).exists() { "file" } else { "text" };
    let key = select();

    println!("Selected key: {}", key);

    let encrypted_output = encrypt(&user_input, &key, &input_type);

    if input_type == "text" {
        write("msg.txt", &encrypted_output).expect("Failed to write to file");
    }
}