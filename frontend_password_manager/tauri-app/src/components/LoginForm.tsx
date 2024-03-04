// src/components/LoginForm.tsx
import { useRef, useState } from "react";
import { useNavigate } from "react-router-dom";
import { login } from '../utils/api'; 

function LoginForm() {
    const passwordRef = useRef<HTMLInputElement>(null);
    const [passwordError, setPasswordError] = useState("");
    const [loginError, setLoginError] = useState(""); 
    const navigate = useNavigate();

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setPasswordError("");
        setLoginError(""); 

        const password = passwordRef.current?.value || "";

        if(password.trim() === "") {
            setPasswordError("Password is required");
            return;
        }

        else if(password.length > 128) {
            setPasswordError("Invalid password attempt");
            return;
        }

        try {
            await login(password); 
            navigate('/dashboard');
        } catch (error) {
            console.error('Failed to login:', error);
            setLoginError('Failed to login'); 
        }
    }

    return (
        <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '20vh' }}>
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column' }}>
                <div style={{ display: 'flex', marginBottom: '10px', alignItems: 'center' }}>
                    <label htmlFor="password" style={{ marginRight: '10px', width: '100px', textAlign: 'left' }}>Password:</label>
                    <input
                        type="password"
                        id="password"
                        name="password"
                        ref={passwordRef}
                    />
                </div>
                    {passwordError && <p>{passwordError}</p>}
                    {loginError && <p>{loginError}</p>} 
                <button type="submit">Login</button>
            </form>
        </div>
    )
}

export default LoginForm;