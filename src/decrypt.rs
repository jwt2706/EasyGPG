use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
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

pub fn decrypt_message(encrypted_text: &str) {
    let output_file = "decrypted_message.txt";
    let mut child = Command::new("gpg")
        .args(&["--output", output_file, "--decrypt"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute gpg command");

    child.stdin.as_mut().unwrap().write_all(encrypted_text.as_bytes()).expect("Failed to write to stdin");

    let output = child.wait_with_output().expect("Failed to wait on child");

    if output.status.success() {
        println!("Decryption successful. Decrypted message written to {}", output_file);
    } else {
        eprintln!("Decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn check_input(input: &str) {
    let input_path = Path::new(input);
    if input_path.exists() {
        match input_path.extension().and_then(OsStr::to_str) {
            Some("gpg") => decrypt_file(input_path),
            _ => (),
        }
    } else {
        decrypt_message(input);
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
    check_input(trimmed_input);
}