// src/pages/LoginPage.tsx
import LoginForm from '../components/LoginForm';
import { Link } from 'react-router-dom';
import logo from "../assets/PawPass_Logo.png";
function LoginPage() {
    return (
        <div style={{ textAlign: 'center'}}>
            <img src={logo} alt="Logo" style={{maxWidth: '20%'}} />
            <h1>Welcome to PawPass</h1>
            <h2 style={{ fontFamily: 'Arial, sans-serif', fontSize: '1.5em', fontWeight: 'normal', color: '#0078f0' }}>Login to your account</h2>
            <LoginForm />
            <Link to="/change-master-password" style={{color: '#047ec9', textDecoration: 'underline', marginTop: '10px'}}>Change Master Password</Link>
        </div>
    );
}

export default LoginPage;