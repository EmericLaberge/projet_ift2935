
import React, { useEffect, useState } from 'react';
import { Box, Card, CardContent, TextField, Button, Typography, Chip, Switch, Stack } from '@mui/material';
import { List, ListItem, ListItemText, ListItemSecondaryAction, IconButton } from '@mui/material';
import { DataGrid, GridActionsCellItem } from '@mui/x-data-grid';
import { toast } from 'react-hot-toast';
import SaveIcon from '@mui/icons-material/Save';
import CancelIcon from '@mui/icons-material/Cancel';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import Modal from '@mui/material/Modal';
import AddIcon from '@mui/icons-material/Add';
import './App.css';
import {
  GridRowsProp,
  GridRowModesModel,
  GridRowModes,
  GridColDef,
  GridToolbarContainer,
  GridEventListener,
  GridRowId,
  GridRowModel,
  GridRowEditStopReasons,
  GridSlots,
} from '@mui/x-data-grid';
import AddEventModal from './Components/AddEventModal';
import Event from './Components/Entities/Event';

function EventList() {
  const [events, setEvents] = useState<Event[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [editEvent, setEditEvent] = useState<Event>({id:-1, name: '', start_date: '', end_date: '' })
  const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>({});
  const [newEvent, setNewEvent] = useState<Partial<Event>>({ name: '', start_date: '', end_date: '' });
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [openEdit, setOpenEdit] = useState(false);
  const [isAllEvents, setIsAllEvents] = useState(true);


  async function handleViewClick() {
    const userId = localStorage.getItem('userId');
    if (isAllEvents) {
      // refetch all usersTeams
      setIsAllEvents(false);
      const response = await fetch(`http://127.0.0.1:6516/user_events/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your events');
      }
      const data: Event[] = await response.json();
      setEvents(data);
    }
    else {
      setIsAllEvents(true);
      // refetch all teams
      const response = await fetch(`http://127.0.0.1:6516/events`);
      if (!response.ok) {
        throw new Error('Failed to fetch all events');
      }
      const data: Event[] = await response.json();
      setEvents(data);
    }
  }




  const handleEditClick = (id: GridRowId) => async () => {
    // query the user by id 
    const response = await fetch(`http://127.0.0.1:6516/events/${id}`);
    const data: Event = await response.json();
    setEditEvent(data);
    setOpenEdit(true);

    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.Edit },
      };
    });
  }

  const handleSaveClick = (id: GridRowId) => () => {
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }

  const handleCancelClick = (id: GridRowId) => () => {
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }

  const handleDeleteClick = (id: GridRowId) => () => {
    handleDeleteEventSubmit(id);
    handleCancelClick(id);
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEditEvent((prev) => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleDeleteEventSubmit = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/events/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete event');
      }
      toast.success('Event deleted successfully');
    } catch (error) {
      toast.error('Failed to delete event');

    }
    const response = await fetch('http://127.0.0.1:6516/events');
    if (!response.ok) {
      throw new Error('Failed to fetch events');
    }
    const data: Event[] = await response.json();
    setEvents(data);
  };
  const handleNewEventSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('http://127.0.0.1:6516/create_event', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newEvent),
      });
      if (!response.ok) {
        throw new Error('Failed to create event');
      }
      setNewEvent({ name: '', start_date: '', end_date: '' });
      toast.success('Event created successfully');

    } catch (error) {
      toast.error('Failed to create event');
    }

    const response = await fetch('http://127.0.0.1:6516/events');
    if (!response.ok) {
      throw new Error('Failed to fetch events');
    }

    const data: Event[] = await response.json();
    setEvents(data);
  };

  const EditEventSubmit = async (event: Event) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/events/${event.id}`, {
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(event),
      });
      if (!response.ok) {
        throw new Error('Failed to update event');
      }
      toast.success('Event updated successfully');
    } catch (error) {
      toast.error('Failed to update event');
    }
    const response = await fetch('http://127.0.0.1:6516/events');
    if (!response.ok) {
      throw new Error('Failed to fetch events');
    }
    const data: Event[] = await response.json();
    setEvents(data);
    setOpenEdit(false);

  };




  useEffect(() => {
    const fetchEvents = async () => {
      try {
        const response = await fetch('http://127.0.0.1:6516/events');
        if (!response.ok) {
          throw new Error('Failed to fetch events');
        }
        const data: Event[] = await response.json();
        setEvents(data);
      } catch (error) {
        toast.error('Failed to fetch events');
      }
    };
    fetchEvents();
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (

    <div className="App">
      <div className="w-3/4 justify-items-start m-auto flex mt-3">
        <Stack direction="row" spacing={1} alignItems="center">
          <Typography variant="h6" component="h2">
            View My Events
          </Typography>
          <Switch
            checked={isAllEvents}
            onChange={() => handleViewClick()}
            inputProps={{ 'aria-label': 'controlled' }}
          />
          <Typography variant="h6" component="h2">
            View All Events
          </Typography>
        </Stack>
      </div>
      <h1 id="header-title">Event Management</h1>
      <div className="w-3/4 align-center justify-end m-auto flex mb-3">
        <Chip label="Add Event" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
      </div>
      <div>
        <AddEventModal
          open={open}
          handleClose={handleClose}
          handleInputChange={(e) => setNewEvent({ ...newEvent, [e.target.name]: e.target.value })}
          handleNewEventSubmit={handleNewEventSubmit}
          newEvent={newEvent}
        />
      </div>


      <DataGrid
        className="w-3/4 align-center justify-center m-auto"
        rows={events}

        columns={[
          { field: 'id', headerName: 'ID', width: 70 },
          { field: 'name', headerName: 'Name', width: 130 },
          { field: 'start_date', headerName: 'Start Date', width: 130 },
          { field: 'end_date', headerName: 'End Date', width: 130 },
          {
            field: 'actions',
            type: 'actions',
            headerName: 'Actions',
            width: 100,
            cellClassName: 'actions',
            getActions: ({ id }) => {
              return [
                <GridActionsCellItem
                  icon={<DeleteIcon />}
                  sx={{ color: 'error.main' }}
                  label="Delete"
                  onClick={handleDeleteClick(id)}
                  color="inherit"
                />,
              ];
            }
          },
        ]}
        checkboxSelection
      />
    </div >
  );
}
export default EventList;
