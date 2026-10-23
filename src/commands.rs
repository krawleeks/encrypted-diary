use crate::editor::open_in_editor;
use crate::storage;
use std::env;

fn ask_password() -> Result<String, String> {
    rpassword::prompt_password("Password: ").map_err(|e| e.to_string())
}

fn ask_new_password() -> Result<String, String> {
    let first = rpassword::prompt_password("Choose a password: ").map_err(|e| e.to_string())?;
    let second = rpassword::prompt_password("Repeat password: ").map_err(|e| e.to_string())?;
    if first != second {
        return Err("passwords did not match".to_string());
    }
    Ok(first)
}

// existing diary then ask once. no diary yet - ask twice for a new password
fn password_for_diary() -> Result<String, String> {
    if storage::diary_exists() {
        ask_password()
    } else {
        ask_new_password()
    }
}

pub fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  diary add <text>");
        println!("  diary edit    (edit in nvim)");
        println!("  diary view    (view in nvim)");
        println!("  diary clear");
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                return Err("please give some text to add".to_string());
            }
            let text = args[2..].join(" ");

            let password = password_for_diary()?;
            let mut contents = storage::load_diary(&password)?;
            contents.push_str(&text);
            contents.push('\n');
            storage::save_diary(&password, &contents)?;
            println!("Saved.");
        }
        "edit" => {
            let password = password_for_diary()?;
            let contents = storage::load_diary(&password)?;
            let edited = open_in_editor(&contents, false)?;

            if edited == contents {
                println!("No changes.");
            } else {
                storage::save_diary(&password, &edited)?;
                println!("Saved.");
            }
        }
        "view" => {
            let password = ask_password()?;
            let contents = storage::load_diary(&password)?;
            open_in_editor(&contents, true)?;
        }
        "clear" => {
            storage::delete_diary()?;
            println!("Diary cleared.");
        }
        _ => return Err(format!("unknown command: {}", args[1])),
    }
    Ok(())
}