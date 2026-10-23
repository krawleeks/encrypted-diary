use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use sha2::Sha256;

const MAGIC: &[u8] = b"DIARY1";
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const PBKDF2_ROUNDS: u32 = 600_000;

// private: only used inside this file
fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ROUNDS, &mut key);
    key
}

pub fn encrypt(password: &str, plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    getrandom::fill(&mut salt).map_err(|e| format!("random generator failed: {}", e))?;
    getrandom::fill(&mut nonce_bytes).map_err(|e| format!("random generator failed: {}", e))?;

    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| "bad key length".to_string())?;
    let nonce = Nonce::try_from(&nonce_bytes[..]).map_err(|_| "bad nonce length".to_string())?;

    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| "encryption failed".to_string())?;

    // so the file made looks like: label | salt | nonce | encrypted data
    let mut out = Vec::new();
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn decrypt(password: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    let header_len = MAGIC.len() + SALT_LEN + NONCE_LEN;
    if data.len() < header_len || &data[..MAGIC.len()] != MAGIC {
        return Err("not a diary file".to_string());
    }
    let salt = &data[MAGIC.len()..MAGIC.len() + SALT_LEN];
    let nonce_bytes = &data[MAGIC.len() + SALT_LEN..header_len];
    let ciphertext = &data[header_len..];

    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| "bad key length".to_string())?;
    let nonce = Nonce::try_from(nonce_bytes).map_err(|_| "bad nonce length".to_string())?;

    cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| "wrong password or corrupted file".to_string())
}