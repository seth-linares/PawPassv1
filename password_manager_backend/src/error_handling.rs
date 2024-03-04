use ring::error::Unspecified;

#[derive(Debug)]
pub enum CryptoError {
    KeyDerivationFailed,
    EncryptionFailed,
    DecryptionFailed,
    NonceGenerationFailed,
    SaltGenerationFailed,
    IO(std::io::Error),
    Serde(serde_json::Error),
    InvalidInput,
    PasswordEntryNotFound,
    PasswordEntryAlreadyExists,
    MasterPasswordDataNotFound,
    MasterPasswordDataAlreadyExists,
    AuthenticationFailed,
    CryptoOperationFailed,
    MekGenerationFailed,
    MekDataAlreadyExists,
    MekUpdateFailed,
    MekDecryptionFailed,
    MekHashVerificationFailed,
    MekDataNotPresent,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::KeyDerivationFailed => write!(f, "Key derivation failed."),
            CryptoError::EncryptionFailed => write!(f, "Encryption process failed."),
            CryptoError::DecryptionFailed => write!(f, "Decryption process failed."),
            CryptoError::NonceGenerationFailed => write!(f, "Nonce generation failed."),
            CryptoError::SaltGenerationFailed => write!(f, "Salt generation failed."),
            CryptoError::IO(e) => write!(f, "IO error: {}", e),
            CryptoError::Serde(e) => write!(f, "Serialization/deserialization error: {}", e),
            CryptoError::InvalidInput => write!(f, "Invalid input."),
            CryptoError::PasswordEntryNotFound => write!(f, "Password entry not found."),
            CryptoError::PasswordEntryAlreadyExists => write!(f, "Password entry already exists."),
            CryptoError::MasterPasswordDataNotFound => write!(f, "Master password not found."),
            CryptoError::MasterPasswordDataAlreadyExists => {
                write!(f, "Master password already exists!")
            }
            CryptoError::AuthenticationFailed => {
                write!(f, "Authentication invalid or not accepted.")
            }
            CryptoError::CryptoOperationFailed => write!(f, "Cryptographic operation failed."),
            CryptoError::MekGenerationFailed => write!(f, "MEK generation failed."),
            CryptoError::MekDataAlreadyExists => write!(f, "MEK data already exists."),
            CryptoError::MekUpdateFailed => write!(f, "MEK update failed."),
            CryptoError::MekDecryptionFailed => write!(f, "MEK decryption failed."),
            CryptoError::MekHashVerificationFailed => write!(f, "MEK hash verification failed."),
            CryptoError::MekDataNotPresent => write!(f, "MEK data not present."),
        }
    }
}

impl From<std::io::Error> for CryptoError {
    fn from(error: std::io::Error) -> Self {
        CryptoError::IO(error)
    }
}

impl From<serde_json::Error> for CryptoError {
    fn from(error: serde_json::Error) -> Self {
        CryptoError::Serde(error)
    }
}

impl From<Unspecified> for CryptoError {
    fn from(_: Unspecified) -> Self {
        CryptoError::CryptoOperationFailed
    }
}