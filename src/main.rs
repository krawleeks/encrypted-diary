mod crypto;
mod commands;
mod storage;
mod editor;

use crate::commands::run;

fn main() {
    if let Err(e) = run() {
        println!("Error: {}", e);
        std::process::exit(1);
    }
}
