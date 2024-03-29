import React, { useState } from 'react';
import { useAtom } from 'jotai';
import { AuthAtom, UserIdAtom, registerModalOpenAtom } from './atoms';
import { TextField } from '@mui/material';
import Button from '@mui/material/Button';
import { atom } from 'jotai';
import RegisterModal from './RegisterModal';


type Credentials = {
  username: string;
  password: string;
};

async function loginUser(credentials: Credentials) {
  return fetch('http://localhost:8080/login', {
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
      <div className="flex justify-center items-center h-screen bg-gray-100">
        <div className="max-w-md w-full bg-white p-8 border border-gray-300 rounded-lg">
          <h1 className="text-xl font-semibold mb-4">Please Log In</h1>
          <form onSubmit={handleSubmit} className="space-y-6">
            <input
              type="text"
              placeholder="Username"
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              onChange={e => setUserName(e.target.value)}
            />
            <input
              type="password"
              placeholder="Password"
              className="w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              onChange={e => setPassword(e.target.value)}
            />
            <button
              type="submit"
              className="w-full py-2 bg-blue-500 text-white rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50"
            >
              Login
            </button>
          </form>
          <Button onClick={() => setRegisterModalOpen(true)}>Register</Button>
        </div>
      </div>
    )
  }
}
