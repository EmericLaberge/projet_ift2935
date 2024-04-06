import React, { useState, useEffect } from 'react';
import { DataGrid, GridActionsCellItem, GridRowId } from '@mui/x-data-grid';
import { Button, Chip, Switch, Typography } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import { toast } from 'react-hot-toast';
import Event from './Components/Entities/Event';
import AddEventModal from './Components/AddEventModal';
import EditEventModal from './Components/EditEventModal';
import { Status } from './Components/Entities/Event';

function EventList() {
  const [events, setEvents] = useState<Event[]>([]);
  const [userEvents, setUserEvents] = useState<Event[]>([]);
  const [fakeEvents, setFakeEvents] = useState<Event[]>([{ id: 1, name: 'Event 1', status: Status.NotStarted }]);

  const [error, setError] = useState<string | null>(null);
  const [editEvent, setEditEvent] = useState<Event>({ id: -1, name: '', status: Status.None });
  const [newEvent, setNewEvent] = useState<Event>({ id: -1, name: '', status: Status.None });
  const [open, setOpen] = useState(false);
  const [openEdit, setOpenEdit] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [isAllEvents, setIsAllEvents] = useState<boolean>(true);
  const apiEndpoint = 'http://127.0.0.1:6516';

  const handleEditClick = (id: GridRowId) => async () => {
    const response = await fetch(`${apiEndpoint}/events/${id}`);
    const data: Event = await response.json();
    setEditEvent(data);
    setOpenEdit(true);
  };

  async function handleViewClick() {
    setIsAllEvents(!isAllEvents);
  }

  const handleEditOpen = async (id: GridRowId) => {
    try {
      const response = await fetch(`${apiEndpoint}/events/${id}`);
      const data: Event = await response.json();
      if (!response.ok) {
        return toast.error('Failed to retrieve event');
      }
      setEditEvent(data);
      setOpenEdit(true);
    } catch (error) {
      setError('Failed to retrieve event');
    }
    {
      const response = await fetch(`${apiEndpoint}/events`);
      if (!response.ok) {
        throw new Error('Failed to fetch events');
      }
      const data: Event[] = await response.json();
      setEvents(data);
    }

    {
      const userId = localStorage.getItem('userId');
      const response = await fetch(`${apiEndpoint}/user_events/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your events');
      }
      const data: Event[] = await response.json();
      setUserEvents(data);
    }
  };
  const handleEditClose = () => setOpenEdit(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewEvent({ ...newEvent, [e.target.name]: e.target.value });
  };

  const handleNewEventSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch(`${apiEndpoint}/create_event`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newEvent),
      });
      if (!response.ok) {
        throw new Error('Failed to create event');
      }
      setNewEvent({ id: -1, name: '', status: Status.None });
      toast.success('Event created successfully');
    } catch (error) {
      setError('Failed to create event');
    }
    const userId = localStorage.getItem('userId');
    const response = await fetch(`${apiEndpoint}/user_events/${userId}`);
    if (!response.ok) {
      throw new Error('Failed to fetch your events');
    }
    const data: Event[] = await response.json();
    setUserEvents(data);
  };

  const handleDeleteClick = (id: GridRowId) => async () => {
    try {
      const response = await fetch(`${apiEndpoint}/events/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete event');
      }
      toast.success('Event deleted successfully');
    } catch (error) {
      setError('Failed to delete event');
    }
    {
      const response = await fetch(`${apiEndpoint}/events`);
      if (!response.ok) {
        throw new Error('Failed to fetch events');
      }
      const data: Event[] = await response.json();
      setEvents(data);
    }

    {
      const userId = localStorage.getItem('userId');
      const response = await fetch(`${apiEndpoint}/user_events/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your events');
      }
      const data: Event[] = await response.json();
      setUserEvents(data);
    }
  };

  useEffect(() => {
    const fetchEvents = async () => {
      try {
        const response = await fetch(`${apiEndpoint}/events`);
        if (!response.ok) {
          throw new Error('Failed to fetch events');
        }
        const data: Event[] = await response.json();
        setEvents(data);
      } catch (error) {
        setError('Failed to fetch events');
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
        <Typography> All Events</Typography>
        <Switch checked={!isAllEvents} onChange={() => handleViewClick()} inputProps={{ 'aria-label': 'controlled' }} />
        <Typography> My Events</Typography>
      </div>
      <h1 id="header-title">Event Management</h1>
      <div className="w-3/4 align-center justify-end m-auto flex mb-3">
        <Chip label="Add Event" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
      </div>
      <div>
        <AddEventModal open={open} handleClose={handleClose} handleInputChange={handleInputChange} handleNewEventSubmit={handleNewEventSubmit} newEvent={newEvent} />
        <EditEventModal open={openEdit} handleClose={handleEditClose} event={editEvent} handleInputChange={handleInputChange} handleEditEventSubmit={(event: Event) => {
          // Add your API call to update the event here
          // Example: const response = await fetch(`http://your-api-endpoint/events/${event.id}`, { method: 'PUT', body: JSON.stringify(event) });
          // Handle response and update state accordingly
          toast.success('Event updated successfully');
          handleEditClose();
        }} />
      </div>
      <DataGrid className="w-3/4 align-center justify-center m-auto" rows={fakeEvents} columns={[
        { field: 'id', headerName: 'ID', width: 90 },
        { field: 'name', headerName: 'Name', width: 200 },
        { field: 'status', headerName: 'Status', width: 150 },
        {
          field: 'actions',
          type: 'actions',
          headerName: 'Actions',
          width: 100,
          cellClassName: 'actions',
          getActions: ({ id }) => [
            <GridActionsCellItem icon={<EditIcon />} label="Edit" sx={{ color: 'info.main' }} onClick={() => handleEditOpen(id)} color="inherit" />,
            <GridActionsCellItem icon={<DeleteIcon />} label="Delete" sx={{ color: 'error.main' }} color="inherit" onClick={handleDeleteClick(id)} />,
          ],
        },
      ]} checkboxSelection />
    </div>
  );
}

export default EventList;

