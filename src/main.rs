use std::env;
use dialoguer::{Select, theme::ColorfulTheme};
use crossterm::cursor::Show;
use crossterm::ExecutableCommand;
use std::io::stdout;

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
        "-e" => encrypt(),
        "-d" => decrypt(),
        _ => println!("Invalid argument. Please use -e for encryption or -d for decryption."),
    }
}

fn display_menu() {
    let selections = &["Encrypt", "Decrypt"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please choose your operation. Select with space key and confirm with space.")
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    match selection {
        0 => encrypt(),
        1 => decrypt(),
        _ => unreachable!(),
    }
}

fn encrypt() {
    println!("Encryption mode selected");
    // Add your encryption code here
}

fn decrypt() {
    println!("Decryption mode selected");
    // Add your decryption code here
}
