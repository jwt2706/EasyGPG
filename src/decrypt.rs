use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;
use std::io::Write;
use std::fs::File;

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

pub fn decrypt_message(encrypted_text: &str) {
    // Write the encrypted text to a temporary file
    let mut temp_file = File::create("temp.gpg").expect("Failed to create temp file");
    write!(temp_file, "{}", encrypted_text).expect("Failed to write to temp file");

    // Decrypt the temporary file
    let output_file = "decrypted_message.txt";
    let output = Command::new("gpg")
        .args(&["--output", output_file, "--decrypt", "temp.gpg"])
        .output()
        .expect("Failed to execute gpg command");

    if output.status.success() {
        println!("Decryption successful. Decrypted message written to {}", output_file);
    } else {
        eprintln!("Decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn check_input(file_path: &Path) {
    if file_path.exists() {
        match file_path.extension().and_then(OsStr::to_str) {
            Some("gpg") => decrypt_file(file_path),
            _ => (),
        }
    } else {
        decrypt_message(file_path.to_str().unwrap());
    }
}

pub fn main() {
    println!("Let's decrypt!");

    let stdin = io::stdin();
    let mut user_input = String::new();

    println!("Please paste the message, or file path/name. Press Ctrl-D when you're done:");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        user_input.push_str(&line);
        user_input.push('\n');
    }

    let trimmed_input = user_input.trim();
    let file_path = Path::new(trimmed_input);
    check_input(&file_path);
}