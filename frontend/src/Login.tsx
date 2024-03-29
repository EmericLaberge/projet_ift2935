import React, { useState } from 'react';
import { useAtom } from 'jotai';
import { AuthAtom, UserIdAtom } from './atoms';

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

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
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

  return (
    <div className="login-wrapper">
      <h1>Please Log In</h1>
      <form onSubmit={handleSubmit}>
        <label>
          <p>Username</p>
          <input type="text" onChange={e => setUserName(e.target.value)} />
        </label>
        <label>
          <p>Password</p>
          <input type="password" onChange={e => setPassword(e.target.value)} />
        </label>
        <div>
          <button type="submit">Submit</button>
        </div>
      </form>
    </div>
  );
}

