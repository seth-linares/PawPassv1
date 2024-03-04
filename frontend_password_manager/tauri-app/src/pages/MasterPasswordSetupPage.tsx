// src/pages/MasterPasswordSetupPage.tsx
import MasterPasswordSetupForm from '../components/MasterPasswordSetupForm';
import reactLogo from "../assets/PawPass_Logo.png";

function MasterPasswordSetupPage() {
    return (
        <div style={{ textAlign: 'center'}}>
            <h1>Welcome to PawPass</h1>
            <h2>Set Up Master Password</h2>
            <MasterPasswordSetupForm />
            <img src={reactLogo} alt="PawPass logo" style={{maxWidth: '15%'}} />
        </div>
    );
}

export default MasterPasswordSetupPage;