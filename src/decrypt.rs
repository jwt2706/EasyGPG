use dialoguer::Input;
use std::path::Path;
use std::ffi::OsStr;
use std::fs;

pub fn decrypt_gpg(file_path: &Path) {
    println!("Decrypted .gpg file at path: {:?}", file_path);
    // TODO: add .gpg decryption code here
}

pub fn decrypt_txt(file_path: &Path) {
    println!("Decrypted .txt file content: {}", content);
    // TODO: add .txt decryption code here
}

pub fn decrypt_plain_text(encrypted_text: &str) {
    println!("Plain text: {}", encrypted_text);
    // TODO: add plain text decryption code here
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

pub fn decrypt() {
    println!("Decryption mode selected");

    let user_input: String = Input::new()
        .with_prompt("Please paste the message, or file path/name")
        .interact()
        .unwrap();

    let file_path = Path::new(&user_input);
    if file_path.exists() {
        check_input_type(&file_path);
    } else {
        // if the input is not a path to a file, assume it's the encrypted message
        decrypt_plain_text(&user_input);
    }
}