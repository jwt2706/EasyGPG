use dialoguer::Input;
use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;
use std::fs;

pub fn decrypt_gpg(file_path: &Path) {
    let output_file = file_path.with_extension("");
    let output = Command::new("gpg")
        .args(&["--output", output_file.to_str().unwrap(), "--decrypt", file_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute gpg command");

    if output.status.success() {
        println!("Decrypted .gpg file at path: {:?}", output_file);
    } else {
        eprintln!("Decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn decrypt_txt(file_path: &Path) {
    let content = fs::read_to_string(file_path)
        .expect("Could not read file!");
    println!("Decrypted .txt file content: {}", content);
}

pub fn decrypt_plain_text(encrypted_text: &str) {
    // Assuming the plain text is not actually encrypted
    println!("Plain text: {}", encrypted_text);
}

pub fn check_input_type(file_path: &Path) {
    if file_path.exists() {
        match file_path.extension().and_then(OsStr::to_str) {
            Some("gpg") => decrypt_gpg(file_path),
            Some("txt") => decrypt_txt(file_path),
            _ => (),
        }
    } else {
        decrypt_plain_text(file_path.to_str().unwrap());
    }
}

pub fn main() {
    println!("Let's decrypt!");

    let user_input: String = Input::new()
        .with_prompt("Please paste the message, or file path/name")
        .interact()
        .unwrap();

    let file_path = Path::new(&user_input);
    check_input_type(&file_path);
}