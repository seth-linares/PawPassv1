// src/components/MasterPasswordSetupForm.tsx
import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { createMasterPassword } from '../utils/api';
import '../App.css'; 


function MasterPasswordSetupForm() {
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

        if(password.trim() === "" || confirmPassword.trim() === "") {
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
            setConfirmPasswordError("Master password must be at least 8 characters");
            setIsLoading(false);
            return;
        }

        try {
            await createMasterPassword(password); 
            navigate('/login');
        } catch (error) {
            console.error('Failed to create master password:', error);
            setError('Failed to create master password');
        } finally {
            setIsLoading(false);
        }
    }

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '50vh' }}>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="password" style={{ marginRight: '10px', width: '150px', textAlign: 'left' }}>Master Password:</label>
                    <input
                        type="password"
                        id="password"
                        name="password"
                        value={password}
                        onChange={(e) => setPassword(e.target.value)}
                    />
                </div>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="confirmPassword" style={{ marginRight: '10px', width: '150px', textAlign: 'left' }}>Confirm Password:</label>
                    <input
                        type="password"
                        id="confirmPassword"
                        name="confirmPassword"
                        value={confirmPassword}
                        onChange={(e) => setConfirmPassword(e.target.value)}
                    />
                </div>
                {confirmPasswordError && <p>{confirmPasswordError}</p>}
                {error && <p>{error}</p>}
                <button type="submit" disabled={isLoading} style={{ marginTop: '20px' }}>
                    Create Master Password
                </button>
                {isLoading && <div className="spinner"></div>}
            </form>
        </div>
    )
}

export default MasterPasswordSetupForm;