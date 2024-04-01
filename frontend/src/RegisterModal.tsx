import React, { useState } from 'react';
import { useAtom } from 'jotai';
import { AuthAtom, UserIdAtom, registerModalOpenAtom } from './atoms';
import { TextField } from '@mui/material';
import Button from '@mui/material/Button';
import CancelRoundedIcon from '@mui/icons-material/CancelRounded';
import IconButton from '@mui/material/IconButton';


type NewUser = {
  email: string;
  address: string;
  first_name: string;
  last_name: string;
  username: string;
  password: string;
};

async function registerUser(newUser: NewUser) {
  return fetch('http://localhost:6516/create_user', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(newUser)
  })
    .then(response => {
      if (response.ok) {
        return response.json();
      }
      throw new Error('Network response was not ok.');
    });

}




function RegisterModal() {
  const [email, setEmail] = useState('');
  const [address, setAddress] = useState('');
  const [first_name, setFirstName] = useState('');
  const [last_name, setLastName] = useState('');
  const [username, setUserName] = useState('');
  const [password, setPassword] = useState('');
  const [, setIsAuth] = useAtom(AuthAtom);
  const [, setUserId] = useAtom(UserIdAtom);
  const [registerModalOpen, setRegisterModalOpen] = useAtom(registerModalOpenAtom);

  function handleClose() {
    setRegisterModalOpen(false);
  }
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await registerUser({
        email,
        address,
        first_name,
        last_name,
        username,
        password
      });
      setUserId(response.id); // Assuming the ID is directly in the response
      setIsAuth(true);
      setRegisterModalOpen(false);
    } catch (error) {
      console.error('Registration failed:', error);
      // Handle registration failure (e.g., incorrect credentials, network error, etc.)
    }
  };

  return (
    <div className="flex justify-center items-center h-screen bg-gray-100">
      <div className="max-w-md w-full bg-white p-8 border border-gray-300 rounded-lg">
        <div className="flex justify-end">
          <IconButton onClick={handleClose} className="absolute top-0 left-0 m-2 text-gray-600 hover:text-gray-800" aria-label="Close">
            <CancelRoundedIcon />
          </IconButton>
        </div>
        <h1 className="text-xl font-semibold mb-4">Please Register</h1>
        <form onSubmit={handleSubmit} className="space-y-6">
          <TextField
            label="Email"
            variant="outlined"
            className="w-full"
            onChange={e => setEmail(e.target.value)}
          />
          <TextField
            label="Address"
            variant="outlined"
            className="w-full"
            onChange={e => setAddress(e.target.value)}
          />
          <TextField
            label="First Name"
            variant="outlined"
            className="w-full"
            onChange={e => setFirstName(e.target.value)}
          />
          <TextField
            label="Last Name"
            variant="outlined"
            className="w-full"
            onChange={e => setLastName(e.target.value)}
          />
          <TextField
            label="Username"
            variant="outlined"
            className="w-full"
            onChange={e => setUserName(e.target.value)}
          />
          <TextField
            label="Password"
            variant="outlined"
            className="w-full"
            onChange={e => setPassword(e.target.value)}
          />
          <Button
            type="submit"
            variant="contained"
            color="primary"
            className="w-full"
          >

            Register
          </Button>
        </form>
      </div>
    </div>
  );
}

export default RegisterModal;

