// src/components/DashboardForm.tsx
import 'react-confirm-alert/src/react-confirm-alert.css';
import 'react-toastify/dist/ReactToastify.css';
import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { useSession } from '../hooks/useSession';
import { DecryptedPasswordEntry } from '../hooks/useSession';
import { getCategories } from '../utils/api';
import { toast, ToastContainer } from 'react-toastify';
import { confirmAlert } from 'react-confirm-alert';




const DashboardForm: React.FC = () => {
    const { sessionState, fetchSessionData, removePasswordEntry } = useSession();
    const [searchTerm, setSearchTerm] = useState('');
    const [selectedCategory, setSelectedCategory] = useState('');
    const [filteredEntries, setFilteredEntries] = useState<DecryptedPasswordEntry[]>([]);
    const [categories, setCategories] = useState<string[]>([]); 
    const [showFavorites, setShowFavorites] = useState(false);

    const navigate = useNavigate();

    useEffect(() => {
        fetchSessionData();
    }, [fetchSessionData]);

    useEffect(() => {
        getCategories().then(setCategories);
    }, []);

    useEffect(() => {
        let filtered = sessionState.passwordEntries.filter((entry) =>
            entry.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
            entry.username?.toLowerCase().includes(searchTerm.toLowerCase()) ||
            entry.category?.toLowerCase().includes(searchTerm.toLowerCase())
        );

        if (selectedCategory) {
            filtered = filtered.filter((entry) => entry.category?.toLowerCase() === selectedCategory.toLowerCase());
        }

        if (showFavorites) {
            filtered = filtered.filter((entry) => entry.favorite);
        }

        filtered.sort((a, b) => a.title.localeCompare(b.title));

        setFilteredEntries(filtered);
    }, [searchTerm, selectedCategory, showFavorites, sessionState.passwordEntries]);
    
    const handleRemove = (id: string) => {
        confirmAlert({
            title: 'Confirm to delete',
            message: 'Are you sure you want to delete this entry?',
            buttons: [
                {
                    label: 'Yes',
                    onClick: async () => await removePasswordEntry(id)
                },
                {
                    label: 'No',
                    onClick: () => {}
                }
            ]
        });
    };
    
    const handleCopyPassword = (password: string) => {
        navigator.clipboard.writeText(password);
        toast.success('Password copied to clipboard', { autoClose: 800 });
    };

    const handleCopyUsername = (username: string) => {
        navigator.clipboard.writeText(username);
        toast.success('Username copied to clipboard', { autoClose: 800 });
    };


    const handleAddNew = () => {
        console.log('Add new clicked');
        navigate('/password-entry');
    };

    const handleEdit = (entry: DecryptedPasswordEntry) => {
        console.log('Edit entry clicked');
        console.log(entry);
        navigate('/password-entry', { state: { entry } });
    };


    return (
        <>
            <ToastContainer />
            <div style={{ maxWidth: '1600px', margin: 'auto' }}>
                <div style={{ display: 'flex', justifyContent: 'center', gap: '30px', marginBottom: '20px' }}>
                    <input
                        type="text"
                        placeholder="Search..."
                        value={searchTerm}
                        onChange={(e) => setSearchTerm(e.target.value)}
                    />
                    <select value={selectedCategory} onChange={(e) => setSelectedCategory(e.target.value)}>
                        <option value="">All Categories</option>
                        {categories.map((category) => (
                            <option key={category} value={category}>{category}</option>
                        ))}
                    </select>
                    <label className="custom-checkbox">
                        Show Favorites
                        <input 
                            type="checkbox" 
                            checked={showFavorites} 
                            onChange={(e) => setShowFavorites(e.target.checked)} 
                            style={{ display: 'none' }} 
                        />
                        <span className="checkmark"></span>
                    </label>
                    <button onClick={handleAddNew}>Add New</button>
                </div>
                <div style={{ overflowY: 'scroll', maxHeight: '700px', backgroundColor: '#3e424a', padding: '10px', borderRadius: '5px' }}>
                    {filteredEntries.length > 0 ? (
                        filteredEntries.map((entry) => (
                            <div key={entry.id} className="entry-container">
                                <div className="entry-fields">
                                    <div className="entry-field-names">
                                        <p>Title:</p>
                                        <p>Username:</p>
                                        <p>URL:</p>
                                        <p>Category:</p>
                                        <p>Date:</p>
                                    </div>
                                    <div className="entry-data">
                                        <p>{entry.title}</p>
                                        <p>{entry.username}</p>
                                        <p>{entry.url}</p>
                                        <p>{entry.category}</p>
                                        <p>{entry.creationDate && new Date(entry.creationDate).toLocaleDateString()}</p>
                                    </div>
                                </div>
                                <div className="entry-buttons">
                                    <button onClick={() => handleEdit(entry)}>Edit</button>
                                    <button onClick={() => entry.username && handleCopyUsername(entry.username)}>Copy Username</button>
                                    <button onClick={() => entry.password && handleCopyPassword(entry.password)}>Copy Password</button>
                                    <button style={{color: '#f72044'}} onClick={() => handleRemove(entry.id)}>Remove</button>
                                </div>
                            </div>
                        ))
                    ) : (
                        <p>No password entries found</p>
                    )}
                </div>
            </div>
        </>
    );
};


export default DashboardForm;