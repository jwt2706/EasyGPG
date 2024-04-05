use dialoguer::Input;
use std::path::Path;
use std::process::Command;
use std::fs::write;

use crate::keys::select;

pub fn encrypt_file(file_path: &str) {
    // TODO: Implement file encryption
    println!("Encrypting file: {}", file_path);
}

pub fn encrypt_text(text: &str) {
    // TODO: Implement text encryption
    println!("Encrypting text: {}", text);
}

pub fn check_input_type(file_path: &Path) -> String {
    if file_path.exists() {
        encrypt_file(file_path.to_str().unwrap());
        "file".to_string()
    } else {
        encrypt_text(file_path.to_str().unwrap());
        "text".to_string()
    }
}

pub fn encrypt(input: &str, key: &str, input_type: &str) -> Vec<u8> {
    let output = if input_type == "text" {
        Command::new("gpg")
            .arg("--encrypt")
            .arg("--armor")
            .arg("--recipient")
            .arg(key)
            .arg("--batch")
            .arg("--yes")
            .arg("--passphrase")
            .arg(input)
            .output()
            .expect("Failed to execute gpg command")
    } else { // input_type == "file"
        let output_file = format!("{}.gpg", input);
        Command::new("gpg")
            .arg("--encrypt")
            .arg("--recipient")
            .arg(key)
            .arg("--output")
            .arg(&output_file)
            .arg(input)
            .output()
            .expect("Failed to execute gpg command")
    };

    if !output.status.success() {
        eprintln!("Encryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    output.stdout
}

pub fn main() {
    println!("Let's encrypt!");
    // prompt user to input
    let user_input: String = Input::new()
        .with_prompt("Please paste your message, or file path/name")
        .interact()
        .unwrap();

    let file_path = Path::new(&user_input);
    let input_type = check_input_type(&file_path);

    // we list the current public keys available for us to encrypt for. And prompt the user to choose, using an options menu
    let key = select();
    println!("Selected key: {}", key);

    // encrypt the file or message using that key
    let encrypted_output = encrypt(&user_input, &key, &input_type);

    // if the input type is "text", write the encrypted output to a "msg.txt" file
    // otherwise, the encrypted output is already a .gpg file
    if input_type == "text" {
        write("msg.txt", &encrypted_output).expect("Failed to write to file");
    }
}