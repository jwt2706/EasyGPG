use dialoguer::Input;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

pub fn decrypt() {
    println!("Decryption mode selected");

let user_input: String = Input::new()
    .with_prompt("Please paste the message, or file path")
    .interact()
    .unwrap();

    // determine if the input is a path to a file or the encrypted message itself
    let mut encrypted_message: String = String::new();
    if Path::new(&user_input).exists() {
        println!("Input is a file path");

        // if the input is a path to a file, read the file
        let file_path = Path::new(&user_input);
        let extension = file_path.extension().and_then(OsStr::to_str);
        match extension {
            Some("txt") => {
                encrypted_message = fs::read_to_string(file_path).unwrap();
                println!("File content: {}", encrypted_message); // Print the file content
            },
            Some("gpg") => {
                // if the file is a .gpg file, you need to decrypt it first
                // TODO: add .gpg decryption code here or call another func or smth
                println!("File is a .gpg file");
            },
            _ => {
                println!("Unsupported file type or file not actually encrypted.");
                return;
            }
        }
    } else {
        // if the input is not a path to a file, assume it's the encrypted message
        encrypted_message = user_input;
        println!("Input is an encrypted message.");
    }

    // todo: once we know what we're dealing with, then we got to find the correct key in the user's keyring
}