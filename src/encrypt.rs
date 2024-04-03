use dialoguer::Input;
use std::path::Path;

use crate::keys::select;

pub fn encrypt_file(file_path: &str) {
    // TODO: Implement file encryption
    println!("Encrypting file: {}", file_path);
}

pub fn encrypt_text(text: &str) {
    // TODO: Implement text encryption
    println!("Encrypting text: {}", text);
}

pub fn check_input_type(file_path: &Path) {
    if file_path.exists() {
        encrypt_file(file_path.to_str().unwrap());
    } else {
        encrypt_text(file_path.to_str().unwrap());
    }
}

pub fn main() {
    println!("Let's encrypt!");
    // prompt user to input
    let user_input: String = Input::new()
        .with_prompt("Please paste your message, or file path/name")
        .interact()
        .unwrap();

    let file_path = Path::new(&user_input);
    check_input_type(&file_path);

    // TODO: next we list the current public keys available for us to sign for. And prompt the user to choose, using an options menu
    let key = select();
    println!("Selected key: {}", key);

    // TODO: encrypt the file or message using that key

    // TODO: save the encrypted file in a directory, or print the encrypted text in the terminal
}