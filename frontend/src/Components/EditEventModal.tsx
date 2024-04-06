import React from 'react';
import { Button, TextField } from '@mui/material';
import Event from './Entities/Event';
import { Modal, Box, Typography } from '@mui/material';

type Props = {
  open: boolean;
  handleClose: () => void;
  handleInputChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  handleEditEventSubmit: (e: React.FormEvent) => void;
  event: Event;
};

const EditEventModal: React.FC<Props> = ({
  open,
  handleClose,
  handleInputChange,
  handleEditEventSubmit,
  event,
}) => {
  return (
    <Modal open={open} onClose={handleClose}>
      <div className="modal-container">
        <Typography variant="h6">Edit Event</Typography>
        <TextField
          name="name"
          label="Name"
          value={event.name}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
          className="justify-content-center text-white label-white"
        />
        <TextField
          name="date"
          label="Date"
          type="date"
          value={event.date}
          onChange={handleInputChange}
          margin="normal"
          fullWidth
        />
        <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
          <Button variant="contained" color="primary" onClick={handleEditEventSubmit}>
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
