import React, { useEffect, useState } from 'react';
import { Box, Card, CardContent, TextField, Button, Typography } from '@mui/material';
import { DataGrid } from '@mui/x-data-grid';
import {toast} from 'react-hot-toast';


type User = {
  id: number;
  email: string; address: string;
  first_name: string;
  last_name: string;
};

function UserList() {
  const [users, setUsers] = useState<User[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [newUser, setNewUser] = useState<Partial<User>>({ email: '', address: '', first_name: '', last_name: '' });

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewUser({ ...newUser, [e.target.name]: e.target.value });
  };

  const handleNewUserSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('http://127.0.0.1:8080/create_user', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newUser),
      });
      if (!response.ok) {
        throw new Error('Failed to create user');
      }
      setNewUser({ email: '', address: '', first_name: '', last_name: '' }); // Reset form fields
      toast.success('User created successfully');

    } catch (error) {
      setError('Failed to create user');
    }
  };

  useEffect(() => {
    const fetchUsers = async () => {
      try {
        const response = await fetch('http://127.0.0.1:8080/users');
        if (!response.ok) {
          throw new Error('Failed to fetch users');
        }
        const data: User[] = await response.json();
        setUsers(data);
      } catch (error) {
        setError('Failed to fetch users');
      }
    };

    fetchUsers();
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (
    <div>
      <h1>Users</h1>
      <Box>
        <Card>
          <CardContent>
            <Typography variant="h5" component="div">
              Add New User
            </Typography>
            <form onSubmit={handleNewUserSubmit}>
              <TextField
                name="email"
                label="Email"
                value={newUser.email}
                onChange={handleInputChange}
                margin="normal"
              />
              <TextField
                name="address"
                label="Address"
                value={newUser.address}
                onChange={handleInputChange}
                margin="normal"
              />
              <TextField
                name="first_name"
                label="First Name"
                value={newUser.first_name}
                onChange={handleInputChange}
                margin="normal"
              />
              <TextField
                name="last_name"
                label="Last Name"
                value={newUser.last_name}
                onChange={handleInputChange}
                margin="normal"
              />
              <Button type="submit" variant="contained" color="primary">
                Add User
              </Button>
            </form>
          </CardContent>
        </Card>
      </Box>
      <h2>User List</h2>
      <DataGrid
        rows={users}
        columns={[
          { field: 'id', headerName: 'ID', width: 90 },
          { field: 'email', headerName: 'Email', width: 200 },
          { field: 'address', headerName: 'Address', width: 200 },
          { field: 'first_name', headerName: 'First Name', width: 150 },
          { field: 'last_name', headerName: 'Last Name', width: 150 },
        ]}
      />
    </div>
  );
}

export default UserList;

