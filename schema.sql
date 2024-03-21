IF NOT EXISTS (SELECT * FROM sys.databases WHERE name = 'MyDatabase')
BEGIN
    CREATE DATABASE MyDatabase;
END
GO

USE MyDatabase;
GO

IF OBJECT_ID('Users') IS NULL
BEGIN
    CREATE TABLE Users (
        ID INT IDENTITY PRIMARY KEY,
        Email NVARCHAR(50) NOT NULL,
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
  
IF OBJECT_ID('Teams') IS NULL
BEGIN
    CREATE TABLE Teams (
        ID INT IDENTITY PRIMARY KEY,
        Name NVARCHAR(50) NOT NULL
    );
END
GO

IF OBJECT_ID('Sports') IS NULL 
BEGIN
    CREATE TABLE Sports (
        ID INT IDENTITY PRIMARY KEY,
        Name NVARCHAR(50) NOT NULL,
        ScoreFormat NVARCHAR(50) NOT NULL
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
        GameDate DATE NOT NULL,
        FinalScore INT,
        FOREIGN KEY (SportID) REFERENCES Sports(ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (EventID) REFERENCES Events(ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END 
GO
-- DECLARE @ConstraintName nvarchar(256), @TableName nvarchar(256), @Sql nvarchar(1000);
--
-- -- Loop through the foreign key constraints that reference the 'Sport' table
-- DECLARE fk_cursor CURSOR FOR 
--     SELECT fk.name, OBJECT_NAME(fk.parent_object_id)
--     FROM sys.foreign_keys AS fk
--     INNER JOIN sys.foreign_key_columns AS fkc ON fk.object_id = fkc.constraint_object_id
--     INNER JOIN sys.tables AS t ON fk.referenced_object_id = t.object_id
--     WHERE t.name = 'Sport';
--
-- OPEN fk_cursor;
-- FETCH NEXT FROM fk_cursor INTO @ConstraintName, @TableName;
--
-- WHILE @@FETCH_STATUS = 0
-- BEGIN
--     SET @Sql = 'ALTER TABLE ' + QUOTENAME(@TableName) + ' DROP CONSTRAINT ' + QUOTENAME(@ConstraintName);
--     EXEC sp_executesql @Sql;
--     FETCH NEXT FROM fk_cursor INTO @ConstraintName, @TableName;
-- END
--
-- CLOSE fk_cursor;
-- DEALLOCATE fk_cursor;
-- SELECT 
--     fk.name AS ForeignKey,
--     OBJECT_NAME(fk.parent_object_id) AS TableName,
--     OBJECT_NAME(fk.referenced_object_id) AS ReferencedTable
-- FROM 
--     sys.foreign_keys AS fk
-- WHERE 
--     OBJECT_NAME(fk.referenced_object_id) = 'Sport' -- Or replace 'Sport' with 'Event' or 'users' as needed
-- DROP TABLE IF EXISTS Sport; -- Repeat for 'Event' and 'users' as necessary
--

 IF OBJECT_ID('users') IS NOT NULL 
 BEGIN
   SELECT * FROM users;
 END

 IF OBJECT_ID('Staff') IS NOT NULL 
 BEGIN
   SELECT * FROM Staff;
 END

 IF OBJECT_ID('Team') IS NOT NULL 
 BEGIN
   SELECT * FROM Team;
 END 

 IF OBJECT_ID('Sport') IS NOT NULL 
 BEGIN
   SELECT * FROM Sport;
 END 

 IF OBJECT_ID('Event') IS NOT NULL
 BEGIN
   SELECT * FROM Event;
 END

 IF OBJECT_ID('Game') IS NOT NULL 
 BEGIN
   SELECT * FROM Game;
 END
