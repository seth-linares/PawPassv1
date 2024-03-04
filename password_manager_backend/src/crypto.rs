use crate::error_handling::CryptoError;
use rand::{rngs::OsRng, RngCore};
use ring::{
    aead::Aad, aead::LessSafeKey, aead::Nonce, aead::UnboundKey, aead::AES_256_GCM, pbkdf2,
    rand::SecureRandom, rand::SystemRandom,
};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;
use zeroize::Zeroize;

const KEY_SIZE: usize = 32;
const ITERATIONS: u32 = 100_000;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cryptographer {
    pub iterations: NonZeroU32,
}

impl Cryptographer {
    pub fn new(iterations: Option<u32>) -> Self {
        let iterations: NonZeroU32 = NonZeroU32::new(iterations.unwrap_or(ITERATIONS))
            .expect("Iteration count cannot be zero.");

        Self { iterations }
    }

    pub fn generate_random_bytes() -> Result<Vec<u8>, CryptoError> {
        let mut bytes = vec![0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        Ok(bytes)
    }

    

    pub fn generate_salt() -> Result<Vec<u8>, CryptoError> {
        let mut salt: Vec<u8> = vec![0u8; 16];
        let rng: SystemRandom = SystemRandom::new();
        rng.fill(&mut salt)
            .map_err(|_| CryptoError::SaltGenerationFailed)?;
        Ok(salt)
    }

    pub fn generate_nonce() -> Result<[u8; 12], CryptoError> {
        let mut nonce: [u8; 12] = [0u8; 12];
        let rng: SystemRandom = SystemRandom::new();
        rng.fill(&mut nonce)
            .map_err(|_| CryptoError::NonceGenerationFailed)?;
        Ok(nonce)
    }

    fn derive_encryption_key(&self, password: &[u8], salt: &[u8]) -> EncryptionKey {
        let mut key: Vec<u8> = vec![0u8; KEY_SIZE];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            self.iterations,
            salt,
            password,
            &mut key,
        );

        EncryptionKey { key }
    }

    pub fn encrypt(&self, data: &[u8], password: &[u8]) -> Result<SecureData, CryptoError> {
        let nonce_array: [u8; 12] = Self::generate_nonce()?;
        let nonce: Nonce = Nonce::assume_unique_for_key(nonce_array);
        let gen_salt: Vec<u8> = Self::generate_salt()?;
        let encryption_key: EncryptionKey = self.derive_encryption_key(password, &gen_salt);
        let sealing_key: LessSafeKey = LessSafeKey::new(
            UnboundKey::new(&AES_256_GCM, &encryption_key.key)
                .map_err(|_| CryptoError::EncryptionFailed)?,
        );

        let mut in_out: Vec<u8> = Vec::with_capacity(data.len() + AES_256_GCM.tag_len());

        in_out.extend_from_slice(data);

        sealing_key
            .seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| CryptoError::EncryptionFailed)?;

        Ok(SecureData {
            encrypted_data: in_out,
            nonce: nonce_array,
            salt: gen_salt,
        })
    }

    pub fn decrypt(
        &self,
        secure_data: SecureData,
        password: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let encryption_key: EncryptionKey = self.derive_encryption_key(password, &secure_data.salt);
        let decryption_key: LessSafeKey = LessSafeKey::new(
            UnboundKey::new(&AES_256_GCM, &encryption_key.key)
                .map_err(|_| CryptoError::KeyDerivationFailed)?,
        );
        let nonce: Nonce = Nonce::assume_unique_for_key(secure_data.nonce);

        let mut in_out: Vec<u8> = secure_data.encrypted_data;
        let decrypted_data: &mut [u8] = decryption_key
            .open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|_| CryptoError::DecryptionFailed)?;

        Ok(decrypted_data.to_vec())
    }
}
pub struct EncryptionKey {
    pub(crate) key: Vec<u8>,
}

impl Zeroize for EncryptionKey {
    fn zeroize(&mut self) {
        self.key.zeroize();
    }
}

impl Drop for EncryptionKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecureData {
    pub(crate) encrypted_data: Vec<u8>,
    pub(crate) nonce: [u8; 12],
    pub(crate) salt: Vec<u8>,
}

impl Default for SecureData {
    // testing purposes
    fn default() -> Self {
        Self {
            encrypted_data: vec![],
            nonce: [0u8; 12],
            salt: vec![],
        }
    }
}