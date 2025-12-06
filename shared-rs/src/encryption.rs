use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use hex;
use rand::RngCore;
use sha2::{Digest, Sha256};

pub struct SecureMessenger {
    cipher: Aes256Gcm,
}

impl SecureMessenger {
    pub fn new(key: &str) -> Result<Self> {
        // Normalize key: trim whitespace and remove any invisible characters
        // This is especially important on Windows where copy-paste can add extra characters
        let normalized_key = key.trim().replace('\0', "").replace('\r', "").replace('\n', "");
        
        let key_bytes = if let Ok(decoded) = hex::decode(&normalized_key) {
            if decoded.len() == 32 {
                decoded
            } else {
                // If hex but wrong length, hash it
                let mut hasher = Sha256::new();
                hasher.update(normalized_key.as_bytes());
                hasher.finalize().to_vec()
            }
        } else {
            // Not hex, hash it
            let mut hasher = Sha256::new();
            hasher.update(normalized_key.as_bytes());
            hasher.finalize().to_vec()
        };

        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        Ok(Self { cipher })
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self.cipher.encrypt(nonce, data)
            .map_err(|e| anyhow!("Encryption failed: {}", e))?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if data.len() < 12 {
            return Err(anyhow!("Data too short"));
        }

        let nonce = Nonce::from_slice(&data[..12]);
        let ciphertext = &data[12..];

        self.cipher.decrypt(nonce, ciphertext)
            .map_err(|e| anyhow!("Decryption failed: {}", e))
    }

    pub fn encrypt_json<T: serde::Serialize>(&self, data: &T) -> Result<String> {
        let json_bytes = serde_json::to_vec(data)?;
        let encrypted = self.encrypt(&json_bytes)?;
        Ok(general_purpose::STANDARD.encode(encrypted))
    }

    pub fn decrypt_json<T: serde::de::DeserializeOwned>(&self, data: &str) -> Result<T> {
        let encrypted_bytes = general_purpose::STANDARD.decode(data)?;
        let decrypted_bytes = self.decrypt(&encrypted_bytes)?;
        let result = serde_json::from_slice(&decrypted_bytes)?;
        Ok(result)
    }
}
