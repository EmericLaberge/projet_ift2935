import React from 'react';
import { Modal, Box, Typography, TextField, Button } from '@mui/material';
import Game from './Entities/Game';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleNewGameSubmit: (e: React.FormEvent) => void;
  newGame: Partial<Game>;
};

const AddGameModal: React.FC<Props> = ({ open, handleClose, handleInputChange, handleNewGameSubmit, newGame }) => {
  return (
    <Modal
      open={open}
      onClose={handleClose}
      aria-labelledby="modal-modal-title"
      aria-describedby="modal-modal-description"
    >
      <Box className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 bg-black p-4 rounded-md shadow-lg border border-gray-700 bg-gray-900">
        <Typography id="modal-modal-title" variant="h6" component="h2">
          Add Game
        </Typography>
        <TextField
          name="sport_name"
          label="Sport Name"
          value={newGame.sport_name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="event_name"
          label="Event Name"
          value={newGame.event_name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="event_date"
          label="Event Date"
          value={newGame.event_date}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="team1_name"
          label="Team 1 Name"
          value={newGame.team1_name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="team2_name"
          label="Team 2 Name"
          value={newGame.team2_name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <TextField
          name="final_score"
          label="Final Score"
          value={newGame.final_score}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white  label-white"
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button onClick={handleClose} variant="contained" color="error">
            Cancel
          </Button>
          <Button onClick={handleNewGameSubmit} variant="contained" color="success">
            Add Game
          </Button>
        </Box>
      </Box>
    </Modal>
  );
};

export default AddGameModal;
