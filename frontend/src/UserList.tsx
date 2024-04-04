import React, { useEffect, useState } from 'react';
import { Box, Card, CardContent, TextField, Button, Typography, Chip } from '@mui/material';
import { List, ListItem, ListItemText, ListItemSecondaryAction, IconButton } from '@mui/material';
import { DataGrid, GridActionsCellItem } from '@mui/x-data-grid';
import { toast } from 'react-hot-toast';
import SaveIcon from '@mui/icons-material/Save';
import CancelIcon from '@mui/icons-material/Cancel';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import Modal from '@mui/material/Modal';
import EditUserModal from './Components/EditUserModal';
import User from './Components/Entities/User';
import AddIcon from '@mui/icons-material/Add';
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
import AddUserModal from './Components/AddUserModal';

function UserList() {
  const [users, setUsers] = useState<User[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [editUser, setEditUser] = useState<User>({ id: -1, email: '', address: '', first_name: '', last_name: '' });
  const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>({});
  const [newUser, setNewUser] = useState<Partial<User>>({ email: '', address: '', first_name: '', last_name: '' });
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [openEdit, setOpenEdit] = useState(false);
  const handleOpenEdit = () => setOpenEdit(true);
  const handleCloseEdit = () => setOpenEdit(false);




  const handleEditClick = (id: GridRowId) => () => {
    let user = users.find((user) => user.id === id);
    if (user) {
      setEditUser(user);
      setOpenEdit(true);
    }
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
    setEditUser((prev) => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleDeleteUserSubmit = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/users/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete user');
      }
      toast.success('User deleted successfully');
    } catch (error) {
      toast.error('Failed to delete user');

    }
        const response = await fetch('http://127.0.0.1:6516/users');
        if (!response.ok) {
          throw new Error('Failed to fetch users');
        }
        const data: User[] = await response.json();
        setUsers(data);
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

  const EditUserSubmit = async (user: User) => {
    const id = user.id;
    try {
      const response = await fetch(`http://127.0.0.1:6516/users/${user.id}`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(user),
      });
      if (!response.ok) {
        throw new Error('Failed to update user');
      }
      toast.success('User updated successfully');
    } catch (error) {
      toast.error('Failed to update user');
    }
  };

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
      <div className="w-3/4 align-center justify-end m-auto flex mb-3">
        <Chip label="Add User" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
      </div>
      <div>
        <AddUserModal
          open={open}
          handleClose={handleClose}
          handleInputChange={(e) => setNewUser({ ...newUser, [e.target.name]: e.target.value })}
          handleNewUserSubmit={handleNewUserSubmit}
          newUser={newUser}
        />
        <EditUserModal
          open={openEdit}
          handleClose={handleCloseEdit}
          user={editUser}
          handleInputChange={handleInputChange}
          handleEditUserSubmit={EditUserSubmit}
        />
      </div>


      <DataGrid
        className="w-3/4 align-center justify-center m-auto"
        rows={users}

        columns={[
          { field: 'id', headerName: 'ID', width: 90 },
          { field: 'email', headerName: 'Email', width: 200, editable: true },
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
                  sx={{ color: 'info.main' }}
                  onClick={handleEditClick(id)}
                  color="inherit"
                />,
                <GridActionsCellItem
                  icon={<DeleteIcon />}
                  sx={{ color: 'error.main' }}
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
    </div >
  );
}
export default UserList;




