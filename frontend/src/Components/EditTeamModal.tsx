import React from 'react';
import { Button, Modal, TextField, Typography } from '@mui/material';
import Team from './Entities/Team';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleEditTeamSubmit: (team: Team) => void;
  team: Team;
};

const EditTeamModal: React.FC<Props> = ({ open, handleClose, handleInputChange, handleEditTeamSubmit, team }) => {
  return (
    <Modal open={open} onClose={handleClose}>
      <div className="modal-container">
        <Typography variant="h6">Edit Team</Typography>
        <TextField
          name="name"
          label="Name"
          value={team.team_name}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
        />
        <TextField
          name="level"
          label="Level"
          value={team.level}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
        />
        <TextField
          name="type"
          label="Type"
          value={team.type}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
        />
        <Button variant="contained" color="primary" onClick={() => handleEditTeamSubmit(team)}>
          Save Changes
        </Button>
        <Button variant="contained" color="secondary" onClick={handleClose}>
          Cancel
        </Button>
      </div>
    </Modal>
  );
};

export default EditTeamModal;

