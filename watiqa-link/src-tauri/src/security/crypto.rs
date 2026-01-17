use crate::error::{AppError, AppResult};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM, NONCE_LEN};
use ring::rand::{SecureRandom, SystemRandom};
use std::fs;
use std::path::PathBuf;

pub struct CryptEngine {
    key: LessSafeKey,
}

impl CryptEngine {
    pub fn new() -> AppResult<Self> {
        Self::with_path(PathBuf::from("../store/master.key"))
    }

    pub fn with_path(key_path: PathBuf) -> AppResult<Self> {
        let key_bytes = if key_path.exists() {
            fs::read(&key_path)?
        } else {
            let mut bytes = vec![0u8; 32];
            SystemRandom::new()
                .fill(&mut bytes)
                .map_err(|_| AppError::Crypto("Key generation failed".into()))?;
            if let Some(parent) = key_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&key_path, &bytes)?;
            bytes
        };

        let unbound_key = UnboundKey::new(&AES_256_GCM, &key_bytes)
            .map_err(|_| AppError::Crypto("Invalid key length".into()))?;
        let key = LessSafeKey::new(unbound_key);

        Ok(Self { key })
    }

    pub fn encrypt(&self, data: &str) -> AppResult<String> {
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; NONCE_LEN];
        rng.fill(&mut nonce_bytes)
            .map_err(|_| AppError::Crypto("Nonce generation failed".into()))?;
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);

        let mut in_out = data.as_bytes().to_vec();
        self.key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| AppError::Crypto("Encryption failed".into()))?;

        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);

        Ok(hex::encode(result))
    }

    pub fn decrypt(&self, encrypted_hex: &str) -> AppResult<String> {
        let data = hex::decode(encrypted_hex)
            .map_err(|_| AppError::Crypto("Invalid hex encoding".into()))?;
        if data.len() < NONCE_LEN {
            return Err(AppError::Crypto("Encrypted data too short".into()));
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = Nonce::assume_unique_for_key(nonce_bytes.try_into().unwrap());

        let mut in_out = ciphertext.to_vec();
        let decrypted_bytes = self
            .key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| AppError::Crypto("Decryption failed".into()))?;

        String::from_utf8(decrypted_bytes.to_vec())
            .map_err(|_| AppError::Crypto("Invalid UTF-8 after decryption".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("master.key");

        let engine = CryptEngine::with_path(key_path).unwrap();
        let original = "secret message 123";
        let encrypted = engine.encrypt(original).unwrap();
        let decrypted = engine.decrypt(&encrypted).unwrap();

        assert_eq!(original, decrypted);
        assert_ne!(original, encrypted);
    }

    #[test]
    fn test_persistence() {
        let temp_dir = tempfile::tempdir().unwrap();
        let key_path = temp_dir.path().join("master.key");

        {
            let engine = CryptEngine::with_path(key_path.clone()).unwrap();
            let _ = engine.encrypt("test").unwrap();
        }

        // Should reload same key
        let engine2 = CryptEngine::with_path(key_path).unwrap();
        let original = "consistent";
        let encrypted = engine2.encrypt(original).unwrap();
        let decrypted = engine2.decrypt(&encrypted).unwrap();
        assert_eq!(original, decrypted);
    }
}
