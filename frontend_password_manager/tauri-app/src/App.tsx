import { useEffect, useState } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import LoginPage from './pages/LoginPage';
import DashboardPage from './pages/DashboardPage';
import MasterPasswordSetupPage from './pages/MasterPasswordSetupPage';
import ChangeMasterPasswordPage from './pages/ChangeMasterPasswordPage';
import { checkApplicationDataExistence } from './utils/api'; 
import PasswordEntryPage from './pages/PasswordEntryPage';

function App() {
  const [initialRoute, setInitialRoute] = useState('/');

  useEffect(() => {
    const initializeApp = async () => {
  
      try {
        const applicationDataExists = await checkApplicationDataExistence();
        if (applicationDataExists) {
          setInitialRoute('/login');
        } else {
          setInitialRoute('/setup-master-password');
        }
      }
      catch (error) {
        console.error('Deserialization not possible -- data corrupted!:', error);
        setInitialRoute('/setup-master-password')
      }

    };

    initializeApp();
  }, []);

  return (
    <Router>
      <Routes>
        <Route path="/" element={<Navigate replace to={initialRoute} />} />
        <Route path="/login" element={<LoginPage />} />
        <Route path="/dashboard" element={<DashboardPage />} />
        <Route path="/setup-master-password" element={<MasterPasswordSetupPage />} />
        <Route path="/change-master-password" element={<ChangeMasterPasswordPage />} />
        <Route path="/password-entry" element={<PasswordEntryPage />} />
        <Route path="/password-entry" element={<PasswordEntryPage />} />
      </Routes>
    </Router>
  );
}

export default App;