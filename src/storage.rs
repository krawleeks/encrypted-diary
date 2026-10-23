use crate::crypto;
use std::fs;
use std::path::Path;

const DIARY_PATH: &str = "/tmp/diary.enc";

pub fn diary_exists() -> bool {
    Path::new(DIARY_PATH).exists()
}

//decrypt diary file and return as string 
pub fn load_diary(password: &str) -> Result<String, String> {
    if !diary_exists() {
        return Ok(String::new()); //if no diary then empty string
    }
    let data = fs::read(DIARY_PATH).map_err(|e| format!("could not read file: {}", e))?;
    let plain = crypto::decrypt(password, &data)?;
    String::from_utf8(plain).map_err(|_| "diary contains invalid text".to_string())
}

//save diary by encrypting and write it to diary.enc
pub fn save_diary(password: &str, text: &str) -> Result<(), String> {
    let data = crypto::encrypt(password, text.as_bytes())?;
    fs::write(DIARY_PATH, data).map_err(|e| format!("could not write file: {}", e))
}

pub fn delete_diary() -> Result<(), String> {
    fs::remove_file(DIARY_PATH).map_err(|e| format!("could not clear diary: {}", e))
}