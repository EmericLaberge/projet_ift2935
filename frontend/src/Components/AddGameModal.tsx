import React from 'react';
import { Button, TextField } from '@mui/material';
import Game from './Entities/Game';
import { Modal, Box, Typography } from '@mui/material';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleNewGameSubmit: (e: React.FormEvent) => void;
  newGame: Partial<Game>;
};

const AddGameModal: React.FC<Props> = ({
  open,
  handleClose,
  handleInputChange,
  handleNewGameSubmit,
  newGame,
}) => {
  return (
    <Modal
      open={open}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 bg-black p-4 rounded-md shadow-lg border border-gray-700 bg-gray-900">
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Add Game
        </Typography>
        <TextField
          name="name"
          label="Name"
          value={newGame.name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white label-white"
        />
        <TextField
          name="status"
          label="Status"
          value={newGame.status}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button onClick={handleClose} variant="contained" color="error">
            Cancel
          </Button>
          <Button onClick={handleNewGameSubmit} variant="contained" color="success">
            Add Game
          </Button>
        </Box>
      </div>
    </Modal>
  );
};

export default AddGameModal;

