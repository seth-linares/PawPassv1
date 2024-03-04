use crate::error_handling::CryptoError;
use ring::{
    digest,
    pbkdf2::{derive, verify, PBKDF2_HMAC_SHA256},
    rand::{SecureRandom, SystemRandom},
};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16; // Salt length in bytes
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN; // Hash output length

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterPasswordData {
    pub(crate) salt: Vec<u8>,
    pub(crate) password_hash: Vec<u8>,
}
impl MasterPasswordData {
    pub fn set_new_master_password(password: &[u8]) -> Result<Self, CryptoError> {
        let rng: SystemRandom = SystemRandom::new();
        let mut salt: Vec<u8> = vec![0u8; SALT_LEN];
        rng.fill(&mut salt)?;

        let mut password_hash: Vec<u8> = vec![0u8; CREDENTIAL_LEN];
        derive(
            PBKDF2_HMAC_SHA256,
            NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
            &salt,
            password,
            &mut password_hash,
        );

        Ok(MasterPasswordData {
            salt,
            password_hash,
        })
    }

    pub fn verify_master_password(&self, password_attempt: &[u8]) -> bool {
        verify(
            PBKDF2_HMAC_SHA256,
            NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
            &self.salt,
            password_attempt,
            &self.password_hash,
        )
        .is_ok()
    }
}

impl PartialEq for MasterPasswordData {
    fn eq(&self, other: &Self) -> bool {
        self.salt == other.salt && self.password_hash == other.password_hash
    }
}
impl Eq for MasterPasswordData {}

impl Default for MasterPasswordData {
    fn default() -> Self {
        MasterPasswordData {
            salt: vec![0u8; SALT_LEN],

            password_hash: vec![],
        }
    }
}