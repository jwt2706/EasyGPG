use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use std::io::stdout;

pub mod encrypt;
pub mod decrypt;
pub mod keys;

fn main() {
    ascii();
    println!("v{}", env!("CARGO_PKG_VERSION"));
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
        "-e" => encrypt::main(),
        "-d" => decrypt::main(),
        "-s" => keys::list(),
        "--help" | "-h" => {
            println!("Usage: \n-e for encryption \n-d for decryption");
        },
        _ => println!("Invalid argument. Please use -e for encryption or -d for decryption."),
    }
}

fn display_menu() {
    let selections = &["Encrypt", "Decrypt", "View keys"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose your operation. Select with space key and confirm with space.")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    match selection {
        0 => encrypt::main(),
        1 => decrypt::main(),
        2 => keys::list(),
        _ => unreachable!(),
    }
}
