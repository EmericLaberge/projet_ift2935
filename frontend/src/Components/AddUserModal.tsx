import React from 'react';
import { Dialog, DialogActions, DialogContent, DialogTitle, Button, TextField } from '@mui/material';
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
      <Box sx={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', width: 400, bgcolor: 'background.paper', border: '2px solid #000', boxShadow: 24, p: 4 }}>
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Add User
        </Typography>
        <TextField
          name="email"
          label="Email"
          value={newUser.email}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
          className="justify-content-center"
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
      </Box>
    </Modal>
  );
};

export default AddUserModal;
