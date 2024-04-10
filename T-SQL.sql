Use Jasson;
GO



--Déclencheur pour signaler une erreur de formattage dans le résultat de la partie
CREATE OR ALTER TRIGGER trScoreFormatCheck
ON Games
AFTER UPDATE, INSERT
AS
    IF (COLUMNS_UPDATED() & 64)>0 AND EXISTS (SELECT inserted.FinalScore FROM inserted WHERE FinalScore NOT LIKE '%-%' AND FinalScore NOT LIKE 'N-A' AND FinalScore NOT LIKE '')
        BEGIN
            RAISERROR ('A formatting problem occured. Operation has been rolled back',16,1)
            ROLLBACK TRANSACTION
        END;
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

--Procédure pour créer une équipe
CREATE OR ALTER PROCEDURE spCreateTeam
    @Name NVARCHAR(50),
    @TeamLevel NVARCHAR(50),
    @TeamType NVARCHAR(50),
    @SportName NVARCHAR(50)
AS
BEGIN
    INSERT INTO Teams (Name, TeamLevel, TeamType, SportName)
    VALUES (@Name, @TeamLevel, @TeamType, @SportName);
END;
GO


--Procédure pour ajouter le score final à une partie
CREATE OR ALTER PROCEDURE addFinalScore(@GameID INT, @scoreTeam1 INT, @scoreTeam2 INT)
AS
BEGIN
    UPDATE Games
    SET FinalScore = FORMAT(CAST(
            CONCAT(
                FORMAT(@scoreTeam1,'000'),FORMAT(@scoreTeam2, '##0')) AS integer
        ),
        (SELECT ScoreFormat
        FROM Sports s
        JOIN Games g
        ON @GameID=g.ID AND g.SportName = s.Name))

END;
GO

--Procédure pour créer un curseur de manière dynamique
CREATE OR ALTER PROCEDURE makeCursor(@topNum INT, @tableName VARCHAR(50))
AS
BEGIN
    DECLARE @query AS NVARCHAR(500)
    SET @query = 'DECLARE '+@tableName+'Cursor CURSOR GLOBAL SCROLL
    FOR SELECT TOP ('+@topNum+') *
    FROM '+@tableName+'
    FOR READ_ONLY'

    EXEC(@query)
END;
GO

--Procédure pour supprimer une entrée d'une table de manière dynamique
--NOTE: Elle ne fonctionne présentement que sur les tables avec des clefs primaires simples.
CREATE OR ALTER PROCEDURE deleteEntryByPK(@tableName VARCHAR(50), @PK NVARCHAR(50))
AS
BEGIN
    DECLARE @type AS NVARCHAR(64)
    SET @type = CASE
                    WHEN @tableName = 'Games'
                        OR @tableName = 'Events'
                        OR @tableName = 'Users'
                        OR @tableName = 'Staff'
                        OR @tableName = 'Teams'
                        OR @tableName = 'Credentials'
                        THEN 'INT'
                    WHEN @tableName = 'Sports'
                        OR @tableName = 'TeamLevel'
                        OR @tableName = 'TeamType'
                        THEN 'NVARCHAR(50)'
    END
    DECLARE @query AS NVARCHAR(500), @field AS NVARCHAR(100)

    set @field = (SELECT top 1 COLUMN_NAME
    FROM INFORMATION_SCHEMA.Columns
    WHERE TABLE_NAME = @tableName)

    SET @query = 'DELETE FROM '+@tableName+' WHERE '+@field+'= CAST('+@PK+') AS '+@type
    EXEC(@query)
END;
GO