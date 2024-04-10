
import React from 'react';
import { HashRouter as Router, Route, Routes, BrowserRouter } from 'react-router-dom';
import './App.css';
import NavBar from './Components/Navbar/NavBar';
import { Card } from '@mui/material';
import UserList from './UserList';
import { Toaster } from 'react-hot-toast';
import { AuthAtom, UserIdAtom } from './atoms';
import { useAtom } from 'jotai';
import Login from './Login';
import TeamList from './TeamList';
import GameList from './GameList';
import EventList from './EventList';


function App() {
  const [auth, setAuth] = useAtom(AuthAtom);
  const [userId, setUserId] = useAtom(UserIdAtom);

  if (!auth) {
    return (
      <>
        <Login />
        <Toaster />
      </>
    )

  }
  return (
    <div>
      <Router>
        <NavBar />
        <Routes>
          <Route path="/" element={<Card />} />
          <Route path="/Users" element={<UserList />} />
          <Route path="/Login" element={<Login />} />
          <Route path="/Teams" element={<TeamList />} />
          <Route path="/Events" element={<EventList />} />
          <Route path="/Games" element={<GameList />} />
        </Routes>
      </Router>
      <Toaster />
    </div>
  );
}

export default App;
