// src/components/PasswordEntryForm.tsx
import React, { useState, useEffect } from 'react';
import { DecryptedPasswordEntry, useSession, UserSettings } from '../hooks/useSession';
import { createNewDecryptedPasswordEntry } from '../utils/api';
import { useNavigate, useLocation } from 'react-router-dom';
import 'react-toastify/dist/ReactToastify.css';
import { toast, ToastContainer } from 'react-toastify';


function PasswordEntryForm() {
  const location = useLocation();
  const initialEntry: DecryptedPasswordEntry | undefined = location.state?.entry;
  
  

  if(initialEntry) {
    console.log('initialEntry', initialEntry);
  }

  else {
    console.log('initialEntry is undefined');
  }

  const { addPasswordEntry, saveSession, generatePassword, updateUserSettings, sessionState } = useSession();
  const navigate = useNavigate();
  const [isLoading, setIsLoading] = useState(false);
  const [title, setTitle] = useState(initialEntry?.title || '');
  const [username, setUsername] = useState(initialEntry?.username || '');
  const [password, setPassword] = useState(initialEntry?.password || '');
  const [url, setUrl] = useState(initialEntry?.url || '');
  const [notes, setNotes] = useState(initialEntry?.notes || '');
  const [category, setCategory] = useState(initialEntry?.category || '');
  const [favorite, setFavorite] = useState(initialEntry?.favorite || false);

  const [settings, setSettings] = useState(sessionState.userSettings);
  const [entry, setEntry] = useState<DecryptedPasswordEntry | null>(initialEntry || null);
  const [titleError, setTitleError] = useState('');
  const [settingsError, setSettingsError] = useState('');

  useEffect(() => {
    console.log('initialEntry changed', initialEntry);
    setTitle(initialEntry?.title || '');
    setUsername(initialEntry?.username || '');
    setPassword(initialEntry?.password || '');
    setUrl(initialEntry?.url || '');
    setNotes(initialEntry?.notes || '');
    setCategory(initialEntry?.category || '');
    setFavorite(initialEntry?.favorite || false);
  }, [initialEntry]);

  const validateSettings = (settings: UserSettings) => {
    console.log('validateSettings called', settings);
    if (!(settings.useNum || settings.useSymbol || settings.useLower || settings.useUpper)) {
      settings.useLower = true; 
      console.log('At least one character type must be selected');
      setSettingsError('At least one character type must be selected');
    } else {
      setSettingsError('');
    }

    let length = 4;
    if (settings.useNum) {
      length += Number(settings.minNum);
    }
    if (settings.useSymbol) {
      length += Number(settings.minSymbol);
    }
    if (settings.useLower) {
      length += 1;
    }
    if (settings.useUpper) {
      length += 1;
    }
    console.log('length', length);
    settings.minPasswordLength = length;
    
    return settings;
  };


  const handleChange = (event: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    console.log('handleChange called', event.target);
    const { name, value, type } = event.target;
    const newValue = type === 'checkbox' ? (event.target as HTMLInputElement).checked : value;
  
    switch (name) {
      case 'title':
        setTitle(newValue as string);
        break;
      case 'username':
        setUsername(newValue as string);
        break;
      case 'password':
        setPassword(newValue as string);
        break;
      case 'url':
        setUrl(newValue as string);
        break;
      case 'notes':
        setNotes(newValue as string);
        break;
      case 'category':
        setCategory(newValue as string);
        break;
      case 'favorite':
        setFavorite(newValue as boolean);
        break;
      default:
        break;
    }
  
    setEntry(prevEntry => {
      if (!prevEntry) return prevEntry;
      return {
        ...prevEntry,
        [name]: newValue,
      };
    });
  };

  const handleSettingsChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    console.log('handleSettingsChange called', event.target);

    const { name, value, checked, type } = event.target;
    setSettings(prevSettings => {
      const newSettings = {
        ...prevSettings,
        [name]: type === 'checkbox' ? checked : Number(value),
      };
      
      if (name === 'passwordLength') {
        newSettings.passwordLength = Math.max(newSettings.minPasswordLength, Math.min(Number(value), 128));
      }

      const validatedSettings = validateSettings(newSettings);
      if (validatedSettings) {
        updateUserSettings(validatedSettings);
      }
      return validatedSettings;
    });
  };

  useEffect(() => {
    setSettings(sessionState.userSettings);
  }, [sessionState.userSettings]);

  const handleSubmit = async (event: React.FormEvent) => {
    console.log('handleSubmit called');
    event.preventDefault();
  
    let hasError = false;
    if (title === '') {
      console.log('Title is missing');
      setTitleError('Title is required');
      hasError = true;
    }
  
    if (!validateSettings(settings)) {
      console.log('Settings are invalid');
      setSettingsError('Settings are invalid');
      hasError = true;
    }
  
    if (hasError) {
      console.log('Form has errors, not submitting');
      return;
    }
  
    console.log('Form is valid, submitting');
    setIsLoading(true);
  
    try {
      let newEntry: DecryptedPasswordEntry;
      if (entry) {
        console.log('Updating existing entry');
        newEntry = {
          ...entry,
          title,
          username,
          password,
          url,
          notes,
          category,
          favorite,
        };
      } else {
        console.log('Creating new entry');
        newEntry = await createNewDecryptedPasswordEntry();
        newEntry = {
          ...newEntry,
          title,
          username,
          password,
          url,
          notes,
          category,
          favorite,
        };
      }
      await addPasswordEntry(newEntry);
      await saveSession();
      navigate('/dashboard');
    } catch (error) {
      console.error('Error submitting form:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleGeneratePassword = async () => {
    console.log('handleGeneratePassword called');
  
    setIsLoading(true);
    await updateUserSettings(settings); 
    const password = await generatePassword();
    setPassword(password);
    setIsLoading(false);
  
    toast.success('Password generated successfully!', {
      autoClose: 500,
    });
  };


  const handleCopyPassword = (): void => {
    console.log('handleCopyPassword called');
  
    navigator.clipboard.writeText(password)
      .then(() => {
        toast.success('Password copied to clipboard!', {
          autoClose: 500,
        });
      })
      .catch(err => {
        toast.error(`Failed to copy password to clipboard. Error: ${err.toString()}`);
      });
  };
  
  const handleCancel = () => {
    console.log('handleCancel called');

    setEntry(initialEntry || null);
    setPassword('');
    navigate('/dashboard');
  };

  console.log('Rendering PasswordEntryForm', {
    isLoading,
    title,
    username,
    password,
    url,
    notes,
    category,
    favorite,
    settings,
    entry,
    titleError,
    settingsError,
  });
  
  return (
    
    <form onSubmit={handleSubmit}>
      <ToastContainer />
      {/* Entry fields */}
      <div className="password-fields">
        <label>
          Title
          <input type="text" name="title" value={title} onChange={handleChange} disabled={isLoading} />
          {titleError && <div>{titleError}</div>}
        </label>
        <label>
          Username
          <input type="text" name="username" value={username} onChange={handleChange} />
        </label>
        <label>
          Password
          <input type="text" name="password" value={password} onChange={handleChange} />
          <button type="button" onClick={handleCopyPassword}>Copy</button>
        </label>
        <label>
          URL
          <input type="text" name="url" value={url} onChange={handleChange} />
        </label>
        <label>
          Notes
          <textarea name="notes" value={notes} onChange={handleChange} />
        </label>
        <label>
          Category
          <input type="text" name="category" value={category} onChange={handleChange} />
        </label>
        <label>
          Favorite
          <input type="checkbox" name="favorite" checked={favorite} onChange={handleChange} />
        </label>
      </div>
      
      {/* Settings fields */}
      <div className="settings">
        <label className="custom-checkbox">
          Use Numbers
          <input type="checkbox" name="useNum" checked={settings.useNum} onChange={handleSettingsChange} style={{ display: 'none' }} />
          <span className="checkmark"></span>
        </label>
        <label>
          Minimum Numbers
          <input type="number" name="minNum" value={settings.minNum} onChange={handleSettingsChange} />
        </label>
        <label className="custom-checkbox">
          Use Symbols
          <input type="checkbox" name="useSymbol" checked={settings.useSymbol} onChange={handleSettingsChange} style={{ display: 'none' }} />
          <span className="checkmark"></span>
        </label>
        <label>
          Minimum Symbols
          <input type="number" name="minSymbol" value={settings.minSymbol} onChange={handleSettingsChange} />
        </label>
        <label className="custom-checkbox">
          Use Lowercase
          <input type="checkbox" name="useLower" checked={settings.useLower} onChange={handleSettingsChange} style={{ display: 'none' }} />
          <span className="checkmark"></span>
        </label>
        {settingsError && <div>{settingsError}</div>}
        <label className="custom-checkbox">
          Use Uppercase
          <input type="checkbox" name="useUpper" checked={settings.useUpper} onChange={handleSettingsChange} style={{ display: 'none' }} />
          <span className="checkmark"></span>
        </label>
        <label>
          Password Length: {settings.passwordLength}
          <input type="range" name="passwordLength" min="5" max="128" value={settings.passwordLength} onChange={handleSettingsChange} />
        </label>
      </div>

      {isLoading && <div className="spinner"></div>}

      <div className="password-buttons">
        <button type="button" onClick={handleGeneratePassword} disabled={isLoading}>Generate Password</button>
        <button type="submit" disabled={isLoading}>Save</button>
        <button type="button" onClick={handleCancel} disabled={isLoading}>Cancel</button>
      </div>

    </form>
  );
}

export default PasswordEntryForm;