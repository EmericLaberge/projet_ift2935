import React from 'react';
import { Button, TextField } from '@mui/material';
import Game from './Entities/Game';
import { Modal, Box, Typography } from '@mui/material';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleEditGameSubmit: (e: React.FormEvent) => void;
  game: Game;
};

const EditGameModal: React.FC<Props> = ({
  open,
  handleClose,
  handleInputChange,
  handleEditGameSubmit,
  game,
}) => {
  return (
    <Modal open={open} onClose={handleClose}>
      <div className="modal-container">
        <Typography variant="h6">Edit Game</Typography>
        <TextField
          name="name"
          label="Name"
          value={game.name}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
          className="justify-content-center text-white label-white"
        />
        <TextField
          name="status"
          label="Status"
          value={game.status}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button variant="contained" color="primary" onClick={handleEditGameSubmit}>
            Save Changes
          </Button>
          <Button variant="contained" color="secondary" onClick={handleClose}>
            Cancel
          </Button>
        </Box>
      </div>
    </Modal>
  );
};

export default EditGameModal;

