// src/utils/api.ts

import { invoke } from '@tauri-apps/api/tauri';
import { DecryptedPasswordEntry, UserSettings, SessionState } from '../hooks/useSession';

/**
 * Checks if the application data file exists and verifies hashes.
 */
export const checkApplicationDataExistence = async (): Promise<boolean> => {
  return await invoke('check_application_data_existence');
};

/**
 * Creates a new master password and initializes application data.
 * @param password The new master password to be set.
 */
export const createMasterPassword = async (password: string): Promise<void> => {
  return await invoke('create_master_password', { password });
};

/**
 * Changes the master password.
 * @param oldPassword The current master password.
 * @param newPassword The new master password to be set.
 */
export const changeMasterPassword = async (oldPassword: string, newPassword: string): Promise<void> => {
  return await invoke('change_master_password', { oldPassword, newPassword });
};

/**
 * Attempts to log in with the provided master password.
 * @param password The master password for authentication.
 */
export const login = async (password: string): Promise<void> => {
  return await invoke('login', { password });
};

/**
 * Logs out the current user and clears the session state.
 */
export const logout = async (): Promise<void> => {
  return await invoke('logout');
};

/**
 * Fetches all password entries from the server.
 * @returns {Promise<Array<DecryptedPasswordEntry>>} A promise that resolves to an array of decrypted password entries.
 */
export const getPasswordEntries = async (): Promise<DecryptedPasswordEntry[]> => {
  return await invoke('get_password_entries');
};


/**
 * Fetches the user settings from the server.
 * @returns {Promise<UserSettings>} A promise that resolves to the user settings.
 */
export const getUserSettings = async (): Promise<UserSettings> => {
  return await invoke('get_user_settings');
};


/**
 * Adds a new password entry.
 * @param {DecryptedPasswordEntry} passwordEntry The password entry to add.
 * @returns {Promise<void>} A promise that resolves when the password entry has been added.
 */
export const addPasswordEntry = async (passwordEntry: DecryptedPasswordEntry): Promise<void> => {
  return await invoke('add_password_entry', { passwordEntry });
};


/**
 * Updates the user settings.
 * @param {UserSettings} userSettings The new user settings.
 * @returns {Promise<void>} A promise that resolves when the user settings have been updated.
 */
export const updateUserSettings = async (userSettings: UserSettings): Promise<void> => {
  return await invoke('update_user_settings', { userSettings });
};


/**
 * Generates a password based on the current user settings.
 * @returns {Promise<string>} A promise that resolves to the generated password.
 */
export const generatePassword = async (): Promise<string> => {
  return await invoke('generate_password');
};

/**
 * Saves the current session state.
 * @param userSettings The current user settings.
 * @param passwordEntries The current password entries.
 * @returns {Promise<string>} A promise that resolves when the session state has been saved.
 */
export const saveSessionState = async (
  userSettings: UserSettings,
  passwordEntries: DecryptedPasswordEntry[]
): Promise<string> => {
  return await invoke('save_session_state', {userSettings, passwordEntries});
};

/**
 * creates a new decrypted password entry.
 * @returns {Promise<DecryptedPasswordEntry>} A promise that resolves to the new decrypted password entry.
 * */

export function createNewDecryptedPasswordEntry(): Promise<DecryptedPasswordEntry> {
  return invoke('create_new_decrypted_password_entry');
}

/**
 * Gets a password entry by its ID.
 * @param entryId The ID of the password entry to fetch.
 * @returns {Promise<DecryptedPasswordEntry>} A promise that resolves to the decrypted password entry.
 */

export function getPasswordEntry(entryId: string): Promise<DecryptedPasswordEntry> {
  return invoke('get_password_entry', { entryId });
}

/**
 * Gets all unique categories from the password entries.
 * @returns {Promise<string[]>} A promise that resolves to the array of categories.
 */
export function getCategories(): Promise<string[]> {
  return invoke('get_categories');
}

/**
 * Gets all favorite password entries.
 * @returns {Promise<DecryptedPasswordEntry[]>} A promise that resolves to the array of favorite password entries.
 */

export function getFavorites(): Promise<DecryptedPasswordEntry[]> {
  return invoke('get_favorites');
}

/**
 * Fetches the current session state from the server.
 * @returns {Promise<SessionState>} A promise that resolves to the session state.
 */
export const getSessionState = async (): Promise<SessionState> => {
  return await invoke('get_session_state');
};

/**
 * Removes a password entry by its ID.
 * @param entryId The ID of the password entry to remove.
 * @returns {Promise<void>} A promise that resolves when the password entry has been removed.
 */

export const removePasswordEntry = async (entryId: string): Promise<void> => {
  return await invoke('remove_password_entry', { entryId });
}