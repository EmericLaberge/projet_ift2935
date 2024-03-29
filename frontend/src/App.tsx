
import React from 'react';
import { HashRouter as Router, Route, Routes, BrowserRouter } from 'react-router-dom';
import './App.css';
import NavBar from './Components/Navbar/NavBar';
import { Card } from '@mui/material';
import UserList from './UserList';
import { Toaster } from 'react-hot-toast';
import {AuthAtom, UserIdAtom}from './atoms';
import { useAtom } from 'jotai';
import Login from './Login';


function App() {
  const [auth, setAuth] = useAtom(AuthAtom);
  const [userId, setUserId] = useAtom(UserIdAtom);

  if (!auth) {
    return <Login/>;
  }
  return (
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
}

export default App;
