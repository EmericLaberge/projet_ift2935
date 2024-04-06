import React, { useState, useEffect } from 'react';
import { DataGrid, GridActionsCellItem, GridRowId, GridRowModes } from '@mui/x-data-grid';
import { Button, Chip, Modal, Stack, Switch, TextField, Typography } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import { toast } from 'react-hot-toast';
import Game from './Components/Entities/Game';
import AddGameModal from './Components/AddGameModal';
import EditGameModal from './Components/EditGameModal';
import { Status } from './Components/Entities/Game';

function GameList() {
  const [games, setGames] = useState<Game[]>([]);
  const [userGames, setUserGames] = useState<Game[]>([]);
  const [fakeGames, setFakeGames] = useState<Game[]>([{ id: 1, name: 'Game 1', status: Status.NotStarted }]);

  const [error, setError] = useState<string | null>(null);
  const [editGame, setEditGame] = useState<Game>({ id: -1, name: '', status: Status.None });
  const [newGame, setNewGame] = useState<Game>({ id: -1, name: '', status: Status.None });
  const [open, setOpen] = useState(false);
  const [openEdit, setOpenEdit] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [isAllGames, setIsAllGames] = useState<boolean>(true);
  const apiEndpoint = 'http://127.0.0.1:6516';

  const handleEditClick = (id: GridRowId) => async () => {
    const response = await fetch(`${apiEndpoint}/games/${id}`);
    const data: Game = await response.json();
    setEditGame(data);
    setOpenEdit(true);
  }

  async function handleViewClick() {
    setIsAllGames(!isAllGames);
  }

  const handleEditOpen = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/games/${id}`);
      const data: Game = await response.json();
      if (!response.ok) {
        return toast.error('Failed to retrieve game');
      }
      setEditGame(data);
      setOpenEdit(true);
    } catch (error) {
      setError('Failed to retrieve game');
    }
    {
      const response = await fetch(`${apiEndpoint}/games`);
      if (!response.ok) {
        throw new Error('Failed to fetch games');
      }
      const data: Game[] = await response.json();
      setGames(data);
    }

    {
      const userId = localStorage.getItem('userId');
      const response = await fetch(`${apiEndpoint}/user_games/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your games');
      }
      const data: Game[] = await response.json();
      setUserGames(data);
    }
  }
  const handleEditClose = () => setOpenEdit(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewGame({ ...newGame, [e.target.name]: e.target.value });
  };

  const handleNewGameSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch(`${apiEndpoint}/create_game`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newGame),
      });
      if (!response.ok) {
        throw new Error('Failed to create game');
      }
      setNewGame({ id: -1, name: '', status: Status.None });
      toast.success('Game created successfully');
    } catch (error) {
      setError('Failed to create game');
    }
    const userId = localStorage.getItem('userId');
    const response = await fetch(`${apiEndpoint}/user_games/${userId}`);
    if (!response.ok) {
      throw new Error('Failed to fetch your games');
    }
    const data: Game[] = await response.json();
    setUserGames(data);
  };

  const handleDeleteClick = (id: GridRowId) => async () => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/games/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete game');
      }
      toast.success('Game deleted successfully');
    } catch (error) {
      setError('Failed to delete game');
    }
    {
      const response = await fetch(`http://127.0.0.1:6516/games`);
      if (!response.ok) {
        throw new Error('Failed to fetch games');
      }
      const data: Game[] = await response.json();
      setGames(data);
    }

    {
      const userId = localStorage.getItem('userId');
      const response = await fetch(`http://127.0.0.1:6516/user_games/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your games');
      }
      const data: Game[] = await response.json();
      setUserGames(data);
    }
  }

  useEffect(() => {
    const fetchGames = async () => {
      try {
        const response = await fetch(`http://127.0.0.1:6516/games`);
        if (!response.ok) {
          throw new Error('Failed to fetch games');
        }
        const data: Game[] = await response.json();
        setGames(data);
      } catch (error) {
        setError('Failed to fetch games');
      }
    }
    fetchGames();
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (
    <div className="App">
      <div className="w-3/4 justify-items-start m-auto flex mt-3">
        <Stack direction="row" spacing={1} alignItems="center">
          <Typography> All Games</Typography>
          <Switch checked={!isAllGames} onChange={() => handleViewClick()} inputProps={{ 'aria-label': 'controlled' }} />
          <Typography> My Games</Typography>
        </Stack>
      </div>
      <h1 id="header-title">Game Management</h1>
      <div className="w-3/4 align-center justify-end m-auto flex mb-3">
        <Chip label="Add Game" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
      </div>

      <div>
        <AddGameModal open={open} handleClose={handleClose} handleInputChange={handleInputChange} handleNewGameSubmit={handleNewGameSubmit} newGame={newGame} />
        <EditGameModal open={openEdit} handleClose={handleEditClose} game={editGame} handleInputChange={handleInputChange} handleEditGameSubmit={(game: Game) => {
          // Add your API call to update the game here
          // Example: const response = await fetch(`http://your-api-endpoint/games/${game.id}`, { method: 'PUT', body: JSON.stringify(game) });
          // Handle response and update state accordingly
          toast.success('Game updated successfully');
          handleEditClose();
        }} />

      </div>

      <DataGrid className="w-3/4 align-center justify-center m-auto" rows={fakeGames} columns={[
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
export default GameList;
