use dialoguer::{Select, theme::ColorfulTheme};
use std::process::Command;

pub fn list() {
    // list all public keys
    let public_keys = Command::new("gpg")
        .arg("--list-keys")
        .output()
        .expect("Failed to execute gpg command");
        
    // convert output to string
    let public_keys_str = String::from_utf8(public_keys.stdout).unwrap();

    // split the output into lines and filter for lines starting with 'uid'
    let keys: Vec<String> = public_keys_str
        .lines()
        .filter(|line| line.starts_with("uid"))
        .map(|line| line.trim_start_matches("uid").trim_start().to_string())
        .collect();

    // print the keys
    for key in keys {
        println!("{}", key);
    }
}

pub fn select() -> String {
    let public_keys = Command::new("gpg")
        .arg("--list-keys")
        .output()
        .expect("Failed to execute gpg command");

    // convert output to string
    let public_keys_str = String::from_utf8(public_keys.stdout).unwrap();

    // split the output into lines and filter for lines starting with 'pub' or 'uid'
    let keys: Vec<String> = public_keys_str
        .lines()
        .filter(|line| line.starts_with("uid"))
        .map(|line| line.trim_start_matches("uid").trim_start().to_string())
        .collect();

    // selection menu
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&keys)
        .default(0)
        .interact()
        .unwrap();

    // extract the email address from the selected key
    let selected_key = keys[selection].clone();
    let email_start = selected_key.find('<').unwrap_or(0) + 1;
    let email_end = selected_key.find('>').unwrap_or(selected_key.len());
    let email = &selected_key[email_start..email_end];

    email.to_string()
}