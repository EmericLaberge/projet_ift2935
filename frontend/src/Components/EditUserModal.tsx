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
      <Box sx={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', width: 400, bgcolor: 'background.paper', border: '2px solid #000', boxShadow: 24, p: 4 }}>
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Edit User
        </Typography>
        <TextField
          name="email"
          label="Email"
          value={user.email}
          onChange={handleInputChange}
          margin="normal"
          className="justify-content-center"
        />
        <TextField
          name="address"
          label="Address"
          value={user.address}
          onChange={handleInputChange}
          margin="normal"
        />
        <TextField
          name="first_name"
          label="First Name"
          value={user.first_name}
          onChange={handleInputChange}
          margin="normal"
        />
        <TextField
          name="last_name"
          label="Last Name"
          value={user.last_name}
          onChange={handleInputChange}
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
      </Box>
    </Modal>
  );
};

export default EditUserModal;

