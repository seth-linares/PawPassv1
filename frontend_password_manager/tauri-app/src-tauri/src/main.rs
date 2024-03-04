#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use password_manager_backend::config::UserSettings;
use password_manager_backend::error_handling::CryptoError;
use password_manager_backend::password_entry::DecryptedPasswordEntry;
use password_manager_backend::storage::ApplicationData;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::path::app_data_dir;
use tauri::{command, State};
use tauri::Config;
use zeroize::Zeroize;

#[derive(Serialize, Debug)]
struct ErrorResponse {
    error: String,
}

// Utility to convert CryptoError to a JSON response
impl From<CryptoError> for ErrorResponse {
    fn from(error: CryptoError) -> Self {
        ErrorResponse {
            error: error.to_string(),
        }
    }
}

#[derive(Default)]
struct AppConfig {
    data_filename: String,
}

impl AppConfig {
    fn new() -> Self {
        AppConfig {
            data_filename: "pass_warden.json".into(),
        }
    }

    fn data_file_path(&self, config: &Config) -> Result<PathBuf, std::io::Error> {
        let app_data_path = app_data_dir(config).ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find app data directory",
        ))?;
        Ok(app_data_path.join(&self.data_filename))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SessionState {
    #[serde(skip_serializing)]
    mek: Vec<u8>,
    user_settings: UserSettings,
    password_entries: Vec<DecryptedPasswordEntry>,
}

impl SessionState {
    //***********only use on successful login!!!************
    fn new(app_data: ApplicationData, master_password: &[u8]) -> Self {
        let mek = match app_data.decrypt_mek_data(master_password) {
            Ok(mek) => mek,
            Err(_) => panic!("Failed to decrypt MEK data"),
        };
        SessionState {
            mek: mek.clone(),
            user_settings: app_data.user_settings,
            password_entries: app_data
                .password_entries
                .iter()
                .map(
                    |entry| match DecryptedPasswordEntry::new_from_password_entry(entry.clone(), &mek) {
                        Ok(decrypted_entry) => decrypted_entry,
                        Err(_) => panic!("Failed to decrypt password entry"),
                    },
                )
                .collect()
        }
    }


    fn add_decrypted_password_entry(&mut self, entry: DecryptedPasswordEntry) {
        match self.password_entries.iter_mut().find(|e| e.id == entry.id) {
            Some(existing_entry) => *existing_entry = entry,
            None => self.password_entries.push(entry),
        }
    }

    fn update_user_settings(&mut self, user_settings: UserSettings) {
        self.user_settings = user_settings;
    }

}

impl Default for SessionState {
    fn default() -> Self {
        SessionState {
            mek: vec![],
            user_settings: UserSettings::default(),
            password_entries: vec![],
        }
    }
}

impl Zeroize for SessionState {
    fn zeroize(&mut self) {
        self.mek.zeroize();
        self.password_entries.zeroize();
    }
}

impl Drop for SessionState {
    fn drop(&mut self) {
        self.zeroize();
    }
}

// Tauri command to check if application data exists and if it does, go to login screen, else call create_master_password (used for routing at startup)
#[command]
async fn check_application_data_existence(
    state: State<'_, AppConfig>,
    config: State<'_, Config>,
) -> Result<bool, ErrorResponse> {
    let app_data_path = state
        .data_file_path(config.inner())
        .map_err(|e| ErrorResponse::from(CryptoError::IO(e)))?;

    if app_data_path.exists() {
        let app_data = match ApplicationData::load_from_file(&app_data_path).await? {
            Some(data) => data,
            None => {
                
                return Ok(false)
            },
        };

        let hash_integrity: bool = app_data.verify_hashes();
        Ok(hash_integrity)
    } else {
        return Ok(false);
    }
}


// command to create the master password after finding the file doesn't exist
#[command]
async fn create_master_password(
    state: State<'_, AppConfig>,
    config: State<'_, Config>,
    password: String,
) -> Result<(), ErrorResponse> {
    let app_data_path = state
        .data_file_path(config.inner())
        .map_err(|e| ErrorResponse::from(CryptoError::IO(e)))?;
    let mut app_data = ApplicationData::new();

    match app_data.add_master_password_data(password.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(ErrorResponse::from(e)),
    }

    app_data
        .save_to_file(&app_data_path)
        .await
        .map_err(|e| ErrorResponse::from(e))?;
    Ok(())
}

/// command to change the master password (accessed through a button on the login screen). Need to verify old password first. We don't need to update session state here as we will not intialize the session until the user logs in with the new password. After setting the new password, the user will be redirected to the login screen.
#[command]
async fn change_master_password(
    state: State<'_, AppConfig>,
    config: State<'_, Config>,
    old_password: String,
    new_password: String,
) -> Result<(), ErrorResponse> {
    let app_data_path = state
        .data_file_path(config.inner())
        .map_err(|e| ErrorResponse::from(CryptoError::IO(e)))?;
    let mut app_data = match ApplicationData::load_from_file(&app_data_path).await? {
        Some(data) => data,
        None => {
            return Err(ErrorResponse {
                error: "Application data not found.".into(),
            })
        }
    };

    app_data
        .update_master_password_data(old_password.as_bytes(), new_password.as_bytes())
        .map_err(|e| ErrorResponse::from(e))?;
    app_data
        .save_to_file(&app_data_path)
        .await
        .map_err(|e| ErrorResponse::from(e))?;
    Ok(())
}

// command to handle logging in. This will not initialize the session but instead will be used before initializing the session to verify the password. If the password is correct, the session will be initialized and the user will be redirected to the main screen. If the password is incorrect, the user will be prompted to try again.
#[command]
async fn login(
    state: State<'_, AppConfig>,
    config: State<'_, Config>,
    session: State<'_, Mutex<SessionState>>,
    password: String,
) -> Result<(), ErrorResponse> {
    let app_data_path = state
        .data_file_path(config.inner())
        .map_err(|e| ErrorResponse::from(CryptoError::IO(e)))?;

    match ApplicationData::load_from_file(&app_data_path).await {
        Ok(Some(app_data)) => {
            // Verify the master password
            if app_data.verify_master_password(password.as_bytes()) {
                // On successful verification, directly initialize the session state
                let mut session_state = session.lock().map_err(|_| ErrorResponse {
                    error: "Failed to lock session state".into(),
                })?;
                *session_state = SessionState::new(app_data, password.as_bytes());

                Ok(())
            } else {
                Err(ErrorResponse {
                    error: "Invalid master password.".into(),
                })
            }
        },
        _ => Err(ErrorResponse {
            error: "Application data not found; Create a password.".into(),
        }),
    }
}

#[command]
async fn logout(
    session: State<'_, Mutex<SessionState>>) -> Result<(), ErrorResponse> {
    let mut session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    session_state.zeroize();
    Ok(())
}


#[command]
async fn get_password_entries(
    session: State<'_, Mutex<SessionState>>,
) -> Result<Vec<DecryptedPasswordEntry>, ErrorResponse> {
    let session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;


    


    Ok(session_state.password_entries.clone())
}


#[command]
async fn get_user_settings(
    session: State<'_, Mutex<SessionState>>,
) -> Result<UserSettings, ErrorResponse> {
    let session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    Ok(session_state.user_settings.clone())
}



#[command]
async fn add_password_entry(
    session: State<'_, Mutex<SessionState>>,
    password_entry: DecryptedPasswordEntry,
) -> Result<(), ErrorResponse> {
    let mut session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;

    session_state.add_decrypted_password_entry(password_entry);
    Ok(())
}

#[command]
fn get_password_entry(
    entry_id: String,
    session: State<'_, Mutex<SessionState>>,
) -> Result<DecryptedPasswordEntry, ErrorResponse> {
    
    let session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    match session_state.password_entries.iter().find(|e| e.id == entry_id) {
        Some(entry) => Ok(entry.clone()),
        None => Err(ErrorResponse {
            error: "Password entry not found".into(),
        }),
    }
}

#[command]

fn create_new_decrypted_password_entry() -> DecryptedPasswordEntry {
    DecryptedPasswordEntry::default()
}


#[command]
async fn update_user_settings(
    session: State<'_, Mutex<SessionState>>,
    user_settings: UserSettings,
) -> Result<(), ErrorResponse> {
    let mut session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    session_state.update_user_settings(user_settings);


    Ok(())
}


#[command]
fn generate_password(session: State<'_, Mutex<SessionState>>) -> Result<String, ErrorResponse> {
    let session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;

    Ok(session_state.user_settings.generate_password())
}

#[command]
async fn save_session_state(
    session: State<'_, Mutex<SessionState>>,
    state: State<'_, AppConfig>,
    config: State<'_, Config>,
) -> Result<(), ErrorResponse> {
    let app_data_path = state
        .data_file_path(config.inner())
        .map_err(|e| ErrorResponse::from(CryptoError::IO(e)))?;

    
    let (user_settings, mut password_entries, mut mek) = {
        let session_state = session.lock().map_err(|_| ErrorResponse {
            error: "Failed to lock session state".into(),
        })?;

        (session_state.user_settings.clone(), session_state.password_entries.clone(), session_state.mek.clone())
    };

    // Spawn a new task to save the session state to file because I want to run this each time we return our useSessionState hook and it could get called a lot
    tokio::spawn(async move {
        let app_data = match ApplicationData::load_from_file(&app_data_path).await {
            Ok(Some(mut data)) => {
                data.user_settings = user_settings;
                data.password_entries = password_entries
                    .iter()
                    .map(|entry| entry.convert_to_encrypted(&mek).unwrap())
                    .collect();
                (password_entries.zeroize(), mek.zeroize()); 
                let _ = data.update_hashes();
                data
            },
            Ok(None) => return Err(ErrorResponse {
                error: "Application data not found".into(),
            }),
            Err(e) => return Err(ErrorResponse::from(e)),
        };

        (password_entries.zeroize(), mek.zeroize());

        match app_data.save_to_file(&app_data_path).await {
            Ok(_) => {
                Ok(())
            },
            Err(e) => Err(ErrorResponse::from(e)),
        }
            
    });
    

    Ok(())
}

#[command]
fn get_categories(
    session: State<'_, Mutex<SessionState>>,
) -> Vec<String> {
    let session_state = match session.lock().map_err(|_| ErrorResponse {error: "Failed to lock session state".into(),}) {
        Ok(state) => state,
        Err(_) => return vec![]
    };

    let categories: HashSet<String> = session_state.password_entries.iter()
        .filter_map(|entry| entry.category.as_ref().map(|s| s.clone()))
        .collect();

    categories.into_iter().collect()
}

#[command]
fn get_favorites(
    session: State<'_, Mutex<SessionState>>,
) -> Vec<DecryptedPasswordEntry> {

    let session_state = match session.lock() {
        Ok(state) => state,
        Err(_) => return vec![],
    };

    let favorites: Vec<DecryptedPasswordEntry> = session_state.password_entries
        .iter()
        .filter(|entry| entry.favorite)
        .cloned()
        .collect();

    favorites
}

#[command]
fn get_session_state(
    session: State<'_, Mutex<SessionState>>,
) -> Result<SessionState, ErrorResponse> {
    let session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    Ok(session_state.clone())
}

#[command]
fn remove_password_entry(
    entry_id: String,
    session: State<'_, Mutex<SessionState>>,
) -> Result<(), ErrorResponse> {
    let mut session_state = session.lock().map_err(|_| ErrorResponse {
        error: "Failed to lock session state".into(),
    })?;
    session_state.password_entries.retain(|entry| entry.id != entry_id);
    Ok(())
}




fn main() {
    tauri::Builder::default()
        .manage(Config::default())
        .manage(AppConfig::new()) // Manage AppConfig state across commands
        .manage(Mutex::new(SessionState::default()))
        .invoke_handler(tauri::generate_handler![
            check_application_data_existence,
            create_master_password,
            change_master_password,
            login,
            logout,
            get_password_entries,
            get_user_settings,
            create_new_decrypted_password_entry,
            add_password_entry,
            update_user_settings,
            generate_password,
            save_session_state,
            get_password_entry,
            get_categories,
            get_favorites,
            get_session_state,
            remove_password_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
