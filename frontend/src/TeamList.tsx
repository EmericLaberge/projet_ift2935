import React, { useEffect, useState } from 'react';
import { DataGrid, GridActionsCellItem, GridRowId, GridRowModes } from '@mui/x-data-grid';
import { Button, Chip, Modal, Switch, TextField, Typography } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import EditIcon from '@mui/icons-material/Edit';
import DeleteIcon from '@mui/icons-material/Delete';
import { toast } from 'react-hot-toast';
import Team from './Components/Entities/Team';
import AddTeamModal from './Components/AddTeamModal';
import EditTeamModal from './Components/EditTeamModal';
import { Level, Type, Sport } from './Components/Entities/Team';
import { log } from 'console';

function TeamList() {
  const [teams, setTeams] = useState<Team[]>([]);
  const [userTeams, setUserTeams] = useState<Team[]>([]);
  const [fakeTeams, setFakeTeams] = useState<Team[]>([{ id: 1, team_name: 'Team 1', level: Level.Junior, type: Type.Masculine, sport: Sport.Soccer }]);

  const [error, setError] = useState<string | null>(null);
  const [editTeam, setEditTeam] = useState<Team>({ id: -1, team_name: '', level: Level.None, type: Type.None, sport: Sport.None });
  const [newTeam, setNewTeam] = useState<Team>({ id: -1, team_name: '', level: Level.None, type: Type.None, sport: Sport.None });
  const [open, setOpen] = useState(false);
  const [openEdit, setOpenEdit] = useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);
  const [isAllTeams, setIsAllTeams] = useState<boolean>(true);
  const apiEndpoint = 'http://127.0.0.1:6516';



  const handleEditClick = (id: GridRowId) => async () => {
    // query the user by id 
    const response = await fetch(`http://127.0.0.1:6516/teams/${id}`);
    const data: Team = await response.json();
    setEditTeam(data);
    setOpenEdit(true);
  }


  async function handleViewClick() {
    const userId = localStorage.getItem('userId');
    if (isAllTeams) {
      // refetch all usersTeams
      setIsAllTeams(false);
      const response = await fetch(`http://127.0.0.1:6516/user_teams/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your teams');
      }
      const data: Team[] = await response.json();
      setUserTeams(data);
    }
    else {
      setIsAllTeams(true);
      // refetch all teams
      const response = await fetch(`http://127.0.0.1:6516/teams`);
      if (!response.ok) {
        throw new Error('Failed to fetch teams');
      }
      const data: Team[] = await response.json();
      setTeams(data);
    }
  }




  const handleEditOpen = async (id: GridRowId) => {
    try {
      const response = await fetch(`http://127.0.0.1:6516/teams/${id}`)
      const data: Team = await response.json();
      if (!response.ok) {
        return toast.error('Failed to retrieve team');
      }
      setEditTeam(data);
      setOpenEdit(true);
    } catch (error) {
      setError('Failed to retrieve team');
    }
    {
      // refetch teams
      const response = await fetch('http://127.0.0.1:6516/teams');
      if (!response.ok) {
        throw new Error('Failed to fetch teams');
      }
      const data: Team[] = await response.json();
      setTeams(data);
    }

    {
      // refetch user teams 
      const userId = localStorage.getItem('userId');
      const response = await fetch(`http://127.0.0.1:6516/user_teams/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your teams');
      }
      const data: Team[] = await response.json();
      setUserTeams(data);
    }
  }
  const handleEditClose = () => setOpenEdit(false);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setNewTeam({ ...newTeam, [e.target.name]: e.target.value });
  };


  const handleNewTeamSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const response = await fetch('http://127.0.0.1:6516/create_team', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(newTeam),
      });
      if (!response.ok) {
        throw new Error('Failed to create team');
      }
      setNewTeam({ id: -1, team_name: '', level: Level.None, type: Type.None, sport: Sport.None }); // Reset form fields
      toast.success('Team created successfully');
    } catch (error) {
      setError('Failed to create team');
    }
    if (isAllTeams) {
      // refetch all teams
      const response = await fetch('http://127.0.0.1:6516/teams');
      if (!response.ok) {
        throw new Error('Failed to fetch teams');
      }
      const data: Team[] = await response.json();
      setTeams(data);
    }
    else {

      // refetch user teams
      const userId = localStorage.getItem('userId');
      const response = await fetch(`http://127.0.0.1:6516/user_teams/${userId}`);
      if (!response.ok) {
        throw new Error('Failed to fetch your teams');
      }
      const data: Team[] = await response.json();
      setUserTeams(data);
    }
  }
      

      const handleDeleteClick = (id: GridRowId) => async () => {
        try {
          const response = await fetch(`http://127.0.0.1:6516/teams/${id}`, {
            method: 'DELETE',
          });
          if (!response.ok) {
            throw new Error('Failed to delete team');
          }
          toast.success('Team deleted successfully');
        } catch (error) {
          setError('Failed to delete team');
        }
        {
          // refetch teams
          const response = await fetch('http://127.0.0.1:6516/teams');
          if (!response.ok) {
            throw new Error('Failed to fetch teams');
          }
          const data: Team[] = await response.json();
          setTeams(data);
        }

        {
          // refetch user teams
          const userId = localStorage.getItem('userId');
          const response = await fetch(`http://127.0.0.1:6516/user_teams/${userId}`);
          if (!response.ok) {
            throw new Error('Failed to fetch your teams');
          }
          const data: Team[] = await response.json();
          setUserTeams(data);
        }
      }



      useEffect(() => {
        const fetchTeams = async () => {
          try {
            const response = await fetch('http://127.0.0.1:6516/teams');
            if (!response.ok) {
              throw new Error('Failed to fetch teams');
            }
            const data: Team[] = await response.json();
            console.log(data);

            setTeams(data);
          } catch (error) {
            setError('Failed to fetch teams');
          }
        }
        // const fetchUserTeams = async () => {
        //   try {
        //     const response = await fetch('http://127.0.0.1:6516/user_teams');
        //     if (!response.ok) {
        //       throw new Error('Failed to fetch user teams');
        //     }
        //     const data: Team[] = await response.json();
        //     setUserTeams(data);
        //   } catch (error) {
        //     setError('Failed to fetch user teams');
        //   }
        // }
        fetchTeams();
        // fetchUserTeams();
      }, []);




      if (error) {
        return <div>Error: {error}</div>;
      }


      return (
        <div className="App">
          <div className="w-3/4 justify-items-start m-auto flex mt-3">
            <Switch
              checked={isAllTeams}
              onChange={() => handleViewClick()}
              inputProps={{ 'aria-label': 'controlled' }}
            />
            <Typography variant="h6" component="h2">
              {isAllTeams ? 'View All Teams' : 'View My Teams'}
            </Typography>
          </div>
          <h1 id="header-title">Team Management</h1>
          <div className="w-3/4 align-center justify-end m-auto flex mb-3">
            <Chip label="Add Team" icon={<AddIcon />} color="primary" onClick={handleOpen} className="mb-3 p-2 justify-end align-center font-bold" size="medium" />
          </div>
          <div>
            <AddTeamModal
              open={open}
              handleClose={handleClose}
              handleInputChange={handleInputChange}
              handleNewTeamSubmit={handleNewTeamSubmit} newTeam={newTeam}
            />
            <EditTeamModal
              open={openEdit} handleClose={handleEditClose}
              team={editTeam}
              handleInputChange={handleInputChange}
              handleEditTeamSubmit={(team: Team) => {
                // Add your API call to update the team here
                // Example: const response = await fetch(`http://your-api-endpoint/teams/${team.id}`, { method: 'PUT', body: JSON.stringify(team) });
                // Handle response and update state accordingly
                toast.success('Team updated successfully');
                handleEditClose();
              }}
            />
          </div>
          <DataGrid
            className="w-3/4 align-center justify-center m-auto"
            rows={teams}
            columns={[
              { field: 'id', headerName: 'ID', width: 90 },
              { field: 'team_name', headerName: 'Team Name', width: 150 },
              { field: 'level', headerName: 'Level', width: 150 },
              { field: 'type', headerName: 'Type', width: 150 },
              { field: 'sport', headerName: 'Sport', width: 150 },
              {
                field: 'actions',
                type: 'actions',
                headerName: 'Actions',
                width: 100,
                cellClassName: 'actions',
                getActions: ({ id }) => [
                  <GridActionsCellItem
                    icon={<EditIcon />}
                    label="Edit"
                    sx={{ color: 'info.main' }}
                    onClick={() => handleEditOpen(id)}
                    color="inherit"
                  />,
                  <GridActionsCellItem
                    icon={<DeleteIcon />}
                    label="Delete"
                    sx={{ color: 'error.main' }}
                    color="inherit"
                    onClick={handleDeleteClick(id)}
                  />,
                ],
              },
            ]}
            checkboxSelection
          />
        </div>
      );
    }
    export default TeamList;
