import React from 'react';
import './App.css';
import SignInSide from './SignInSide';
import Dashboard from './templates/dashboard/Dashboard';
import Button from '@mui/material/Button/Button';



function App() {
  return (
    <div className="App">
    <Dashboard />
    <Button variant="contained" color="primary" className="btn mt-2">Hello World</Button>
    </div>
  );
}

export default App;
