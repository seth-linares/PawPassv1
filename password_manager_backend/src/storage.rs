use crate::config::UserSettings;
use crate::error_handling::CryptoError;
use crate::master_password::MasterPasswordData;
use crate::mek::MekData;
use crate::password_entry::PasswordEntry;
use ring::digest::Digest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationData {
    pub master_password_data: Option<MasterPasswordData>,
    pub mek_data: Option<MekData>,
    pub password_entries: Vec<PasswordEntry>,
    pub user_settings: UserSettings,
    pub master_password_data_hash: Option<String>,
    pub mek_data_hash: Option<String>,
    pub password_entries_hash: Option<String>,
    
}

impl ApplicationData {
    pub fn new() -> Self {
        Self {
            master_password_data: None,
            mek_data: None,
            password_entries: Vec::new(),
            user_settings: UserSettings::default(),
            master_password_data_hash: None,
            mek_data_hash: None,
            password_entries_hash: None,
        }
    }

    pub fn add_mek_data(&mut self, master_password: &[u8]) -> Result<(), CryptoError> {
        if self.mek_data.is_none() {
            self.mek_data = Some(MekData::new(master_password)?);
            self.update_hashes()?;
        } else {
            return Err(CryptoError::MekDataAlreadyExists);
        }
        Ok(())
    }

    pub fn update_mek_data(&mut self, old_master_password: &[u8], new_master_password: &[u8],) -> Result<(), CryptoError> {
        if let Some(mek_data) = &mut self.mek_data {
            if mek_data.decrypt_mek(old_master_password).is_ok() {
                self.mek_data = Some(mek_data.update_mek(old_master_password, new_master_password)?);
                self.update_hashes()?;
                Ok(())
            } else {
                Err(CryptoError::AuthenticationFailed)
            }
        } else {
            Err(CryptoError::MekUpdateFailed)
        }
    }

    pub fn decrypt_mek_data(&self, master_password: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if !self.verify_master_password(master_password) {
            return Err(CryptoError::AuthenticationFailed);
        }
        if let Some(mek_data) = &self.mek_data {
            mek_data.decrypt_mek(master_password)
        } else {
            Err(CryptoError::MekDecryptionFailed)
        }
    }

    pub fn verify_hashes(&self) -> bool {
        let master_data_serialized = serde_json::to_string(&self.master_password_data).unwrap();
        let mek_data_serialized = serde_json::to_string(&self.mek_data).unwrap();
        let entries_serialized = serde_json::to_string(&self.password_entries).unwrap();

        let master_data_hash = Self::generate_hash(master_data_serialized.as_bytes());
        let mek_data_hash = Self::generate_hash(mek_data_serialized.as_bytes());
        let entries_hash = Self::generate_hash(entries_serialized.as_bytes());


        self.master_password_data_hash == Some(master_data_hash)
            && self.mek_data_hash == Some(mek_data_hash)
            && self.password_entries_hash == Some(entries_hash)
    }

    pub fn add_master_password_data(&mut self, new_password: &[u8]) -> Result<(), CryptoError> {
        if self.master_password_data.is_none() {
            self.master_password_data =
                Some(MasterPasswordData::set_new_master_password(new_password)?);
            self.add_mek_data(new_password)?;
            self.update_hashes()?;
        } else {
            return Err(CryptoError::MasterPasswordDataAlreadyExists);
        }
        Ok(())
    }

    pub fn update_master_password_data(&mut self, password_attempt: &[u8], new_password: &[u8]) -> Result<(), CryptoError> {
        if let Some(master_data) = &self.master_password_data {
            if master_data.verify_master_password(password_attempt) {
                self.master_password_data =
                    Some(MasterPasswordData::set_new_master_password(new_password)?);
                let _ = self.update_mek_data(password_attempt, new_password);
                self.update_hashes()?;
                Ok(())
            } else {
                Err(CryptoError::AuthenticationFailed)
            }
        } else {
            Err(CryptoError::MasterPasswordDataNotFound)
        }
    }

    
    pub fn verify_master_password(&self, password_attempt: &[u8]) -> bool {
        if let Some(master_data) = &self.master_password_data {
            master_data.verify_master_password(password_attempt)
        } else {
            false
        }
    }


    pub fn search_password_entries(&self, search_term: &str) -> Vec<PasswordEntry> {
        self.password_entries
            .iter()
            .filter(|e| {
                e.title.contains(search_term)
                    || e.url.as_ref().map_or(false, |u| u.contains(search_term))
            })
            .cloned()
            .collect()
    }

    pub fn find_password_entry(
        &self,
        password_entry: &PasswordEntry,
    ) -> Result<usize, CryptoError> {
        match self
            .password_entries
            .iter()
            .position(|e| e == password_entry)
        {
            Some(index) => Ok(index),
            None => Err(CryptoError::PasswordEntryNotFound),
        }
    }

    pub fn add_password_entry(&mut self, entry: PasswordEntry) -> Result<(), CryptoError> {
        if self.find_password_entry(&entry).is_err() {
            self.password_entries.push(entry);
            self.update_hashes()?;
            Ok(())
        } else {
            Err(CryptoError::PasswordEntryAlreadyExists)
        }
    }

    pub fn update_password_entry(&mut self, updated_entry: &PasswordEntry) -> Result<(), CryptoError> {
        let index = self.find_password_entry(&updated_entry)?;
        self.password_entries[index] = updated_entry.clone();
        self.update_hashes()?;
        Ok(())
    }

    pub fn remove_password_entry(&mut self, password_entry: PasswordEntry) -> Result<(), CryptoError> {
        let index = self.find_password_entry(&password_entry)?;
        self.password_entries.remove(index);
        self.update_hashes()?;
        Ok(())
    }


    pub fn update_hashes(&mut self) -> Result<(), CryptoError> {
        let master_data_serialized =
            serde_json::to_string(&self.master_password_data).map_err(CryptoError::Serde)?;
        self.master_password_data_hash =
            Some(Self::generate_hash(master_data_serialized.as_bytes()));

        let mek_data_serialized =
            serde_json::to_string(&self.mek_data).map_err(CryptoError::Serde)?;
        self.mek_data_hash = Some(Self::generate_hash(mek_data_serialized.as_bytes()));

        let entries_serialized =
            serde_json::to_string(&self.password_entries).map_err(CryptoError::Serde)?;


        self.password_entries_hash = Some(Self::generate_hash(entries_serialized.as_bytes()));


        Ok(())
    }

        


    fn generate_hash(data: &[u8]) -> String {
        let hash: Digest = ring::digest::digest(&ring::digest::SHA256, data);
        hex::encode(hash.as_ref())
    }

    pub async fn save_to_file(&self, file_path: &Path) -> Result<(), CryptoError> {
        let serialized: String = serde_json::to_string(self).map_err(CryptoError::Serde)?;
        let mut file: File = File::create(file_path).await.map_err(CryptoError::IO)?;
        file.write_all(serialized.as_bytes())
            .await
            .map_err(CryptoError::IO)?;
        Ok(())
    }

    pub async fn load_from_file(file_path: &Path) -> Result<Option<Self>, CryptoError> {
        let mut file = match File::open(file_path).await {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(CryptoError::IO(e)),
        };

        let mut content = String::new();
        file.read_to_string(&mut content)
            .await
            .map_err(|e| CryptoError::IO(e))?;
        let deserialized: ApplicationData =
            serde_json::from_str(&content).map_err(|e| CryptoError::Serde(e))?;

        Ok(Some(deserialized))
    }

    pub fn update_user_settings(&mut self, new_settings: UserSettings) -> Result<(), CryptoError> {
        self.user_settings = new_settings;
        self.update_hashes()?;
        Ok(())
    }

    pub fn get_user_settings(&self) -> &UserSettings {
        &self.user_settings
    }

    pub fn generate_password(&mut self) -> String {
        let user_settings: &mut UserSettings = &mut self.user_settings;
        user_settings.generate_password()
    }
}


