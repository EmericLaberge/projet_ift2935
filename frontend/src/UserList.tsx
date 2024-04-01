import React, { useEffect, useState } from 'react';
import { Box, Card, CardContent, TextField, Button, Typography } from '@mui/material';
import { DataGrid, GridActionsCellItem } from '@mui/x-data-grid';
import { toast } from 'react-hot-toast';
import SaveIcon from '@mui/icons-material/Save';
import CancelIcon from '@mui/icons-material/Cancel';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import './App.css';
import {
  GridRowsProp,
  GridRowModesModel,
  GridRowModes,
  GridColDef,
  GridToolbarContainer,
  GridEventListener,
  GridRowId,
  GridRowModel,
  GridRowEditStopReasons,
  GridSlots,
} from '@mui/x-data-grid';

type User = {
  id: number;
  email: string;
  address: string;
  first_name: string;
  last_name: string;
};



function UserList() {
  const [users, setUsers] = useState<User[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [newUser, setNewUser] = useState<Partial<User>>({ email: '', address: '', first_name: '', last_name: '' });
  const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>({});


  const handleEditClick = (id: GridRowId) => () => {
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.Edit },
      };
    });
  }

  const handleSaveClick = (id: GridRowId) => () => {
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }

  const handleCancelClick = (id: GridRowId) => () => {
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }

  const handleDeleteClick = (id: GridRowId) => () => {
      handleDeleteUserSubmit(id);
      handleCancelClick(id);
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewUser({ ...newUser, [e.target.name]: e.target.value });
  };

  const handleDeleteUserSubmit = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/delete_user/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete user');
      }
      toast.success('User deleted successfully');
    } catch (error) {
      toast.error('Failed to delete user');

    }
  };
  const handleNewUserSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('http://127.0.0.1:6516/create_user', {
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

  const handleEdit = async (id: number) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/users/${id}`);
      if (!response.ok) {
        throw new Error('Failed to fetch user');
      }
      const user: User = await response.json();
      setNewUser(user);
    } catch (error) {
      setError('Failed to fetch user');
    }
  }

  const handleDelete = async (id: number) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/users/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete user');
      }
      const updatedUsers = users.filter((user) => user.id !== id);
      setUsers(updatedUsers);
      toast.success('User deleted successfully');
    } catch (error) {
      setError('Failed to delete user');
    }
  }





  useEffect(() => {
    const fetchUsers = async () => {
      try {
        const response = await fetch('http://127.0.0.1:6516/users');
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

    <div className="App">
      <h1 id="header-title">User Management</h1>
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
          {
            field: 'actions',
            type: 'actions',
            headerName: 'Actions',
            width: 100,
            cellClassName: 'actions',
            getActions: ({ id }) => {
              const isInEditMode = rowModesModel[id]?.mode === GridRowModes.Edit;

              if (isInEditMode) {
                return [
                  <GridActionsCellItem
                    icon={<SaveIcon />}
                    label="Save"
                    sx={{
                      color: 'primary.main',
                    }}
                    onClick={handleSaveClick(id.toString())}
                  />,
                  <GridActionsCellItem
                    icon={<CancelIcon />}
                    label="Cancel"
                    className="textPrimary"
                    onClick={handleCancelClick(id)}
                    color="inherit"
                  />,
                ];
              }

              return [
                <GridActionsCellItem
                  icon={<EditIcon />}
                  label="Edit"
                  className="textPrimary"
                  onClick={handleEditClick(id)}
                  color="inherit"
                />,
                <GridActionsCellItem
                  icon={<DeleteIcon />}
                  label="Delete"
                  onClick={handleDeleteClick(id)}
                  color="inherit"
                />,
              ];
            }
          },
        ]}
        checkboxSelection
      />
    </div>
  );
}


export default UserList;



