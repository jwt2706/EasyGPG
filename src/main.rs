use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use walkdir::WalkDir;
use std::io::stdout;
use std::fs;

pub mod encrypt;
pub mod decrypt;

fn main() {
    ascii();
    print!("v{}", env!("CARGO_PKG_VERSION"));
    let args: Vec<String> = env::args().collect();
    let _ = stdout().execute(Show);

    if args.len() > 1 {
        handle_args(&args[1]);
    } else {
        display_menu();
    }
}

fn ascii(){
    print!(r#"
+==========================================+
|  _____                 ____ ____   ____  |
| | ____|__ _ ___ _   _ / ___|  _ \ / ___| |
| |  _| / _` / __| | | | |  _| |_) | |  _  |
| | |__| (_| \__ \ |_| | |_| |  __/| |_| | |
| |_____\__,_|___/\__, |\____|_|    \____| |
|                 |___/                    |
+==========================================+
"#);
}

fn handle_args(arg: &str) {
    match arg {
        "-e" => encrypt::encrypt(),
        "-d" => decrypt::decrypt(),
        "-s" => scan(),
        _ => println!("Invalid argument. Please use -e for encryption or -d for decryption."),
    }
}

fn display_menu() {
    let selections = &["Encrypt", "Decrypt", "Scan system for keys"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose your operation. Select with space key and confirm with space.")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    match selection {
        0 => encrypt::encrypt(),
        1 => decrypt::decrypt(),
        2 => scan(),
        _ => unreachable!(),
    }
}

fn scan() {
    // placeholder path
    let root_dir = "/";

    let key_files: Vec<String> = WalkDir::new(root_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.file_name().to_string_lossy().ends_with(".key"))
        .map(|e| e.into_path().to_string_lossy().into_owned())
        .collect();

    // save the paths to the key files for later use
    // placeholder path
    let save_path = "/path/to/save/file";
    fs::write(save_path, key_files.join("\n")).unwrap();
}
