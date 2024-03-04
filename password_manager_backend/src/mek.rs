use crate::crypto::Cryptographer;
use crate::crypto::SecureData;
use crate::error_handling::CryptoError;
use ring::pbkdf2;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

const KEY_SIZE: usize = 32; // aes-256-gcm key size

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MekData {
    pub(crate) encrypted_mek: SecureData, // Contains the encrypted MEK, nonce, and salt for the MEK encryption
    pub(crate) mek_salt: Vec<u8>, // Salt used to derive the key for MEK encryption/decryption
}

impl MekData {
    pub fn new(master_password: &[u8]) -> Result<Self, CryptoError> {
        let crypto_bub: Cryptographer = Cryptographer::new(None);
        let mek_salt: Vec<u8> = Cryptographer::generate_salt()?; // length 16
        let derived_key: [u8; KEY_SIZE] = MekData::derive_mek_key(master_password, &mek_salt);
        let mek: Vec<u8> = Cryptographer::generate_random_bytes()?;

        let encrypted_mek: SecureData = Cryptographer::encrypt(&crypto_bub, &mek, &derived_key)?;

        Ok(Self {
            encrypted_mek,
            mek_salt,
        })
    }

    // Decrypts the MEK and returns it
    pub fn decrypt_mek(&self, master_password: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let cryptographer: Cryptographer = Cryptographer::new(None);
        let derived_key: [u8; KEY_SIZE] = MekData::derive_mek_key(master_password, &self.mek_salt);
        let mek: Vec<u8> = cryptographer.decrypt(self.encrypted_mek.clone(), &derived_key)?;
        Ok(mek)
    }

    // meant to encrypt the mek back using the same master key, not the same as update_mek
    pub fn reencrypt_mek(&self, master_password: &[u8], mek: &[u8]) -> Result<Self, CryptoError> {
        let cryptographer: Cryptographer = Cryptographer::new(None);
        let derived_key: [u8; KEY_SIZE] = MekData::derive_mek_key(master_password, &self.mek_salt);
        let encrypted_mek: SecureData = cryptographer.encrypt(mek, &derived_key)?;
        Ok(MekData {
            encrypted_mek,
            mek_salt: self.mek_salt.clone(),
        })
    }

    pub fn update_mek(&mut self, old_master_password: &[u8], new_master_password: &[u8]) -> Result<Self, CryptoError> {
        let cryptographer: Cryptographer = Cryptographer::new(None);
        
        // Derive the old key and decrypt the existing MEK
        let old_key: [u8; KEY_SIZE] = MekData::derive_mek_key(old_master_password, &self.mek_salt);
        let mek: Vec<u8> = cryptographer.decrypt(self.encrypted_mek.clone(), &old_key)?;
    
        // Derive the new key from the new master password
        let new_key: [u8; KEY_SIZE] = MekData::derive_mek_key(new_master_password, &self.mek_salt);
    
        // Re-encrypt the MEK with the new key
        self.encrypted_mek = cryptographer.encrypt(&mek, &new_key)?;
    
        Ok(self.clone())
    }

    // generate derived key from master password and salt
    pub fn derive_mek_key(master_password: &[u8], salt: &[u8]) -> [u8; KEY_SIZE] {
        let mut key: [u8; KEY_SIZE] = [0u8; KEY_SIZE];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            master_password,
            &mut key,
        );
        key
    }
}