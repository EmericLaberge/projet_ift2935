// App.tsx
import React from 'react';
import { HashRouter as Router, Route, Routes } from 'react-router-dom';
import './App.css';
import NavBar from './Components/Navbar/NavBar';
import { Card } from '@mui/material';
import UserList from './UserList';
import { Toaster} from 'react-hot-toast';

const App: React.FC = () => (
<div>
  <Router>
    <NavBar />
    <Routes>
      <Route path="/" element={<Card />} />
      <Route path="/Users" element={<UserList />} />
    </Routes>
  </Router>
  <Toaster />
</div>
);

export default App;
