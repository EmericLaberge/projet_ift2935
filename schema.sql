use master;
IF NOT EXISTS (SELECT * FROM sys.databases WHERE name = 'Jasson')
BEGIN
    CREATE DATABASE Jasson;
END
GO
USE Jasson;
GO

IF OBJECT_ID('Users') IS NULL
BEGIN
    CREATE TABLE Users (
        ID INT IDENTITY PRIMARY KEY,
        Email NVARCHAR(50) UNIQUE NOT NULL,
        Address NVARCHAR(50) NOT NULL,
        FirstName NVARCHAR(50) NOT NULL,
        LastName NVARCHAR(50) NOT NULL
    );
END
GO

IF OBJECT_ID('Staff') IS NULL
BEGIN
    CREATE TABLE Staff (
        ID INT IDENTITY PRIMARY KEY,
        UserID INT NOT NULL,
        NAS NVARCHAR(50) NOT NULL,
        HiringDate DATE NOT NULL,
        FOREIGN KEY (UserID) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO
  
IF OBJECT_ID('Sports') IS NULL 
BEGIN
    CREATE TABLE Sports (
        ID INT IDENTITY PRIMARY KEY,
<<<<<<< Updated upstream
        Name NVARCHAR(50) NOT NULL,
    );
END 
||||||| Stash base
        Name NVARCHAR(50) NOT NULL);
    INSERT INTO Sports (ID, Name)
    VALUES
    (1, 'Soccer'),
    (2, 'Basketball'),
    (3, 'Volleyball'),
    (4, 'Baseball'),
    (5, 'Football');
END
=======
        Name NVARCHAR(50) NOT NULL);
    INSERT INTO Sports (Name)
    VALUES
    ('None'),
    ('Soccer'),
    ('Basketball'),
    ('Volleyball'),
    ('Baseball'),
    ('Football');
END
>>>>>>> Stashed changes
GO


IF OBJECT_ID('TeamLevel') IS NULL
BEGIN
    CREATE TABLE TeamLevel (
        ID INT PRIMARY KEY,
        level varchar(50) NOT NULL
    );
    INSERT INTO TeamLevel (ID, level)
    VALUES 
    (1, 'Junior'),
    (2, 'Recreational'),
    (3, 'Competitive');
END
GO

IF OBJECT_ID('TeamType') IS NULL 
BEGIN
    CREATE TABLE TeamType (
        ID INT PRIMARY KEY,
        type varchar(50) NOT NULL
    );
    INSERT INTO TeamType (ID, type)
    VALUES 
    (1, 'Masculine'),
    (2, 'Feminine'),
    (3, 'Mixed');
END 
GO

IF OBJECT_ID('Teams') IS NULL
BEGIN
    CREATE TABLE Teams (
        ID INT IDENTITY PRIMARY KEY,
        Name NVARCHAR(50) NOT NULL,
        LevelID INT NOT NULL,
        TypeID INT NOT NULL,
        SportID INT NOT NULL,
        FOREIGN KEY (LevelID) REFERENCES TeamLevel(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TypeID) REFERENCES TeamType(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (SportID) REFERENCES Sports(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('Players') IS NULL
BEGIN
    CREATE TABLE Players (
        ID INT IDENTITY PRIMARY KEY,
        UserID INT NOT NULL,
        TeamID INT NOT NULL,
        FOREIGN KEY (UserID) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamID) REFERENCES Teams(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

<<<<<<< Updated upstream

IF OBJECT_ID('Events') IS NULL 
||||||| Stash base
IF OBJECT_ID('TeamInEvent') IS NULL
BEGIN
    CREATE TABLE TeamInEvent (
        ID INT IDENTITY PRIMARY KEY,
        EventID INT NOT NULL,
        TeamID INT NOT NULL,
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamID) REFERENCES Teams(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO


IF OBJECT_ID('Events') IS NULL
=======

IF OBJECT_ID('Events') IS NULL
>>>>>>> Stashed changes
BEGIN
    CREATE TABLE Events (
        ID INT IDENTITY PRIMARY KEY,
        StartDate DATE NOT NULL,
        EndDate DATE NOT NULL
    );
END 
GO

IF OBJECT_ID('TeamInEvent') IS NULL
BEGIN
    CREATE TABLE TeamInEvent (
        ID INT IDENTITY PRIMARY KEY,
        EventID INT NOT NULL,
        TeamID INT NOT NULL,
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamID) REFERENCES Teams(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO
IF OBJECT_ID('Games') IS NULL
BEGIN
    CREATE TABLE Games (
        ID INT IDENTITY PRIMARY KEY,
        SportID INT NOT NULL,
        EventID INT NOT NULL,
        FirstTeamID INT NOT NULL,
        SecondTeamID INT NOT NULL,
        GameDate DATE NOT NULL,
        FinalScore NVARCHAR(50) NOT NULL,
        FOREIGN KEY (SportID) REFERENCES Sports(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (FirstTeamID) REFERENCES Teams(ID),
        FOREIGN KEY (SecondTeamID) REFERENCES Teams(ID)
    );
END;
GO

IF OBJECT_ID('Credentials') IS NULL
BEGIN
    CREATE TABLE Credentials (
        id INT PRIMARY KEY,
        username NVARCHAR(50) NOT NULL,
        password NVARCHAR(50) NOT NULL
        FOREIGN KEY (id) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END;
GO

<<<<<<< Updated upstream

||||||| Stash base

CREATE OR ALTER FUNCTION getPlayersWithId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT * FROM Players
WHERE
@identifiant = Players.UserID)
GO
=======
CREATE OR ALTER FUNCTION getPlayersByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID
FROM Players
WHERE
UserID = @identifiant)
GO
>>>>>>> Stashed changes

<<<<<<< Updated upstream
-- Déclencheur pour supprimer les informations de connexion lors de la suppression d'un utilisateur 
||||||| Stash base
CREATE OR ALTER FUNCTION getTeamsWithId(@identifiant INT)
RETURNS TABLE
AS
RETURN(
SELECT * from Teams
JOIN
(SELECT TeamID from dbo.getPlayersWithId(@identifiant)) as thePlayers
ON
thePlayers = Teams.ID)
GO 

CREATE OR ALTER FUNCTION getGamesWithId(@identifiant INT)
RETURNS TABLE
AS
RETURN(
SELECT * from Games
JOIN
(SELECT TeamID from dbo.getTeamsWithId(@identifiant)) as theTeams
ON
(theTeams = Games.FirstTeamID )OR theGames = Games.SecondTeamID)
GO 

CREATE OR ALTER FUNCTION getEventsWithId(@identifiant INT)
RETURNS TABLE
AS
RETURN(
SELECT * from Events
JOIN
(SELECT EventID from dbo.getGamesWithId(@identifiant)) as theGames
ON
(theGames = Events.ID ))
GO

-- Déclencheur pour supprimer les informations de connexion lors de la suppression d'un utilisateur
=======
CREATE OR ALTER FUNCTION getPlayersByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID
FROM Players
WHERE
TeamID = @identifiant)
GO
CREATE OR ALTER FUNCTION getEventsByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT TeamInEvent.ID AS EventInTeamID, TeamInEvent.EventID AS EventID, TeamInEvent.TeamID
FROM TeamInEvent
WHERE
TeamID = @identifiant)
GO

CREATE OR ALTER FUNCTION getEventsByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT TeamInEvent.ID AS EventInTeamID, TeamInEvent.EventID AS EventID, TeamInEvent.TeamID
FROM TeamInEvent
WHERE
EventID = @identifiant)
GO

CREATE OR ALTER FUNCTION getGamesByTeamId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Games.ID AS GameID, Games.SportID, Games.EventID, Games.FirstTeamID, Games.SecondTeamID, Games.GameDate, Games.FinalScore
FROM Games
WHERE
FirstTeamID = @identifiant OR SecondTeamID = @identifiant)
GO

CREATE OR ALTER FUNCTION getGamesByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Games.ID AS GameID, Games.SportID, Games.EventID, Games.FirstTeamID, Games.SecondTeamID, Games.GameDate, Games.FinalScore
FROM Games
WHERE
EventID = @identifiant)
GO

CREATE OR ALTER FUNCTION getGamesBySportId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Games.ID AS GameID, Games.SportID, Games.EventID, Games.FirstTeamID, Games.SecondTeamID, Games.GameDate, Games.FinalScore
FROM Games
WHERE
SportID = @identifiant)
GO

CREATE OR ALTER FUNCTION getPlayersBySportId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID, Teams.SportID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.SportID = @identifiant)
GO

CREATE OR ALTER FUNCTION getPlayersByLevelId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID, Teams.LevelID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.LevelID = @identifiant)
GO

CREATE OR ALTER FUNCTION getPlayersByTypeId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID, Teams.TypeID
FROM Players
JOIN Teams ON Players.TeamID = Teams.ID
WHERE
Teams.TypeID = @identifiant)
GO

CREATE OR ALTER FUNCTION getPlayersByEventId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID
FROM Players
JOIN TeamInEvent ON Players.TeamID = TeamInEvent.TeamID
WHERE
TeamInEvent.EventID = @identifiant)
GO

CREATE OR ALTER FUNCTION getPlayersByGameId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Players.ID AS PlayerID, Players.UserID, Players.TeamID
FROM Players
JOIN Games ON Players.TeamID = Games.FirstTeamID OR Players.TeamID = Games.SecondTeamID
WHERE
Games.ID = @identifiant)
GO

CREATE OR ALTER FUNCTION getTeamsByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Teams.ID AS TeamID, Teams.Name, Teams.LevelID, Teams.TypeID, Teams.SportID
FROM Teams
JOIN Players ON Teams.ID = Players.TeamID
WHERE
Players.UserID = @identifiant)
GO

CREATE OR ALTER FUNCTION getEventsByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Events.ID AS EventID, Events.StartDate, Events.EndDate
FROM Events
JOIN TeamInEvent ON Events.ID = TeamInEvent.EventID
JOIN Teams ON TeamInEvent.TeamID = Teams.ID
JOIN Players ON Teams.ID = Players.TeamID
WHERE
Players.UserID = @identifiant)
GO

CREATE OR ALTER FUNCTION getGamesByUserId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Games.ID AS GameID, Games.SportID, Games.EventID, Games.FirstTeamID, Games.SecondTeamID, Games.GameDate, Games.FinalScore
FROM Games
JOIN Teams ON Games.FirstTeamID = Teams.ID OR Games.SecondTeamID = Teams.ID
JOIN Players ON Teams.ID = Players.TeamID
WHERE
Players.UserID = @identifiant)
GO

-- Déclencheur pour supprimer les informations de connexion lors de la suppression d'un utilisateur
>>>>>>> Stashed changes
IF OBJECT_ID('trDeleteUserCredentials') IS NULL
BEGIN
    EXEC('
        CREATE TRIGGER trDeleteUserCredentials
        ON Users
        FOR DELETE
        AS
        BEGIN
            DELETE FROM Credentials
            WHERE id IN (SELECT id FROM DELETED);
        END;
    ');
END;
GO

-- Procédure stockée pour authentifier un utilisateur
IF OBJECT_ID('spAuthenticateUser') IS NULL
BEGIN
    EXEC('
        CREATE PROCEDURE spAuthenticateUser
            @Username NVARCHAR(50),
            @Password NVARCHAR(50)
        AS 
        BEGIN
            SELECT id
            FROM Credentials
            WHERE username = @Username AND password = @Password;
        END;
    ');
END;
GO

-- Procédure stockée pour enregistrer un utilisateur
IF OBJECT_ID('spRegisterUser') IS NULL
BEGIN
    EXEC('
        CREATE PROCEDURE spRegisterUser
            @Email NVARCHAR(50),
            @Address NVARCHAR(50),
            @FirstName NVARCHAR(50),
            @LastName NVARCHAR(50),
            @Username NVARCHAR(50),
            @Password NVARCHAR(50)
        AS 
        BEGIN
            INSERT INTO Users (Email, Address, FirstName, LastName)
            VALUES (@Email, @Address, @FirstName, @LastName);

            DECLARE @UserID INT;
            SET @UserID = SCOPE_IDENTITY();

            INSERT INTO Credentials (id, username, password)
            VALUES (@UserID, @Username, @Password);
        END;
    ');
END;
GO

-- Fonction pour récupérer les joueurs d'une équipe
IF OBJECT_ID('fnGetPlayers') IS NULL
BEGIN
    EXEC('
        CREATE FUNCTION fnGetPlayers (@TeamID INT) 
        RETURNS TABLE 
        AS 
        RETURN (
            SELECT u.ID, u.Email, u.Address, u.FirstName, u.LastName
            FROM Players p
            JOIN Users u ON p.UserID = u.ID
            WHERE p.TeamID = @TeamID
        );
    ');
END;
GO

<<<<<<< Updated upstream

||||||| Stash base
Insert INTO Users(ID, Email, Address, FirstName, LastName) VALUES (1000, 'ticoune@gmail.com', '123 rue Tabaga', 'Ticoune', 'Savard')
Insert INTO Teams(ID, Name, LevelID, TypeID, SportID) VALUES (1, 'LesZigotos', 1, 1, 1)
Insert INTO Players(ID, UserID, TeamID) VALUES(1, 1000, 1)
=======
-- Insert with a coherent data using the identity values
SET IDENTITY_INSERT Users ON; 
Insert INTO Users( ID, Email, Address, FirstName, LastName) VALUES (2, 'sheesh@gmail.com', '1234 rue de la rue', 'Sheesh', 'Sheesh');
SET IDENTITY_INSERT Users OFF; 
GO
SET IDENTITY_INSERT Teams ON;
Insert INTO Teams(ID, Name, LevelID, TypeID, SportID) VALUES (1, 'Les Tigres', 1, 1, 2); 
SET IDENTITY_INSERT Teams OFF; 
GO
SET IDENTITY_INSERT Players ON; 
Insert INTO Players(ID, UserID, TeamID) VALUES (1, 2, 1);
SET IDENTITY_INSERT Players OFF;
GO
>>>>>>> Stashed changes
