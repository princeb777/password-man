use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use argon2::Argon2;
use std::fs;

/// Derive a 32-byte AES key from a password using Argon2id
fn derive_key_from_password(password: &str, salt: &[u8]) -> [u8; 32] {
    let argon2 = Argon2::default();

    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("Argon2 failed to derive key");
    key
}

pub struct PasswordVault {
    pub master_password: String,
    pub file_path: String,
    pub data: String, // decrypted plain text
}

impl PasswordVault {
    /// Load vault: decrypt if file exists, else create new
    pub fn load(master_password: String, file_path: &str) -> Self {
        if let Ok(file_data) = fs::read(file_path) {
            // File exists → decrypt
            let (salt, rest) = file_data.split_at(16);
            let (nonce_bytes, cipher_text) = rest.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);

            let key = derive_key_from_password(&master_password, salt);
            let cipher = Aes256Gcm::new_from_slice(&key).unwrap();

            match cipher.decrypt(nonce, cipher_text) {
                Ok(plaintext) => {
                    let decrypted = String::from_utf8(plaintext).unwrap();
                    Self {
                        master_password,
                        file_path: file_path.to_string(),
                        data: decrypted,
                    }
                }
                Err(_) => {
                    eprintln!("❌ Wrong master password!");
                    std::process::exit(1);
                }
            }
        } else {
            // No file yet → start fresh
            Self {
                master_password,
                file_path: file_path.to_string(),
                data: String::new(),
            }
        }
    }

    /// Save current data (encrypted with master password)
    pub fn save(&self) {
        // Generate random salt
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);

        // Derive key
        let key = derive_key_from_password(&self.master_password, &salt);
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();

        // Random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let cipher_text = cipher.encrypt(nonce, self.data.as_bytes()).unwrap();

        // Final file data: [salt || nonce || ciphertext]
        let mut file_data = Vec::new();
        file_data.extend_from_slice(&salt);
        file_data.extend_from_slice(&nonce_bytes);
        file_data.extend_from_slice(&cipher_text);

        fs::write(&self.file_path, file_data).unwrap();
    }

    /// Append new entry
    pub fn add_entry(&mut self, entry: &str) {
        self.data.push_str(entry);
        self.data.push('\n');
    }

    /// Get all entries
    pub fn get_entries(&self) -> Vec<String> {
        self.data
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }
}
