USE Jasson;
GO

CREATE OR ALTER FUNCTION getPlayersByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT UserID AS PlayerID, TeamID
FROM Players
WHERE
UserID = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT UserID AS PlayerID, TeamID
FROM Players
WHERE
TeamID = @identifiant);
GO

CREATE OR ALTER FUNCTION getEventsByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT EventID, TeamID
FROM TeamInEvent
WHERE
TeamID = @identifiant);
GO

CREATE OR ALTER FUNCTION getTeamsByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT EventID, TeamID
FROM TeamInEvent
WHERE
EventID = @identifiant)
GO

CREATE OR ALTER FUNCTION getGamesByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT ID AS GameID, SportName, EventID, FirstTeamID, SecondTeamID, GameDate, FinalScore
FROM Games
WHERE
FirstTeamID = @identifiant OR SecondTeamID = @identifiant);
GO

CREATE OR ALTER FUNCTION getGamesByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT ID AS GameID, SportName, EventID, FirstTeamID, SecondTeamID, GameDate, FinalScore
FROM Games
WHERE
EventID = @identifiant);
GO

CREATE OR ALTER FUNCTION getGamesBySportName(@identifiant NVARCHAR(50))
RETURNS TABLE
AS
RETURN
(SELECT ID AS GameID, SportName, EventID, FirstTeamID, SecondTeamID, GameDate, FinalScore
FROM Games
WHERE
SportName = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersBySportName(@identifiant NVARCHAR(50))
RETURNS TABLE
AS
RETURN
(SELECT Players.UserID AS PlayerID, Players.TeamID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.SportName = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersByLevel(@identifiant NVARCHAR(50))
RETURNS TABLE
AS
RETURN
(SELECT Players.UserID AS PlayerID, Players.TeamID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.TeamLevel = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersByTeamType(@identifiant NVARCHAR(50))
RETURNS TABLE
AS
RETURN
(SELECT Players.UserID AS PlayerID, Players.TeamID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.TeamType = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.UserID AS PlayerID, Players.TeamID
FROM Players
JOIN TeamInEvent ON Players.TeamID = TeamInEvent.TeamID
WHERE
TeamInEvent.EventID = @identifiant);
GO

CREATE OR ALTER FUNCTION getPlayersByGameId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.UserID AS PlayerID, Players.TeamID
FROM Players
JOIN Games ON Players.TeamID = Games.FirstTeamID OR Players.TeamID = Games.SecondTeamID
WHERE
Games.ID = @identifiant);
GO

CREATE OR ALTER FUNCTION getTeamsByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Teams.ID AS TeamID, Teams.Name, Teams.TeamLevel, Teams.TeamType, Teams.SportName
FROM Teams
JOIN Players ON Teams.ID = Players.TeamID
WHERE
Players.UserID = @identifiant);
GO

CREATE OR ALTER FUNCTION getEventsByPlayerId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT e.ID AS EventID, e.Name, e.StartDate, e.EndDate
FROM  (Players p JOIN TeamInEvent te ON p.UserID = @identifiant AND te.TeamID = p.TeamID)
JOIN Events e ON te.EventID = e.ID);
GO
--REQUÊTE COMPLEXE 4
CREATE OR ALTER FUNCTION getEventsByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT e.ID AS EventID, e.Name, e.StartDate, e.EndDate
FROM  (Players p JOIN TeamInEvent te ON p.UserID = @identifiant AND te.TeamID = p.TeamID)
JOIN Events e ON te.EventID = e.ID
UNION
SELECT e.ID AS EventID, e.Name, e.StartDate, e.EndDate
FROM StaffInEvent se JOIN Events e ON se.UserID = @identifiant AND se.EventID=e.ID);
GO

CREATE OR ALTER FUNCTION getGamesByPlayerId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Games.ID AS GameID, Games.SportName, Games.EventID, Games.FirstTeamID, Games.SecondTeamID, Games.GameDate, Games.FinalScore
FROM Games
JOIN Players ON Games.FirstTeamID = Players.TeamID OR Games.SecondTeamID = Players.TeamID
WHERE
Players.UserID = @identifiant);
GO

--REQUÊTE COMPLEXE 1
--Fonction pour obtenir la liste complète des utilisateurs participant à un évènement
--retourne une table de Users
CREATE OR ALTER FUNCTION getUsersByEventId(@EventID INT)
RETURNS TABLE
AS
RETURN (
    SELECT u.ID, u.Email, u.Address, u.FirstName, u.LastName
    FROM ((TeamInEvent t JOIN Players p ON p.TeamID = t.TeamID AND @EventID= t.EventID)
    JOIN Users u ON u.ID = p.UserID)
    UNION
    SELECT u.ID, u.Email, u.Address, u.FirstName, u.LastName
    FROM (StaffInEvent st JOIN Users u ON @EventID = st.EventID AND u.ID = st.UserID)
);
GO

--REQUÊTE COMPLEXE 2
--fonction pour obtenir la table des joueurs participant à un certain évènement
--et faisant partie d'une équipe d'un certain niveau
CREATE OR ALTER FUNCTION getPlayersByEventAndLevel(@EventID INT, @TeamLevel NVARCHAR(50))
RETURNS TABLE
AS
RETURN (
    SELECT UserID AS PlayerID, p.TeamID
    FROM (TeamInEvent te JOIN Teams t ON t.[TeamLevel] = @TeamLevel AND te.TeamID = t.ID AND @EventID = te.EventID)
    JOIN Players p ON te.TeamID = p.TeamID
);
GO

--REQUÊTE COMPLEXE 3
--fonction pour obtenir la table des joueurs qui jouent dans une équipe d'un certain sport
--et dont l'équipe est d'un certain niveau
CREATE OR ALTER FUNCTION getPlayersBySportAndLevel(@SportName NVARCHAR(50), @TeamLevel NVARCHAR(50))
RETURNS TABLE
AS
RETURN (
    SELECT UserID AS PlayerID, p.TeamID
    FROM Players p JOIN Teams t ON p.TeamID = t.ID AND @SportName = t.SportName AND @TeamLevel = t.TeamLevel
);
GO

--Fonction pour obtenir le format du score d'un sport
CREATE OR ALTER FUNCTION getScoreFormat(@SportName NVARCHAR(50))
RETURNS NVARCHAR(50)
AS
BEGIN
    RETURN (
        SELECT ScoreFormat FROM Sports WHERE @SportName = Name
    )
END;
GO

--Vue pour voir le top 5 des équipes ayant joué le plus de parties
CREATE OR ALTER VIEW [Top 5 Teams with most games played] AS
SELECT TOP 5 Teams.Name, Teams.SportName AS [Sport], COUNT(Games.ID) AS [Games]
FROM Teams JOIN Games ON Teams.ID = Games.FirstTeamID OR Teams.ID = Games.SecondTeamID
GROUP BY Teams.ID, Name, Teams.SportName
ORDER BY COUNT(Games.ID);
GO

--Vue pour voir les 5 évènements les plus populaires présentement en cours
CREATE OR ALTER VIEW [Most popular ongoing events] AS
SELECT TOP 5 e.Name, e.StartDate, e.EndDate
FROM ((Events e JOIN TeamInEvent te ON TeamID = ID) JOIN Players p ON p.TeamID=te.TeamID)
WHERE e.StartDate<GETDATE() AND e.EndDate>GETDATE()
GROUP BY EventID, e.Name, e.StartDate, e.EndDate
ORDER BY COUNT(DISTINCT p.UserID);
GO

--REQUÊTE COMPLEXE 5
--Vue pour afficher les attributs se ratachant de près ou de loins à une partie
CREATE OR ALTER VIEW [displayGame] AS
SELECT g.ID, g.sportName, e.Name AS [Event name], t1.Name AS [Team 1], t2.Name AS [Team 2], g.GameDate, g.FinalScore
FROM ((Games g JOIN Teams t1 ON g.FirstTeamID = t1.ID) JOIN Teams t2 ON g.SecondTeamID = t2.ID) JOIN Events e ON g.EventID = e.ID
;
GO

-- Fonction pour récupérer les joueurs d'une équipe
CREATE OR ALTER FUNCTION fnGetPlayers (@TeamID INT)
RETURNS TABLE
AS
RETURN (
    SELECT u.ID, u.Email, u.Address, u.FirstName, u.LastName
    FROM Players p
    JOIN Users u ON p.UserID = u.ID
    WHERE p.TeamID = @TeamID
);
GO

IF OBJECT_ID('EventsView') IS NOT NULL
    DROP VIEW EventsView;
GO

CREATE VIEW EventsView AS
SELECT
    ID,
    Name,
    CONVERT(NVARCHAR(10), StartDate, 23) AS StartDateAsString,
    CONVERT(NVARCHAR(10), EndDate, 23) AS EndDateAsString
FROM Events;
GO

IF OBJECT_ID ('GamesView') IS NOT NULL
    DROP VIEW GamesView;
GO

CREATE VIEW GamesView  AS
SELECT
    ID,
    SportName,
    EventID,
    FirstTeamID,
    SecondTeamID,
    CONVERT(NVARCHAR(10), GameDate, 23) AS GameDateAsString,
    FinalScore
    FROM Games;
GO





