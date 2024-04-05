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
        Name NVARCHAR(50) NOT NULL,
    );
END
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
BEGIN
    CREATE TABLE Events (
        ID INT IDENTITY PRIMARY KEY,
        StartDate DATE NOT NULL,
        EndDate DATE NOT NULL
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


CREATE OR ALTER FUNCTION getPlayersWithId(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT * FROM Players
WHERE
@identifiant = Players.UserID)
GO;

CREATE OR ALTER FUNCTION getTeamsOfUser(@identifiant INT)
RETURNS TABLE
AS
RETURN
(SELECT * FROM Teams
Players.UserID = (
dbo.getPlayersWithId(@identifint).ID
)
GO;

-- Déclencheur pour supprimer les informations de connexion lors de la suppression d'un utilisateur
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
