
import React from 'react';
import { HashRouter as Router, Route, Routes, BrowserRouter } from 'react-router-dom';
import './App.css';
import NavBar from './Components/Navbar/NavBar';
import { Card } from '@mui/material';
import UserList from './UserList';
import { Toaster } from 'react-hot-toast';
import Login from './Login';


function setToken(userToken: string) {
  sessionStorage.setItem('token', JSON.stringify(userToken));
}

function getToken() {
  const tokenString = sessionStorage.getItem('token');
  const userToken = JSON.parse(tokenString || '{}');
  return userToken?.token;
}


function App() {
  // const token = getToken();
  // if (!token) {
  //   return <Login setToken={setToken} />
  // }
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
