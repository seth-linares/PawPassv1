pub mod config;
pub mod crypto;
pub mod error_handling;
pub mod master_password;
pub mod mek;
pub mod password_entry;
pub mod storage;

#[cfg(test)]
mod tests {
    mod storage_tests {
        use crate::config::UserSettings;
        use crate::master_password::MasterPasswordData;
        use crate::password_entry::PasswordEntry;
        use crate::storage::ApplicationData;
        use std::fs;

        #[test]
        fn test_add_master_password_data() {
            let mut app_data: ApplicationData = ApplicationData::new();

            assert!(app_data.add_master_password_data(b"password").is_ok());
            assert!(app_data.add_master_password_data(b"password").is_err());
        }

        #[test]
        fn test_update_master_password_data() {
            let mut app_data = ApplicationData {
                master_password_data: Some(
                    MasterPasswordData::set_new_master_password(b"bub").unwrap(),
                ),
                mek_data: None,
                password_entries: vec![],
                user_settings: UserSettings::default(),
                master_password_data_hash: None,
                mek_data_hash: None,
                password_entries_hash: None,
            };

            assert!(app_data
                .update_master_password_data(b"password", b"new_password")
                .is_err());
            assert!(app_data
                .update_master_password_data(b"wrong_password", b"new_password")
                .is_err());
            assert!(app_data
                .update_master_password_data(b"bub", b"new_password")
                .is_ok());
        }

        #[test]
        fn test_find_password_entry() {
            let password_entry = PasswordEntry::new(
                "title".to_string(),
                None,
                Some(b"password"),
                None,
                None,
                None,
                None,
                Some(b"master_password"),
            )
            .unwrap();
            let app_data = ApplicationData {
                master_password_data: None,
                mek_data: None,
                password_entries: vec![password_entry.clone()],
                user_settings: UserSettings::default(),
                master_password_data_hash: None,
                mek_data_hash: None,
                password_entries_hash: None,
            };

            assert!(app_data.find_password_entry(&password_entry).is_ok());
            assert!(app_data
                .find_password_entry(&PasswordEntry::default())
                .is_err());
        }

        #[test]
        fn test_add_password_entry() {
            let password_entry = PasswordEntry::default();
            let mut app_data = ApplicationData::new();

            assert!(app_data.add_password_entry(password_entry.clone()).is_ok());
            assert!(app_data.add_password_entry(password_entry).is_err());
        }

        #[test]
        fn test_remove_password_entry() {
            let password_entry = PasswordEntry::default();
            let mut app_data = ApplicationData {
                master_password_data: None,
                mek_data: None,
                password_entries: vec![password_entry.clone()],
                user_settings: UserSettings::default(),
                master_password_data_hash: None,
                mek_data_hash: None,
                password_entries_hash: None,
            };

            assert!(app_data
                .remove_password_entry(password_entry.clone())
                .is_ok());
            assert!(app_data.remove_password_entry(password_entry).is_err());
        }

        #[test]
        fn test_integrity_hash_updates() {
            let mut app_data = ApplicationData {
                master_password_data: Some(
                    MasterPasswordData::set_new_master_password(b"initial").unwrap(),
                ),
                mek_data: None,
                password_entries: vec![],
                user_settings: UserSettings::default(),
                master_password_data_hash: None,
                mek_data_hash: None,
                password_entries_hash: None,
            };

            // Initial update to generate hashes
            app_data.update_hashes().unwrap();
            let initial_master_hash = app_data.master_password_data_hash.clone();
            let initial_entries_hash = app_data.password_entries_hash.clone();

            // Add a new password entry and check that only the entries hash changes
            let password_entry = PasswordEntry::new(
                "title".to_string(),
                Some("username".to_string()),
                Some(b"password"),
                Some("url".to_string()),
                Some("notes".to_string()),
                Some("category".to_string()),
                Some(true),
                Some(b"master_password"),
            )
            .unwrap();
            app_data.add_password_entry(password_entry).unwrap();
            assert_ne!(app_data.password_entries_hash, initial_entries_hash);
            assert_eq!(app_data.master_password_data_hash, initial_master_hash);

            // Update master password data and check that only the master password hash changes
            app_data
                .update_master_password_data(b"initial", b"new_password")
                .unwrap();
            assert_ne!(app_data.master_password_data_hash, initial_master_hash);
            // Assuming password entries are unchanged, their hash should remain the same
            assert_ne!(app_data.password_entries_hash, initial_entries_hash);
        }

        #[tokio::test]
        async fn test_save_and_load_from_file() {
            let temp_file = tempfile::NamedTempFile::new().unwrap();
            let test_file_path = temp_file.path();

            // Prepare some test data
            let password_entry = PasswordEntry::default();
            let app_data = ApplicationData {
                master_password_data: Some(MasterPasswordData::default()),
                mek_data: None,
                password_entries: vec![password_entry.clone()],
                user_settings: UserSettings::default(),
                master_password_data_hash: None,
                mek_data_hash: None,
                password_entries_hash: None,
            };

            
            app_data.save_to_file(&test_file_path).await.unwrap();

            
            let loaded_app_data = ApplicationData::load_from_file(&test_file_path)
                .await
                .unwrap()
                .unwrap();
            println!("master_password_data");
            println!("{:?}", app_data.master_password_data);
            println!("{:?}", loaded_app_data.master_password_data);

            println!();

            println!("password_entries");
            println!("{:?}", app_data.password_entries);
            println!("{:?}", loaded_app_data.password_entries);

            println!();

            println!("user_settings");
            println!("{:?}", app_data.user_settings);
            println!("{:?}", loaded_app_data.user_settings);

            println!();

            println!("master_password_data_hash");
            println!("{:?}", app_data.master_password_data_hash);
            println!("{:?}", loaded_app_data.master_password_data_hash);

            println!();

            println!("password_entries_hash");
            println!("{:?}", app_data.password_entries_hash);
            println!("{:?}", loaded_app_data.password_entries_hash);

            // Clean up the test file
            fs::remove_file(&test_file_path).unwrap();
            //
            // Check that the loaded data is the same as the original
            assert_eq!(
                app_data.master_password_data.unwrap(),
                loaded_app_data.master_password_data.unwrap()
            );
            assert_eq!(app_data.password_entries, loaded_app_data.password_entries);
            assert_eq!(app_data.user_settings, loaded_app_data.user_settings);
            assert_eq!(
                app_data.master_password_data_hash,
                loaded_app_data.master_password_data_hash
            );
            assert_eq!(
                app_data.password_entries_hash,
                loaded_app_data.password_entries_hash
            );
        }

        #[test]
        fn test_add_master_password_successfully() {
            let mut app_data = ApplicationData::new(); 

            // Try adding a new master password where none exists.
            assert!(app_data
                .add_master_password_data(b"initial_master_password")
                .is_ok());
            // Ensure adding another master password fails since one already exists.
            assert!(app_data
                .add_master_password_data(b"another_master_password")
                .is_err());
        }

        #[test]
        fn test_update_master_password_successfully() {
            let mut app_data = ApplicationData::new();
            app_data
                .add_master_password_data(b"initial_master_password")
                .unwrap();

            // Attempt to update the master password with the correct current password.
            assert!(app_data
                .update_master_password_data(b"initial_master_password", b"new_master_password")
                .is_ok());
        }

        #[test]
        fn test_update_master_password_with_incorrect_password() {
            let mut app_data = ApplicationData::new();
            app_data
                .add_master_password_data(b"initial_master_password")
                .unwrap();

            // Attempt to update the master password with an incorrect current password.
            assert!(app_data
                .update_master_password_data(b"wrong_password", b"new_master_password")
                .is_err());
        }

        #[test]
        fn test_update_master_password_when_none_exists() {
            let mut app_data = ApplicationData::new();

            // Attempt to update the master password when none is set.
            assert!(app_data
                .update_master_password_data(b"any_password", b"new_master_password")
                .is_err());
        }
    }

    mod password_entry_tests {
        use crate::config::UserSettings;
        use crate::password_entry::{DecryptedPasswordEntry, PasswordEntry};

        #[test]
        fn test_password_entry_new() {
            let title = "Test Title".to_string();
            let username = Some("Test User".to_string());
            let password = Some("Test Password".as_bytes());
            let url = Some("https://test.com".to_string());
            let notes = Some("Test Notes".to_string());
            let category = Some("Test Category".to_string());
            let favorite = Some(true);
            let master_pass = Some("Master Password".as_bytes());

            let password_entry = PasswordEntry::new(
                title.clone(), 
                username.clone(),
                password,
                url.clone(),
                notes.clone(),
                category.clone(),
                favorite,
                master_pass,
            )
            .unwrap();

            assert_eq!(password_entry.title, title); 
            assert_eq!(password_entry.username, username);
            assert_eq!(password_entry.url, url);
            assert_eq!(password_entry.notes, notes);
            assert_eq!(password_entry.category, category);
            assert_eq!(password_entry.favorite, favorite.unwrap());
        }

        #[test]
        fn test_user_settings_generate_password() {
            let user_settings = UserSettings::default();
            let password = user_settings.generate_password();

            println!("password: {}", password);

            assert_eq!(password.len(), user_settings.password_length as usize);
        }
        #[test]
        fn test_encrypt_to_decrypt() {
            let example_pass = "password".to_string();
            let password_entry = PasswordEntry::new(
                "title".to_string(),
                Some("username".to_string()),
                Some(example_pass.as_bytes()),
                Some("url".to_string()),
                Some("notes".to_string()),
                Some("category".to_string()),
                Some(true),
                Some(example_pass.as_bytes()), 
            ).unwrap();

            let decrypted_password_entry = DecryptedPasswordEntry::new_from_password_entry(password_entry, example_pass.as_bytes()).ok().unwrap();
            assert!(decrypted_password_entry.password.is_some());
            assert_eq!(decrypted_password_entry.password.as_ref().unwrap(), "password");
        }
    }

    mod master_password_tests {
        use crate::master_password::MasterPasswordData;

        #[test]
        fn test_master_password_data_set_new_master_password() {
            let password = b"Test Password";
            let master_password_data = MasterPasswordData::set_new_master_password(password);

            assert!(master_password_data.is_ok());
        }

        #[test]
        fn test_master_password_data_verify_master_password() {
            let password = b"Test Password";
            let master_password_data =
                MasterPasswordData::set_new_master_password(password).unwrap();

            assert!(master_password_data.verify_master_password(password));
        }

        #[test]
        fn test_master_password_data_verify_master_password_wrong_password() {
            let password = b"Test Password";
            let wrong_password = b"Wrong Password";
            let master_password_data =
                MasterPasswordData::set_new_master_password(password).unwrap();

            assert!(!master_password_data.verify_master_password(wrong_password));
        }

        #[test]
        fn test_master_password_data_default() {
            let default_master_password_data = MasterPasswordData::default();

            assert_eq!(default_master_password_data.salt, vec![0u8; 16]);
            assert_eq!(default_master_password_data.password_hash, Vec::<u8>::new());
        }
    }

    mod crypto_tests {
        use crate::crypto::{Cryptographer, EncryptionKey, SecureData};
        use zeroize::Zeroize;

        #[test]
        fn test_encryption_key_zeroize() {
            let mut encryption_key = EncryptionKey { key: vec![1, 2, 3] };
            encryption_key.zeroize();

            assert_eq!(encryption_key.key, Vec::<u8>::new());
        }

        #[test]
        fn test_secure_data_default() {
            let secure_data = SecureData::default();

            assert_eq!(secure_data.encrypted_data, Vec::<u8>::new());
            assert_eq!(secure_data.nonce, [0u8; 12]);
            assert_eq!(secure_data.salt, Vec::<u8>::new());
        }

        #[test]
        fn test_cryptographer_new() {
            let cryptographer = Cryptographer::new(Some(1000));

            assert_eq!(cryptographer.iterations.get(), 1000);
        }

        #[test]
        fn test_cryptographer_generate_salt() {
            let salt = Cryptographer::generate_salt();

            assert!(salt.is_ok());
            assert_eq!(salt.unwrap().len(), 16);
        }

        #[test]
        fn test_cryptographer_generate_nonce() {
            let nonce = Cryptographer::generate_nonce();

            assert!(nonce.is_ok());
            assert_eq!(nonce.unwrap().len(), 12);
        }

        #[test]
        fn test_cryptographer_encrypt_decrypt() {
            let cryptographer = Cryptographer::new(Some(100_000));
            let data = b"Test Data";
            let password = b"Test Password";

            let secure_data = cryptographer.encrypt(data, password).unwrap();
            let decrypted_data = cryptographer.decrypt(secure_data, password).unwrap();

            assert_eq!(decrypted_data, data);
        }

        #[test]
        fn test_decrypt_with_incorrect_password() {
            let cryptographer = Cryptographer::new(None);
            let password = b"my_master_password";
            let wrong_password = b"wrong_password";
            let data = b"Sensitive data here";

            let encrypted_data = cryptographer.encrypt(data, password).unwrap();
            let decrypted_result = cryptographer.decrypt(encrypted_data, wrong_password);

            assert!(decrypted_result.is_err());
        }
    }

    mod mek_tests {

        use crate::{crypto::Cryptographer, error_handling::CryptoError, mek::MekData, storage::ApplicationData};

        #[test]
        fn test_mek_data_new_1() {
            let master_password = b"Test Master Password";
            let mek_data = MekData::new(master_password);

            assert!(mek_data.is_ok());
        }

        #[test]
        fn test_mek_data_decrypt_mek() {
            let master_password = b"Test Master Password";
            let mek_data = MekData::new(master_password).unwrap();
            let decrypted_mek = mek_data.decrypt_mek(master_password);

            assert!(decrypted_mek.is_ok());
        }

        #[test]
        fn test_mek_data_derive_mek_key() {
            let master_password = b"Test Master Password";
            let salt = vec![1, 2, 3];
            let derived_key = MekData::derive_mek_key(master_password, &salt);

            assert_eq!(derived_key.len(), 32);
        }

        #[test]
        fn test_mek_data_new() {
            let master_password = "password".as_bytes();
            let mek_data = MekData::new(master_password)
                .map_err(|e| println!("{}", e.to_string()))
                .unwrap();
            println!("{:?}", mek_data);
            assert_eq!(mek_data.mek_salt.len(), 16);
            assert_ne!(mek_data.encrypted_mek.encrypted_data, master_password);
        }

        #[test]
        fn test_decrypt_mek() {
            let master_password = "password".as_bytes();
            let mek_data = MekData::new(master_password).unwrap();
            let decrypted_mek = mek_data.decrypt_mek(master_password).unwrap();

            println!("Decrypted mek: {:?}", decrypted_mek);

            

            // Create a new Cryptographer and use it to encrypt the decrypted MEK
            let cryptographer = Cryptographer::new(None);
            let derived_key = MekData::derive_mek_key(master_password, &mek_data.mek_salt);
            let re_encrypted_mek = cryptographer.encrypt(&decrypted_mek, &derived_key).unwrap();
            let decrypted_mek_2 = cryptographer.decrypt(re_encrypted_mek.clone(), &derived_key).unwrap();

            println!("Decrypted mek: {:?}", decrypted_mek_2);

            let derived_key_2 = MekData::derive_mek_key(master_password, &mek_data.mek_salt);
            let re_encrypted_mek_2 = cryptographer.encrypt(&decrypted_mek_2, &derived_key_2).unwrap();
            let decrypted_mek_3 = cryptographer.decrypt(re_encrypted_mek_2.clone(), &derived_key_2).unwrap();

            println!("Decrypted mek: {:?}", decrypted_mek_3);

            assert_eq!(
                decrypted_mek,
                decrypted_mek_2
            );

            assert_eq!(
                decrypted_mek_2,
                decrypted_mek_3
            );
        }

        #[test]
        fn test_decrypt_reencrypt_mek() {
            let master_password = "password".as_bytes();

            // Generate a new MEK and encrypt it
            let mek_data = MekData::new(master_password).unwrap();

            // Decrypt the MEK
            let decrypted_mek = mek_data.decrypt_mek(master_password).unwrap();
            println!("Decrypted MEK: {:?}", decrypted_mek);

            // Re-encrypt the decrypted MEK using the same master password
            let re_encrypted_mek_data = mek_data.reencrypt_mek(master_password, &decrypted_mek).unwrap();

            // Decrypt the re-encrypted MEK to verify it matches the original decryption
            let cryptographer = Cryptographer::new(None);
            let derived_key = MekData::derive_mek_key(master_password, &mek_data.mek_salt);
            let decrypted_re_encrypted_mek = cryptographer.decrypt(re_encrypted_mek_data.encrypted_mek, &derived_key).unwrap();
            println!("Decrypted re-encrypted MEK: {:?}", decrypted_re_encrypted_mek);

            // Assert that the original decrypted MEK matches the decrypted re-encrypted MEK
            assert_eq!(decrypted_mek, decrypted_re_encrypted_mek, "The decrypted MEK should match the decrypted re-encrypted MEK.");
        }

        #[test]
        fn test_update_mek() {
            let old_master_password = b"old_password";
            let new_master_password = b"new_password";
            let mut mek_data = MekData::new(old_master_password).unwrap();

            let original_decrypted_mek = mek_data.decrypt_mek(old_master_password).unwrap();
            // Update the MEK
            let _ = mek_data.update_mek(old_master_password, new_master_password).unwrap();

            // Try to decrypt the MEK with the old password (should fail)
            assert!(mek_data.decrypt_mek(old_master_password).is_err());

            // Decrypt the MEK with the new password (should succeed)
            let decrypted_mek = mek_data.decrypt_mek(new_master_password).unwrap();

            println!("Decrypted MEK: {:?}", original_decrypted_mek);
            println!("Decrypted MEK: {:?}", decrypted_mek);

            assert_eq!(original_decrypted_mek, decrypted_mek, "The decrypted MEK should match the original decrypted MEK.");

            // assert that the decrypt with the new password works

            // Create a new Cryptographer and use it to encrypt the decrypted MEK
            let cryptographer = Cryptographer::new(None);
            let derived_key = MekData::derive_mek_key(new_master_password, &mek_data.mek_salt);
            let re_encrypted_mek = cryptographer.encrypt(&decrypted_mek, &derived_key).unwrap();
            let decrypted_mek_2 = cryptographer.decrypt(re_encrypted_mek.clone(), &derived_key).unwrap();

            assert_eq!(decrypted_mek, decrypted_mek_2);
        }

        #[test]
        fn test_update_mek_data() {
            let old_master_password = b"old_password";
            let new_master_password = b"new_password";
            let wrong_password = b"wrong_password";
            let mut storage = ApplicationData::new();

            let _ = storage.add_master_password_data(old_master_password);
            

            // Attempt to update the MEK with a wrong password (should fail)
            assert!(storage.update_mek_data(wrong_password, new_master_password).is_err());

            // Update the MEK with the correct old password (should succeed)
            assert!(storage.update_master_password_data(old_master_password, new_master_password).is_ok());


        }

        #[test]
        fn test_mek_operations_2() {
            let password1 = b"password1";
            let password2 = b"password2";
            let password3 = b"password3";

            let mut app_data = ApplicationData::new();
            app_data.add_master_password_data(password1).unwrap();

            let mek_data_password1 = app_data.decrypt_mek_data(password1).unwrap();
            println!("MEK data: {:?}", mek_data_password1);

            app_data.update_master_password_data(password1, password2).unwrap();

            println!("{}", app_data.verify_master_password(password1));
            println!("{}", app_data.verify_master_password(password2));
            
            let mek_data_password2 = app_data.decrypt_mek_data(password1).map_err(|e: CryptoError| println!("{}", e.to_string()));
            println!("MEK data: {:?}", mek_data_password2);
            let mek_data_password3 = app_data.decrypt_mek_data(password2).map_err(|e| println!("{}", e.to_string())).unwrap();
            println!("MEK data: {:?}", mek_data_password3);

            let _ = app_data.update_master_password_data(password2, password3).map_err(|e| println!("{}", e.to_string()));
            println!("{}", app_data.verify_master_password(password2));
            println!("{}", app_data.verify_master_password(password3));


            let mek_data_password4 = app_data.decrypt_mek_data(password2).map_err(|e| println!("{}", e.to_string()));
            println!("MEK data: {:?}", mek_data_password4);
            let mek_data_password5 = app_data.decrypt_mek_data(password3).map_err(|e| println!("{}", e.to_string())).unwrap();
            println!("MEK data: {:?}", mek_data_password5);
            

        }

    }


}