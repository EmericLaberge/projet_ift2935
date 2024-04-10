USE master;
GO
alter database [Jasson] set single_user with rollback immediate
IF EXISTS (SELECT * FROM sys.databases WHERE name = 'Jasson')
BEGIN
    DROP DATABASE Jasson;
END
GO

CREATE DATABASE Jasson;
GO
use Jasson;
go

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
        Name NVARCHAR(50) PRIMARY KEY,
        minJoueurs INT NOT NULL,
        maxJoueurs INT NOT NULL,
        scoreFormat NVARCHAR(50) NOT NULL,
        CHECK (scoreFormat LIKE '%[0]-[0]%')
    );
    INSERT INTO Sports
    VALUES
    ('None',0,0,'##0-0##'),
    ('Soccer',12,20,'##0-0##'),
    ('Basketball',12,20,'##0-0##'),
    ('Volleyball',12, 12, '##0-0##'),
    ('Baseball', 14, 20, '##0-0##'),
    ('Football', 16, 20, '##0-0##');
END
GO


IF OBJECT_ID('TeamLevel') IS NULL
BEGIN
    CREATE TABLE TeamLevel (
        TeamLevel NVARCHAR(50) PRIMARY KEY
    );
    INSERT INTO TeamLevel (TeamLevel)
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
        TeamLevel NVARCHAR(50) NOT NULL,
        TeamType NVARCHAR(50) NOT NULL,
        SportName NVARCHAR(50) NOT NULL,
        FOREIGN KEY (TeamLevel) REFERENCES TeamLevel(TeamLevel) ON DELETE CASCADE ON UPDATE CASCADE,
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
        Name NVARCHAR(64) NOT NULL,
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
        FinalScore NVARCHAR(50),
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

-- Insert with a coherent data using the identity values
IF (SELECT COUNT(*) FROM Users)=0
    Insert INTO Users(Email, Address, FirstName, LastName) VALUES
    ('sheesh@gmail.com', '1234 rue de la rue', 'Sheesh', 'Sheesh'),
    ('FelipeAlou@gmail.com', '123 Rue Saint-Jacques', 'Felipe', 'Alou'),
    ('RustyStaub@Hotmail.com', '456 Rue Sainte-Catherine', 'Rusty', 'Staub'),
    ('GaryCarter@yahoo.ca', '789 Avenue du Parc', 'Gary', 'Carter'),
    ('TimRaines@gmail.com', '132 Avenue des Pins', 'Tim', 'Raines'),
    ('AndreDawson@Hotmail.com', '567 Rue Sherbrooke', 'Andre', 'Dawson'),
    ('TimWallach@yahoo.ca', '910 Boulevard Saint-Laurent', 'Tim', 'Wallach'),
    ('DennisMartinez@gmail.com', '1112 Rue Saint-Denis', 'Dennis', 'Martinez'),
    ('PascualPerez@Hotmail.com', '1314 Avenue McGill College', 'Pascual', 'Perez'),
    ('BillLee@yahoo.ca', '1516 Rue Peel', 'Bill', 'Lee'),
    ('SteveRogers@gmail.com', '1718 Rue de la Montagne', 'Steve', 'Rogers'),
    ('JohnDooley@Hotmail.com', '1920 Rue de Bleury', 'John', 'Dooley'),
    ('WarrenCromartie@yahoo.ca', '2122 Avenue du Docteur-Penfield', 'Warren', 'Cromartie'),
    ('ElliottMaddox@gmail.com', '2324 Rue Guy', 'Elliott', 'Maddox'),
    ('LarryParish@Hotmail.com', '2526 Rue de la Gauchetiere', 'Larry', 'Parish'),
    ('RowlandOffice@yahoo.ca', '2728 Rue Crescent', 'Rowland', 'Office'),
    ('PedroMartinez@gmail.com', '2930 Avenue Atwater', 'Pedro', 'Martinez'),
    ('KenSingleton@Hotmail.com', '3132 Rue Sainte-Famille', 'Ken', 'Singleton'),
    ('DarrinFletcher@yahoo.ca', '3334 Rue de Bullion', 'Darrin', 'Fletcher'),
    ('MarquisGrissom@gmail.com', '3536 Rue de la Commune', 'Marquis', 'Grissom'),
    ('MoisesAlou@Hotmail.com', '3738 Rue Notre-Dame', 'Moises', 'Alou'),
    ('MelRojas@yahoo.ca', '3940 Rue Saint-Paul', 'Mel', 'Rojas'),
    ('UguethUrbina@gmail.com', '4142 Rue Saint-Antoine', 'Ugueth', 'Urbina'),
    ('VladimirGuerrero@Hotmail.com', '4344 Rue de la Gauchetiere', 'Vladimir', 'Guerrero'),
    ('OrlandoCabrera@yahoo.ca', '4546 Rue de Bleury', 'Orlando', 'Cabrera'),
    ('JavierVazquez@gmail.com', '4748 Avenue du Parc', 'Javier', 'Vazquez'),
    ('BartoloColon@Hotmail.com', '4950 Rue Saint-Urbain', 'Bartolo', 'Colon'),


    ('SarahSmith@gmail.com', '123 Main St', 'Sarah', 'Smith'),
    ('EmmaJohnson@gmail.com', '456 Elm St', 'Emma', 'Johnson'),
    ('AvaWilliams@gmail.com', '789 Pine St', 'Ava', 'Williams'),
    ('OliviaBrown@gmail.com', '321 Oak St', 'Olivia', 'Brown'),
    ('SophiaDavis@gmail.com', '654 Maple St', 'Sophia', 'Davis'),
    ('IsabellaMiller@gmail.com', '987 Birch St', 'Isabella', 'Miller'),
    ('MiaWilson@gmail.com', '135 Cedar St', 'Mia', 'Wilson'),
    ('CharlotteMoore@gmail.com', '246 Cherry St', 'Charlotte', 'Moore'),
    ('AmeliaTaylor@gmail.com', '357 Walnut St', 'Amelia', 'Taylor'),
    ('HarperAnderson@gmail.com', '468 Hickory St', 'Harper', 'Anderson'),
    ('EvelynThomas@gmail.com', '579 Magnolia St', 'Evelyn', 'Thomas'),
    ('AbigailWhite@gmail.com', '690 Spruce St', 'Abigail', 'White'),
    ('EmilyHarris@gmail.com', '701 Willow St', 'Emily', 'Harris'),
    ('HannahClark@gmail.com', '812 Chestnut St', 'Hannah', 'Clark'),
    ('MadisonLewis@gmail.com', '923 Ash St', 'Madison', 'Lewis'),
    ('AveryMartin@gmail.com', '1034 Beech St', 'Avery', 'Martin'),
    ('SophiaGarcia@gmail.com', '1145 Dogwood St', 'Sophia', 'Garcia'),
    ('IsabellaRodriguez@gmail.com', '1256 Elmwood St', 'Isabella', 'Rodriguez'),
    ('MiaMartinez@gmail.com', '1367 Fern St', 'Mia', 'Martinez'),
    ('EmilyHernandez@gmail.com', '1478 Hazel St', 'Emily', 'Hernandez'),
    ('HannahLopez@gmail.com', '1589 Juniper St', 'Hannah', 'Lopez'),
    ('MadisonGonzalez@gmail.com', '1601 Locust St', 'Madison', 'Gonzalez'),
    ('AveryWright@gmail.com', '1712 Maplewood St', 'Avery', 'Wright'),
    ('SophiaScott@gmail.com', '1823 Oakwood St', 'Sophia', 'Scott'),
    ('IsabellaJones@gmail.com', '1934 Pinewood St', 'Isabella', 'Jones'),
    ('MiaTorres@gmail.com', '2045 Redwood St', 'Mia', 'Torres'),
    ('EmilyRamirez@gmail.com', '2156 Sprucewood St', 'Emily', 'Ramirez'),
    ('HannahReyes@gmail.com', '2267 Willowwood St', 'Hannah', 'Reyes'),
    ('MadisonFlores@gmail.com', '2378 Birchwood St', 'Madison', 'Flores');


GO
IF (SELECT COUNT(*) FROM Teams)=0
    Insert INTO Teams(Name, TeamLevel, TeamType, SportName) VALUES
    ('Les Tigres', 'Junior', 'Mixed', 'Soccer'),
    ('The Sluggers', 'Competitive', 'Masculine', 'Baseball'),
    ('Les Hard Hitters', 'Competitive', 'Masculine', 'Baseball'),
    ('The Kicks', 'Junior', 'Mixed', 'Soccer'),
    ('The Beach Girls', 'Recreational', 'Feminine', 'Volleyball'),
    ('VolleyGirls', 'Recreational', 'Feminine', 'Volleyball'),
    ('The Blundettos', 'Competitive', 'Mixed', 'Baseball'),
    ('Les instincteurs', 'Competitive', 'Mixed', 'Baseball'),
    ('The Thunder Bears', 'Competitive', 'Mixed', 'Baseball'),
    ('The Swamp Cats', 'Competitive', 'Mixed', 'Baseball');
GO


IF (SELECT COUNT(*) FROM Players)=0
    Insert INTO Players(UserID, TeamID) VALUES
    -- Team de Baseball
    (1, 1),
    (2, 2),  (3, 3),
    (4, 2),  (5, 3),
    (6, 2),  (7, 3),
    (8, 2),  (9, 3),
    (10, 2), (11, 3),
    (12, 2), (13, 3),
    (14, 2), (15, 3),
    (16, 2), (17, 3),
    (18, 2), (19, 3),
    (20, 2), (21, 3),
    (22, 2), (23, 3),
    (24, 2), (25, 3),
    (26, 2), (27, 3),

    --Teams de baseBall mixte
    (20, 7),  (21, 8),
    (22, 7),  (23, 8),
    (24, 7),  (25, 8),
    (26, 7),  (27, 8),
    (28, 7),  (29, 8),
    (30, 7),  (31, 8),
    (32, 7),  (33, 8),
    (34, 7),  (35, 8),
    (36, 7),  (37, 8),
    (38, 7),  (39, 8),
    (40, 7),  (41, 8),
    (42, 7),  (43, 8),
    (44, 7),  (45, 8),
    (46, 7),  (47, 8),
    (48, 7),  (49, 8),
    (50, 7),  (51, 8),

    (2, 9),   (3, 10),
    (4, 9),   (5, 10),
    (6, 9),   (7, 10),
    (8, 9),   (9, 10),
    (10, 9),  (11, 10),

    --Team de Soccer
    (2, 1),  (3, 4),
    (4, 1),  (5, 4),
    (6, 1),  (7, 4),
    (8, 1),  (9, 4),
    (10, 1), (11, 4),
    (12, 1), (13, 4),
    (14, 1), (15, 4),
    (16, 1), (17, 4),
    (18, 1), (19, 4),
    (20, 1), (21, 4),
    (22, 1), (23, 4),
    (24, 1), (25, 4),
    (26, 1), (27, 4),

    --team de VolleyBall
    (28, 5),  (29, 6),
    (30, 5),  (31, 6),
    (32, 5),  (33, 6),
    (34, 5),  (35, 6),
    (36, 5),  (37, 6),
    (38, 5),  (39, 6),
    (40, 5),  (41, 6),
    (42, 5),  (43, 6),
    (44, 5),  (45, 6),
    (46, 5),  (47, 6),
    (48, 5),  (49, 6),
    (50, 5),  (51, 6),
    (52, 5),  (53, 6),
    (54, 5),  (55, 6);
GO
IF (SELECT COUNT(*) FROM Events)=0
    INSERT INTO Events(Name, StartDate, EndDate) VALUES
    ('Tournois BaseBall Mile End','2024-08-01', '2024-08-02'),
    ('Soccer Mixte Valleyfield', '2024-06-02', '2024-06-04'),
    ('Beach Volleyball feminin', '2024-07-20', '2024-07-22'),
    ('Quebec City Baseball Classic', '2024-07-01', '2024-07-04'),
    ('Gatineau Volleyball Tournament', '2024-05-17', '2024-05-19'),
    ('Montreal Football Championship', '2024-11-22', '2024-11-24'),
    ('Sherbrooke Soccer Cup', '2024-08-15', '2024-08-18'),
    ('Trois-Rivières Baseball Open', '2024-06-07', '2024-06-09'),
    ('Laval Volleyball Challenge', '2024-03-15', '2024-03-17');
GO
IF (SELECT COUNT(*) FROM TeamInEvent)=0
    INSERT INTO TeamInEvent(EventID, TeamID) VALUES
    (1, 2),
    (1, 3),
    (2, 1),
    (2, 4),
    (3, 5),
    (3, 6),
    (1, 7),
    (1, 8),
    (1, 9),
    (1, 10);
GO
IF (SELECT COUNT(*) FROM StaffInEvent)=0
    INSERT INTO StaffInEvent(UserID, EventID) VALUES(1, 1)
GO
IF (SELECT COUNT(*) FROM Games)=0
    INSERT INTO Games(SportName, EventID, FirstTeamID, SecondTeamID, GameDate)
    VALUES ('Baseball', 1, 2, 3, '2024-08-01'),
           ('Soccer', 2, 1, 4, '2024-06-03'),
           ('Volleyball', 3, 5, 6, '2024-07-21'),
           ('Baseball', 1, 7, 8, '2024-08-01'),
           ('Baseball', 1, 2, 8, '2024-08-02'),
           ('Baseball', 1, 7, 10, '2024-08-01'),
           ('Baseball', 1, 9, 10, '2024-08-01'),
           ('Baseball', 1, 9, 8, '2024-08-02');
GO
