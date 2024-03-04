// src/hooks/useSession.ts
import { useState, useEffect, useCallback } from 'react';
import {
    updateUserSettings as apiUpdateUserSettings,
    addPasswordEntry as apiAddPasswordEntry,
    generatePassword as apiGeneratePassword,
    saveSessionState as apiSaveSessionState,
    getSessionState as apiGetSessionState,
    removePasswordEntry as apiRemovePasswordEntry,
} from '../utils/api';

export interface UserSettings {
    passwordLength: number;
    minPasswordLength: number;
    useNum: boolean;
    minNum: number; 
    useSymbol: boolean;
    minSymbol: number;
    useLower: boolean;
    useUpper: boolean;
}

export interface DecryptedPasswordEntry {
    id: string; // uuid string (not set by user)
    title: string;
    username?: string;
    password?: string;
    url?: string;
    notes?: string;
    creationDate: string; // date string (not set by user)
    category?: string;
    favorite: boolean;
}

export interface SessionState {
    userSettings: UserSettings;
    passwordEntries: DecryptedPasswordEntry[];
}

const defaultUserSettings: UserSettings = {
    passwordLength: 14,
    minPasswordLength: 10,
    useNum: true,
    minNum: 2,
    useSymbol: true,
    minSymbol: 2,
    useLower: true,
    useUpper: true,
};

export const useSession = () => {
    const [sessionState, setSessionState] = useState<SessionState>({
        userSettings: defaultUserSettings,
        passwordEntries: [],
    });

    const saveSession = async () => {
        console.log('Saving session state');
        try {
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
            console.log('Sending user settings:', sessionState.userSettings);
            console.log('Sending password entries:', sessionState.passwordEntries);
            await apiSaveSessionState(sessionState.userSettings, sessionState.passwordEntries);
            console.log('Session state saved successfully');
        } catch (error) {
            console.error('Error saving session state:', error);
        }
    };

    const fetchSessionData = useCallback(async () => {
        console.log('Fetching session data');
        try {
            const sessionState = await apiGetSessionState(); 
            setSessionState(sessionState); 
            console.log('Session data fetched successfully');
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
            return Promise.resolve(sessionState);
        } catch (error) {
            console.error('Error fetching session data:', error);
            return Promise.reject(error);
        }
    }, []);

    const updateUserSettings = async (newSettings: UserSettings) => {
        console.log('Updating user settings', newSettings);
        try {
            await apiUpdateUserSettings(newSettings);
            setSessionState((prevState) => ({
                ...prevState,
                userSettings: newSettings,
            }));
            console.log('User settings updated successfully');
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
        } catch (error) {
            console.error('Error updating user settings:', error);
        }
    };

    const getPasswordEntry = async (id: string): Promise<DecryptedPasswordEntry> => {
        console.log('Getting password entry', id);
        const entry = sessionState.passwordEntries.find((e) => e.id === id);
        if (entry) {
            console.log('Password entry found', entry);
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
            return entry;
        } else {
            console.error('Password entry not found');
            throw new Error('Password entry not found');
        }
    }

    const addPasswordEntry = async (newEntry: DecryptedPasswordEntry) => {
        console.log('Adding password entry', newEntry);
        try {
            await apiAddPasswordEntry(newEntry);
            setSessionState((prevState) => ({
                ...prevState,
                passwordEntries: [...prevState.passwordEntries, newEntry],
            }));
            console.log('Password entry added successfully');
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
        } catch (error) {
            console.error('Error adding password entry:', error);
        }
    };

    const removePasswordEntry = async (id: string) => {
        console.log('Removing password entry', id);
        try {
            await apiRemovePasswordEntry(id);
            setSessionState((prevState) => ({
                ...prevState,
                passwordEntries: prevState.passwordEntries.filter((entry) => entry.id !== id),
            }));
            await saveSession();
            console.log('Password entry removed successfully');
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
        } catch (error) {
            console.error('Error removing password entry:', error);
        }
    };

    const generatePassword = async (): Promise<string> => {
        console.log('Generating password');
        try {
            const password = await apiGeneratePassword();
            console.log('Password generated successfully', password);
            console.log('Session state:', JSON.stringify(sessionState, null, 2));
            return password;
        } catch (error) {
            console.error('Error generating password:', error);
            return '';
        }
    };


    useEffect(() => {
        fetchSessionData()
            .catch((error) => {
                console.error('Failed to fetch session data:', error);
            });
    }, [fetchSessionData]);


    return { sessionState, fetchSessionData, updateUserSettings, addPasswordEntry, generatePassword, saveSession, getPasswordEntry, removePasswordEntry };
};
