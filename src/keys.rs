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

pub fn selection() {
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

    println!("You selected: {}", keys[selection]);
}