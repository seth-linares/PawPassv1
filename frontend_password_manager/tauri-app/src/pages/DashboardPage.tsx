// src/pages/DashboardPage.tsx
import React from 'react';
import DashboardForm from '../components/DashboardForm';
import { useNavigate } from 'react-router-dom';
import { logout } from '../utils/api'; 

const DashboardPage: React.FC = () => {
    const navigate = useNavigate();

    const handleLogout = async () => {
        try {
            await logout(); 
            alert('You have been logged out'); 
            console.log('Logged out');
            navigate('/login'); 
        } catch (error) {
            console.error('Logout failed:', error);
            alert('Logout failed: ' + error);
        
        }
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', minHeight: '97vh' }}>
            <header style={{ display: 'flex', justifyContent: 'center', padding: '10px' }}>
                <h1>Password Manager</h1>
            </header>
            <nav style={{ display: 'flex', justifyContent: 'flex-end', padding: '1px' }}>
            <button className="logout-button" onClick={handleLogout}>Logout</button>
            </nav>
            <main style={{ flex: 1, paddingBottom: '0px' }}> 
                <DashboardForm />
            </main>
            <footer style={{ textAlign: 'center' }}> 
                © 2024 Password Manager. All rights reserved.
            </footer>
        </div>
    );
};

export default DashboardPage;