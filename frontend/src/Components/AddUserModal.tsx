import React from 'react';
import { Button, TextField } from '@mui/material';
import User from './Entities/User';

import { Modal, Box, Typography } from '@mui/material';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleNewUserSubmit: (e: React.FormEvent) => void;
  newUser: Partial<User>;
};

const AddUserModal: React.FC<Props> = ({ open, handleClose, handleInputChange, handleNewUserSubmit, newUser }) => {
  return (
    <Modal
      open={open}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 bg-black p-4 rounded-md shadow-lg border border-gray-700 bg-gray-900">
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Add User
        </Typography>
        <TextField
          name="email"
          label="Email"
          value={newUser.email}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="address"
          label="Address"
          value={newUser.address}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
        />
        <TextField
          name="first_name"
          label="First Name"
          value={newUser.first_name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
        />
        <TextField
          name="last_name"
          label="Last Name"
          value={newUser.last_name}
          onChange={handleInputChange}
          sx={{ width: '100%' }}
          margin="normal"
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button onClick={handleClose} variant="contained" color="error">
            Cancel
          </Button>
          <Button onClick={handleNewUserSubmit} variant="contained" color="success">
            Add User
          </Button>
        </Box>
      </div>
    </Modal>
  );
};

export default AddUserModal;
