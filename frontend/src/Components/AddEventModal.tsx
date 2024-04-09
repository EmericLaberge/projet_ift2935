import React from 'react';
import { Button, TextField } from '@mui/material';
import Event from './Entities/Event';
import { Modal, Box, Typography } from '@mui/material';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleNewEventSubmit: (e: React.FormEvent) => void;
  newEvent: Partial<Event>;
};

const AddEventModal: React.FC<Props> = ({
  open,
  handleClose,
  handleInputChange,
  handleNewEventSubmit,
  newEvent,
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
          Add Event
        </Typography>
        <TextField
          name="name"
          label="Name"
          value={newEvent.name}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%', color: 'white' }}
          className="justify-content-center text-white label-white"
        />
        <TextField
          name="date"
          label="Start Date"
          type="date"
          value={newEvent.start_date}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }}
        />
        <TextField 
          name="date"
          label="End Date"
          type="date"
          value={newEvent.end_date}
          onChange={handleInputChange}
          margin="normal"
          sx={{ width: '100%' }} 
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button onClick={handleClose} variant="contained" color="error">
            Cancel
          </Button>
          <Button onClick={handleNewEventSubmit} variant="contained" color="success">
            Add Event
          </Button>
        </Box>
      </div>
    </Modal>
  );
};

export default AddEventModal;
