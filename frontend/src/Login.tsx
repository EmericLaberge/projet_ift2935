import React, { useState } from 'react';
import { useAtom } from 'jotai';
import { AuthAtom, UserIdAtom, registerModalOpenAtom } from './atoms';
import { TextField } from '@mui/material';
import Button from '@mui/material/Button';
import { atom } from 'jotai';
import RegisterModal from './RegisterModal';
import Chip from '@mui/material/Chip';


type Credentials = {
  username: string;
  password: string;
};


async function registerUser(credentials: Credentials) {
  return

}

async function loginUser(credentials: Credentials) {
  return fetch('http://localhost:6516/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(credentials)
  })
    .then(response => {
      if (response.ok) {
        return response.json();
      }
      throw new Error('Network response was not ok.');
    });
}

export default function Login() {
  const [username, setUserName] = useState('');
  const [password, setPassword] = useState('');
  const [, setIsAuth] = useAtom(AuthAtom);
  const [, setUserId] = useAtom(UserIdAtom);
  const [registerModalOpen, setRegisterModalOpen] = useAtom(registerModalOpenAtom);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await loginUser({
        username,
        password
      });
      setUserId(response.id); // Assuming the ID is directly in the response
      setIsAuth(true);
    } catch (error) {
      console.error('Login failed:', error);
      // Handle login failure (e.g., incorrect credentials, network error, etc.)
    }
  };
  if (registerModalOpen) {
    return <RegisterModal />;
  } else {
    return (
      <div>

        <div className="flex flex-col justify-center items-center h-screen">
          <h1 id="header-title" className="text-4xl font-semibold text-white mb-12">Welcome to the App</h1>
      <div className="max-w-md w-full bg-gray-900   p-8 border border-gray-700 rounded-md relative">
            <h1 className="text-xl font-semibold mb-4">Please Log In</h1>
            <form onSubmit={handleSubmit} className="space-y-6">
              <TextField
                label="Username"
                type="text"
                variant="outlined"
                color="primary"
                className="w-full px-4 py-2 border  rounded-md"
                onChange={e => setUserName(e.target.value)}
              />
              <TextField
                label="Password"
                type="password"
                variant="outlined"
                color="primary"
                className="w-full px-4 py-2 border  rounded-md"
                onChange={e => setPassword(e.target.value)}
              />
              <Button
                type="submit"
                className="w-full py-2   rounded-md "
                variant="contained"
                color="primary"
              >
                Log In
              </Button>


            </form>
            <div className="flex justify-left items-center mt-6">
              <Button onClick={() => setRegisterModalOpen(true)} variant="outlined" color="primary">Create Account</Button>
            </div>
          </div>
        </div>
      </div>
    )
  }
}
