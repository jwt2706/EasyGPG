use std::io::{self, BufRead};
use std::process::Command;
use std::path::Path;
use std::ffi::OsStr;

pub fn decrypt_file(file_path: &Path) {
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

pub fn check_input(input: &str) {
    let input_path = Path::new(input);
    if input_path.exists() && input_path.extension().and_then(OsStr::to_str) == Some("gpg") {
        decrypt_file(input_path);
    } else {
        println!("Invalid input. Please provide a path to a .gpg file.");
        // TODO: add direct ascii-armored input decryption
    }
}

pub fn main() {
    println!("Let's decrypt!");

    let stdin = io::stdin();
    let mut user_input = String::new();

    println!("Please enter the file path/name:");

    let line = stdin.lock().lines().next().expect("Failed to read line");
    let line = line.expect("Failed to read line");
    user_input.push_str(&line);

    let trimmed_input = user_input.trim();
    check_input(trimmed_input);
}