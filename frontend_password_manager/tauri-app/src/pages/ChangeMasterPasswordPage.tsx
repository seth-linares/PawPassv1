// src/pages/ChangeMasterPasswordPage.tsx
import { Link } from 'react-router-dom';
import ChangeMasterPasswordForm from '../components/ChangeMasterPasswordForm';
import kitty from '../assets/kitty.png';

function ChangeMasterPasswordPage() {
    return (
        <div style={{ textAlign: 'center'}}>

            <h1>Change Master Password</h1>
            <div className='kitty-container'>
                <img src={kitty} className='kitty-image' alt="logo"/>
                <p className='kitty-text'>AAAAAAAAAAAAAAAA</p>
            </div>
            <ChangeMasterPasswordForm />
            <div style={{ marginTop: '75px' }}>
                <Link to="/login" style={{color: 'light_blue', textDecoration: 'underline'}}>Back to Login</Link>
            </div>
        </div>
    );
}

export default ChangeMasterPasswordPage;