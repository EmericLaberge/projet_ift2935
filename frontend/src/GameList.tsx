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
import AddGameModal from './Components/AddGameModal';
import Game from './Components/Entities/Game';

function GameList() {
  const [games, setGames] = useState<Game[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [editGame, setEditGame] = useState<Game>({ id: -1, sport_name: '', event_name: '', event_date: '', team1_name: '', team2_name: '', final_score: '' });
  const [rowModesModel, setRowModesModel] = React.useState<GridRowModesModel>({});
  const [newGame, setNewGame] = useState<Partial<Game>>({ sport_name: '', event_name: '', event_date: '', team1_name: '', team2_name: '', final_score: '' });
  const [open, setOpen] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [openEdit, setOpenEdit] = useState(false);
  const [isAllGames, setIsAllGames] = useState(true);


  async function handleViewClick() {
    const userId = localStorage.getItem('userId');
    if (isAllGames) {
      // refetch all usersTeams
      setIsAllGames(false);
      const response = await fetch(`http://127.0.0.1:6969/user_games/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your games');
      }
      const data: Game[] = await response.json();
      setGames(data);
    }
    else {
      setIsAllGames(true);
      // refetch all teams
      const response = await fetch(`http://127.0.0.1:6969/games`);
      if (!response.ok) {
        throw new Error('Failed to fetch all games');
      }
      const data: Game[] = await response.json();
      setGames(data);
    }
  }




  const handleEditClick = (id: GridRowId) => async () => {
    // query the user by id 
    const response = await fetch(`http://127.0.0.1:6969/games/${id}`);
    const data: Game = await response.json();
    setEditGame(data);
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
    handleDeleteGameSubmit(id);
    handleCancelClick(id);
    setRowModesModel((prev) => {
      return {
        ...prev,
        [id]: { mode: GridRowModes.View },
      };
    });
  }
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setEditGame((prev) => ({
      ...prev,
      [e.target.name]: e.target.value,
    }));
  };

  const handleDeleteGameSubmit = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6969/games/${id}`, {
        method: 'DELETE',
      });
      if (!response.ok) {
        throw new Error('Failed to delete game');
      }
      toast.success('Game deleted successfully');
    } catch (error) {
      toast.error('Failed to delete game');

    }
    const response = await fetch('http://127.0.0.1:6969/games');
    if (!response.ok) {
      throw new Error('Failed to fetch games');
    }
    const data: Game[] = await response.json();
    setGames(data);
  };
  const handleNewGameSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('http://127.0.0.1:6969/create_game', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newGame),
      });
      if (!response.ok) {
        throw new Error('Failed to create game');
      }
      setNewGame({ sport_name: '', event_name: '', event_date: '', team1_name: '', team2_name: '', final_score: '' });
      toast.success('Game created successfully');

    } catch (error) {
      toast.error('Failed to create game');
    }

    const response = await fetch('http://127.0.0.1:6969/games');
    if (!response.ok) {
      throw new Error('Failed to fetch games');
    }

    const data: Game[] = await response.json();
    setGames(data);
  };

  const EditGameSubmit = async (game: Game) => {
    try {
      const response = await fetch(`http://127.0.0.1:6969/games/${game.id}`, {
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(game),
      });
      if (!response.ok) {
        throw new Error('Failed to update game');
      }
      toast.success('Game updated successfully');
    } catch (error) {
      toast.error('Failed to update game');
    }
    const response = await fetch('http://127.0.0.1:6969/games');
    if (!response.ok) {
      throw new Error('Failed to fetch games');
    }
    const data: Game[] = await response.json();
    setGames(data);
    setOpenEdit(false);

  };




  useEffect(() => {
    const fetchGames = async () => {
      try {
        const response = await fetch('http://127.0.0.1:6969/games');
        if (!response.ok) {
          throw new Error('Failed to fetch games');
        }
        const data: Game[] = await response.json();
        setGames(data);
      } catch (error) {
        toast.error('Failed to fetch games');
      }
    };
    fetchGames();
  }, []);

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (

    <div className="App">
      <div className="w-3/4 justify-items-start m-auto flex mt-3">
        <Stack direction="row" spacing={1} alignItems="center">
          <Typography variant="h6" component="h2">
            View My Games
          </Typography>
          <Switch
            checked={isAllGames}
            onChange={() => handleViewClick()}
            inputProps={{ 'aria-label': 'controlled' }}
          />
          <Typography variant="h6" component="h2">
            View All Games
          </Typography>
        </Stack>
      </div>
      <h1 id="header-title">Game Management</h1>
      <div className="w-3/4 align-center justify-end m-auto flex mb-3">
        <Chip label="Add Game" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
      </div>
      <div>
        <AddGameModal
          open={open}
          handleClose={handleClose}
          handleInputChange={(e) => setNewGame({ ...newGame, [e.target.name]: e.target.value })}
          handleNewGameSubmit={handleNewGameSubmit}
          newGame={newGame}
        />
      </div>


      <DataGrid
        className="w-3/4 align-center justify-center m-auto"
        rows={games}

        columns={[
          { field: 'id', headerName: 'ID', width: 70 },
          { field: 'sport_name', headerName: 'Sport Name', width: 130 },
          { field: 'event_name', headerName: 'Event Name', width: 250 },
          { field: 'event_date', headerName: 'Event Date', width: 130 },
          { field: 'team1_name', headerName: 'Team 1 Name', width: 130 },
          { field: 'team2_name', headerName: 'Team 2 Name', width: 130 },
          { field: 'final_score', headerName: 'Final Score', width: 130 },
          {
            field: 'actions',
            type: 'actions',
            headerName: 'Actions',
            width: 100,
            cellClassName: 'actions',
            getActions: ({ id }) => {
              return [
                <GridActionsCellItem
                  icon={<EditIcon />}
                  label="Edit"
                  sx={{ color: 'info.main' }}
                  onClick={handleEditClick(id)}
                  color="inherit"
                />,
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
export default GameList;
