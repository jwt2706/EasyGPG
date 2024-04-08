use rpassword::read_password;
use std::io::{self, BufRead};
use std::process::Command;
use std::io::Write;
use std::process::Stdio;

pub fn decrypt_plain_text(encrypted_text: &str, passphrase: &str) {
    let mut child = Command::new("gpg")
        .args(&["--decrypt", "--passphrase-fd", "0"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute gpg command");

    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(passphrase.as_bytes()).expect("Failed to write to stdin");
        stdin.write_all(b"\n").expect("Failed to write to stdin");
        stdin.write_all(encrypted_text.as_bytes()).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    println!("Output: {:?}", output);

    if output.status.success() {
        println!("Decrypted text: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Decryption failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

pub fn main() {
    println!("Let's decrypt!");

    let stdin = io::stdin();
    let mut user_input = String::new();

    println!("Please paste the encrypted message. Press Ctrl-D when you're done:");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        user_input.push_str(&line);
        user_input.push('\n');
    }

    println!("Please enter your passphrase:");
    let passphrase = read_password().unwrap();

    let trimmed_input = user_input.trim();
    decrypt_plain_text(trimmed_input, &passphrase);
}