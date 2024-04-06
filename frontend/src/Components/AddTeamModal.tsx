import React from 'react';
import { Button, Modal, TextField, Typography } from '@mui/material';
import Team from './Entities/Team';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleNewTeamSubmit: (e: React.FormEvent) => void;
  newTeam: Partial<Team>;
};

const AddTeamModal: React.FC<Props> = ({ open, handleClose, handleInputChange, handleNewTeamSubmit, newTeam }) => {
  return (
    <Modal
      open={open}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
    <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 bg-black p-4 rounded-md shadow-lg border border-gray-700 bg-gray-900">
      <Typography id="modal-modal-title" variant="h6" component="h2">
        Add Team
      </Typography>
      <TextField
        name="name"
        label="Name"
        value={newTeam.name}
        onChange={handleInputChange}
        margin="normal"
        sx={{ width: '100%' }}
      />
      <TextField
        name="level"
        label="Level"
        value={newTeam.level}
        onChange={handleInputChange}
        margin="normal"
        sx={{ width: '100%' }}
      />
      <TextField
        name="type"
        label="Type"
        value={newTeam.type}
        onChange={handleInputChange}
        margin="normal"
        sx={{ width: '100%' }}
      />
      <TextField
        name="sport"
        label="Sport"
        value={newTeam.sport}
        onChange={handleInputChange}
        margin="normal"
        sx={{ width: '100%' }}
      />
      <div className="flex justify-between">
      <Button onClick={handleClose} variant="contained" color="error">
        Cancel
      </Button>
      <Button onClick={handleNewTeamSubmit} variant="contained" color="success">
        Add Team
      </Button>
      </div>
    </div>
    </Modal>
  );
};
export default AddTeamModal;
