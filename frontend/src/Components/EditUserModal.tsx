import React from 'react';
import User from './Entities/User';
import { Box, Button, Typography, TextField } from '@mui/material';
import { Modal } from '@mui/material';



type Props = {
  open: boolean;
  handleClose: () => void;
  user: User;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleEditUserSubmit: (user: User) => void;
};

const EditUserModal: React.FC<Props> = ({ open, handleClose, user, handleInputChange, handleEditUserSubmit }) => {
  return (
    <Modal
      open={open}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 bg-black p-4 rounded-md shadow-lg border border-gray-700 bg-gray-900">
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Edit User
        </Typography>
        <TextField
          disabled
          name="email"
          label="Email"
          value={user.email}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
        />
        <TextField
          name="address"
          label="Address"
          value={user.address}
          onChange={handleInputChange}
          sx={{ width: '100%' }}
          margin="normal"
        />
        <TextField
          name="first_name"
          label="First Name"
          value={user.first_name}
          onChange={handleInputChange}
          sx={{ width: '100%' }}
          margin="normal"
        />
        <TextField
          name="last_name"
          label="Last Name"
          value={user.last_name}
          onChange={handleInputChange}
          sx={{ width: '100%' }}
          margin="normal"
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button onClick={handleClose} variant="contained" color="error">
            Cancel
          </Button>
          <Button onClick={() => handleEditUserSubmit(user)} variant="contained" color="success">
            Save
          </Button>
        </Box>
      </div>
    </Modal>
  );
};

export default EditUserModal;
