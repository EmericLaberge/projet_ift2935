
IF OBJECT_ID('users') IS NULL
BEGIN
    CREATE TABLE users (
        User_ID INT IDENTITY PRIMARY KEY,
        Email NVARCHAR(50) NOT NULL,
        Address NVARCHAR(50) NOT NULL,
        First_Name NVARCHAR(50) NOT NULL,
        Last_Name NVARCHAR(50) NOT NULL
    );
END
GO

IF OBJECT_ID('Staff') IS NULL
BEGIN
    CREATE TABLE Staff (
        Staff_ID INT IDENTITY PRIMARY KEY,
        User_ID INT NOT NULL,
        NAS NVARCHAR(50) NOT NULL,
        Hiring_Date DATE NOT NULL,
        FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END
GO
  
IF OBJECT_ID('Team') IS NULL
BEGIN
    CREATE TABLE Team (
        Team_ID INT IDENTITY PRIMARY KEY,
        Team_Name NVARCHAR(50) NOT NULL
    );
END

IF OBJECT_ID('Sport') IS NULL 
BEGIN
    CREATE TABLE Sport (
        Sport_ID INT IDENTITY PRIMARY KEY,
        Sport_Name NVARCHAR(50) NOT NULL,
        Score_Format NVARCHAR(50) NOT NULL
    );
END 

IF OBJECT_ID('Event') IS NULL 
BEGIN
    CREATE TABLE Event (
        Event_ID INT IDENTITY PRIMARY KEY,
        Date_Start DATE NOT NULL,
        Date_End DATE NOT NULL
    );
END 

IF OBJECT_ID('Game') IS NULL
BEGIN
    CREATE TABLE Game (
        Game_ID INT IDENTITY PRIMARY KEY,
        Sport_ID INT NOT NULL,
        Event_ID INT NOT NULL,
        Game_Date DATE NOT NULL,
        Final_Score INT,
        FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID) ON DELETE CASCADE ON UPDATE CASCADE,
        FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID) ON DELETE CASCADE ON UPDATE CASCADE
    );
END 
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
