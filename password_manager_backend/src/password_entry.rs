use crate::crypto::{Cryptographer, SecureData};
use crate::error_handling::CryptoError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zeroize::Zeroize;



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub username: Option<String>,
    pub password: Option<SecureData>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub creation_date: String,
    pub category: Option<String>,
    pub favorite: bool,
}

impl PasswordEntry {
    pub fn new(
        title_: String,
        username_: Option<String>,
        password_: Option<&[u8]>,
        url_: Option<String>,
        notes_: Option<String>,
        category_: Option<String>,
        favorite_: Option<bool>,
        mek: Option<&[u8]>,
    ) -> Result<Self, CryptoError> {
        let crypto_bub: Cryptographer = Cryptographer::new(None);

        let result = match (password_, mek) {
            (Some(password_), Some(mek)) => {
                Some(Cryptographer::encrypt(&crypto_bub, password_, mek)?)
            }
            _ => None,
        };

        Ok(Self {
            id: Uuid::new_v4().to_string(),
            title: title_,
            username: username_,
            password: result,
            url: url_,
            notes: notes_,
            creation_date: Utc::now().to_string(),
            category: category_,
            favorite: favorite_.unwrap_or(false),
        })
    }

    pub fn to_decrypted(&self, mek: &[u8]) -> Result<DecryptedPasswordEntry, CryptoError> {
        DecryptedPasswordEntry::new_from_password_entry(self.clone(), mek)
    }

    pub fn display_name(&self) -> [String; 3] {
        [
            self.title.clone(),
            self.username.as_deref().unwrap_or_default().to_string(),
            self.url.as_deref().unwrap_or_default().to_string(),
        ]
    }

    pub fn add_password(&self, data: &[u8], mek: &[u8]) -> Result<Self, String> {
        let crypto_bub: Cryptographer = Cryptographer::new(None);
        match Cryptographer::encrypt(&crypto_bub, data, mek) {
            Ok(data) => Ok(Self {
                id: self.id.clone(),
                title: self.title.clone(),
                username: self.username.clone(),
                password: Some(data),
                url: self.url.clone(),
                notes: self.notes.clone(),
                creation_date: self.creation_date.clone(),
                category: self.category.clone(),
                favorite: self.favorite,
            }),
            Err(e) => Err(format!("Failed to add encrypted password; {}", e)),
        }
    }
}

impl PartialEq for PasswordEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PasswordEntry {}

impl Default for PasswordEntry {
    // testing purposes
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            username: None,
            password: None,
            url: None,
            notes: None,
            creation_date: Utc::now().to_string(),
            category: None,
            favorite: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecryptedPasswordEntry {
    pub id: String,
    pub title: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub creation_date: String,
    pub category: Option<String>,
    pub favorite: bool,
}

impl DecryptedPasswordEntry {
    
    pub fn new(
        title_: String,
        username_: Option<String>,
        password_: Option<String>,
        url_: Option<String>,
        notes_: Option<String>,
        category_: Option<String>,
        favorite_: Option<bool>,
    ) -> Result<Self, CryptoError> {


        Ok(Self {
            id: Uuid::new_v4().to_string(),
            title: title_,
            username: username_,
            password: match password_ { Some(password) => Some(password), None => None },
            url: url_,
            notes: notes_,
            creation_date: Utc::now().to_string(),
            category: category_,
            favorite: favorite_.unwrap_or(false),
        })
    }
    
    
    pub fn new_from_password_entry(entry: PasswordEntry, mek: &[u8]) -> Result<Self, CryptoError> {
        let cryptographer = Cryptographer::new(None);
        let decrypted_password = match entry.password {
            Some(ref password) => {
                let result = cryptographer.decrypt(password.clone(), mek);
                Some(result?)
            },
            None => None,
        };
        Ok(Self {
            id: entry.id,
            title: entry.title,
            username: entry.username,
            password: decrypted_password.map(|p| match String::from_utf8(p) { Ok(s) => s, Err(_) => String::new() }),
            url: entry.url,
            notes: entry.notes,
            creation_date: entry.creation_date,
            category: entry.category,
            favorite: entry.favorite,
        })
    }

    pub fn convert_to_encrypted(&self, mek: &[u8]) -> Result<PasswordEntry, CryptoError> {
        let cryptographer = Cryptographer::new(None);
        let encrypted_password = match self.password {
            Some(ref password) => Some(cryptographer.encrypt(password.as_bytes(), mek)?),
            None => None,
        };
        Ok(PasswordEntry {
            id: self.id.clone(),
            title: self.title.clone(),
            username: self.username.clone(),
            password: encrypted_password,
            url: self.url.clone(),
            notes: self.notes.clone(),
            creation_date: self.creation_date.clone(),
            category: self.category.clone(),
            favorite: self.favorite,
        })
    }

    pub fn display_name(&self) -> [String; 3] {
        [
            self.title.clone(),
            self.username.as_deref().unwrap_or_default().to_string(),
            self.url.as_deref().unwrap_or_default().to_string(),
        ]
    }
}

impl Default for DecryptedPasswordEntry {
    // testing purposes
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::new(),
            username: None,
            password: None,
            url: None,
            notes: None,
            creation_date: Utc::now().to_string(),
            category: None,
            favorite: false,
        }
    }
}


impl PartialEq for DecryptedPasswordEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DecryptedPasswordEntry {}

impl Zeroize for DecryptedPasswordEntry {
    fn zeroize(&mut self) {
        self.password.zeroize();
    }
}

impl Drop for DecryptedPasswordEntry {
    fn drop(&mut self) {
        self.zeroize();
    }
}