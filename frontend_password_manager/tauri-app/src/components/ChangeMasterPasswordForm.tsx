// src/components/ChangeMasterPasswordForm.tsx
import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { changeMasterPassword } from '../utils/api'; 

function ChangeMasterPasswordForm() {
    const [oldPassword, setOldPassword] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [confirmPasswordError, setConfirmPasswordError] = useState("");
    const [error, setError] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const navigate = useNavigate();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);
        setConfirmPasswordError("");
        setError("");

        if(oldPassword.trim() === "" || password.trim() === "" || confirmPassword.trim() === "") {
            setConfirmPasswordError("Password is required");
            setIsLoading(false);
            return;
        }

        else if(password !== confirmPassword) { 
            setConfirmPasswordError("Passwords do not match");
            setIsLoading(false);
            return;
        }
            
        else if(password.length < 8) {
            setConfirmPasswordError("Master password must be at least 8 characters!");
            setIsLoading(false);
            return;
        }

        try {
            await changeMasterPassword(oldPassword, password);
            navigate('/login');
        } catch (error) {
            console.error('Failed to change master password:', error);
            setError('Failed to change master password');
        } finally {
            setIsLoading(false);
        }
    }

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '20vh' }}>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column'}}>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="oldPassword" style={{ marginRight: '10px', width: '200px', textAlign: 'left' }}>Current Master Password:</label>
                    <input
                        type="password"
                        id="oldPassword"
                        name="oldPassword"
                        value={oldPassword}
                        onChange={(e) => setOldPassword(e.target.value)}
                    />
                </div>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="password" style={{ marginRight: '10px', width: '200px', textAlign: 'left' }}>New Master Password:</label>
                    <input
                        type="password"
                        id="password"
                        name="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </div>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="confirmPassword" style={{ marginRight: '10px', width: '200px', textAlign: 'left' }}>Confirm New Password:</label>
                    <input
                        type="password"
                        id="confirmPassword"
                        name="confirmPassword"
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                    />
                </div>
                {confirmPasswordError && <p>{confirmPasswordError}</p>}
                <button type="submit" disabled={isLoading}>Change Master Password</button>
                {error && <p>{error}</p>}
            </form>
        </div>
    );
}

export default ChangeMasterPasswordForm;

