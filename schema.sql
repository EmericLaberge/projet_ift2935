/*USE master
GO
DROP DATABASE Jasson
GO*/

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
        UserID INT PRIMARY KEY,
        NAS NVARCHAR(50) NOT NULL,
        HiringDate DATE NOT NULL,
        FOREIGN KEY (UserID) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('Sports') IS NULL
BEGIN
    CREATE TABLE Sports (
        Name NVARCHAR(50) PRIMARY KEY
    );
    INSERT INTO Sports (Name)
    VALUES
    ('None'),
    ('Soccer'),
    ('Basketball'),
    ('Volleyball'),
    ('Baseball'),
    ('Football');
END
GO


IF OBJECT_ID('TeamLevel') IS NULL
BEGIN
    CREATE TABLE TeamLevel (
        Level NVARCHAR(50) PRIMARY KEY
    );
    INSERT INTO TeamLevel (Level)
    VALUES
    ('None'),
    ('Junior'),
    ('Recreational'),
    ('Competitive');
END
GO

IF OBJECT_ID('TeamType') IS NULL
BEGIN
    CREATE TABLE TeamType (
        Type NVARCHAR(50) PRIMARY KEY
    );
    INSERT INTO TeamType (Type)
    VALUES
    ('None'),
    ('Masculine'),
    ('Feminine'),
    ('Mixed');
END
GO

IF OBJECT_ID('Teams') IS NULL
BEGIN
    CREATE TABLE Teams (
        ID INT IDENTITY PRIMARY KEY,
        Name NVARCHAR(50) NOT NULL,
        Level NVARCHAR(50) NOT NULL,
        TeamType NVARCHAR(50) NOT NULL,
        SportName NVARCHAR(50) NOT NULL,
        FOREIGN KEY (Level) REFERENCES TeamLevel(Level) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamType) REFERENCES TeamType(Type) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (SportName) REFERENCES Sports(Name) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('Players') IS NULL
BEGIN
    CREATE TABLE Players (
        UserID INT NOT NULL,
        TeamID INT NOT NULL,
        PRIMARY KEY(UserID, TeamID),
        FOREIGN KEY (UserID) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamID) REFERENCES Teams(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('Events') IS NULL
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
        EventID INT NOT NULL,
        TeamID INT NOT NULL,
        PRIMARY KEY(EventID, TeamID),
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (TeamID) REFERENCES Teams(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('StaffInEvent') IS NULL
BEGIN
    CREATE TABLE StaffInEvent (
        UserID INT,
        EventID INT,
        PRIMARY KEY (UserID, EventID),
        FOREIGN KEY (UserID) REFERENCES Users(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO

IF OBJECT_ID('Games') IS NULL
BEGIN
    CREATE TABLE Games (
        ID INT IDENTITY PRIMARY KEY,
        SportName NVARCHAR(50) NOT NULL,
        EventID INT NOT NULL,
        FirstTeamID INT NOT NULL,
        SecondTeamID INT NOT NULL,
        GameDate DATE NOT NULL,
        FinalScore NVARCHAR(50) NOT NULL,
        FOREIGN KEY (SportName) REFERENCES Sports(Name) ON DELETE CASCADE ON UPDATE CASCADE,
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
Teams.Level = @identifiant);
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
(SELECT Teams.ID AS TeamID, Teams.Name, Teams.Level, Teams.TeamType, Teams.SportName
FROM Teams
JOIN Players ON Teams.ID = Players.TeamID
WHERE
Players.UserID = @identifiant);
GO

CREATE OR ALTER FUNCTION getEventsByPlayerId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT Events.ID AS EventID, Events.StartDate, Events.EndDate
FROM Events
JOIN TeamInEvent ON Events.ID = TeamInEvent.EventID
JOIN Players ON TeamInEvent.TeamID = Players.TeamID
WHERE
Players.UserID = @identifiant);
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

--FONCTION COMPLEXE 1
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
)
GO

--FONCTION COMPLEXE 2
CREATE OR ALTER FUNCTION getPlayersByEventAndLevel(@EventID INT, @Level NVARCHAR(50))
RETURNS TABLE
AS
RETURN (
    SELECT UserID AS PlayerID, p.TeamID
    FROM (TeamInEvent te JOIN Teams t ON t.[level]= @Level) JOIN Players p ON te.TeamID = p.TeamID 
);
GO

-- Déclencheur pour supprimer les informations de connexion lors de la suppression d'un utilisateur
CREATE OR ALTER TRIGGER trDeleteUserCredentials
ON Users
FOR DELETE
AS
BEGIN
    DELETE FROM Credentials
    WHERE id IN (SELECT id FROM DELETED);
END;
GO

-- Procédure stockée pour authentifier un utilisateur
CREATE OR ALTER PROCEDURE spAuthenticateUser
    @Username NVARCHAR(50),
    @Password NVARCHAR(50)
AS
BEGIN
    SELECT id
    FROM Credentials
    WHERE username = @Username AND password = @Password;
END;
GO


-- Procédure stockée pour enregistrer un utilisateur
CREATE OR ALTER PROCEDURE spRegisterUser
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

-- Insert with a coherent data using the identity values
IF (SELECT COUNT(*) FROM Users)=0 
    Insert INTO Users(Email, Address, FirstName, LastName) VALUES ('sheesh@gmail.com', '1234 rue de la rue', 'Sheesh', 'Sheesh')
GO
IF (SELECT COUNT(*) FROM Teams)=0
    Insert INTO Teams(Name, Level, TeamType, SportName) VALUES ('Les Tigres', 'Junior', 'Mixed', 'Soccer'); 
GO
IF (SELECT COUNT(*) FROM Players)=0
    Insert INTO Players(UserID, TeamID) VALUES (1, 1);
GO
