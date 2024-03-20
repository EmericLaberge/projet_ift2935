IF DB_ID('GANGGANG') IS NULL
BEGIN
    CREATE DATABASE GANGGANG;
END

USE GANGGANG;


CREATE TABLE users (
  User_ID INT PRIMARY KEY,
  Email NVARCHAR(50) NOT NULL,
  Address NVARCHAR(50) NOT NULL,
  First_Name NVARCHAR(50) NOT NULL,
  Last_Name NVARCHAR(50) NOT NULL,
);

CREATE TABLE Staff (
  Staff_ID INT PRIMARY KEY,
  User_ID INT NOT NULL,
  NAS NVARCHAR(50) NOT NULL,
  Hiring_Date DATE NOT NULL,
  FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Team (
  Team_ID INT PRIMARY KEY,
  Team_Name NVARCHAR(50) NOT NULL,
);

CREATE TABLE Sport (
  Sport_ID INT PRIMARY KEY,
  Sport_Name NVARCHAR(50) NOT NULL,
  Score_Format NVARCHAR(50) NOT NULL,
);

CREATE TABLE Event (
  Event_ID INT PRIMARY KEY,
  Date_Start DATE NOT NULL,
  Date_End DATE NOT NULL,
);


CREATE TABLE Game (
  Game_ID INT PRIMARY KEY,
  Sport_ID INT NOT NULL,
  Event_ID INT NOT NULL,
  Game_Date DATE NOT NULL,
  Final_Score INT,
  FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID) ON DELETE CASCADE ON UPDATE CASCADE
);




CREATE TABLE Team_Composed_Of_Users (
  Team_ID INT NOT NULL,
  User_ID INT NOT NULL,
  Role NVARCHAR(50) NOT NULL,
  PRIMARY KEY (Team_ID, User_ID),
  FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE User_Plays_Sport (
  Sport_ID INT NOT NULL,
  User_ID INT NOT NULL,
  PRIMARY KEY (Sport_ID, User_ID),
  FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Team_Participate_In_Game (
  Team_ID INT NOT NULL,
  Game_ID INT NOT NULL,
  Final_Score INT,
  PRIMARY KEY (Team_ID, Game_ID),
  FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Staff_Works_For_Event (
  Event_ID INT NOT NULL,
  User_ID INT NOT NULL,
  Role NVARCHAR(50) NOT NULL,
  Salary INT NOT NULL,
  PRIMARY KEY (Event_ID, User_ID),
  FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Staff_Works_For_Game (
  Game_ID INT NOT NULL,
  User_ID INT NOT NULL,
  Role NVARCHAR(50) NOT NULL,
  Salary INT NOT NULL,
  PRIMARY KEY (Game_ID, User_ID),
  FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (User_ID) REFERENCES users(User_ID) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE Sport_Intervenes_In_Event (
  Sport_ID INT NOT NULL,
  Event_ID INT NOT NULL,
  PRIMARY KEY (Sport_ID, Event_ID),
  FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID) ON DELETE CASCADE ON UPDATE CASCADE
);


--
-- ALTER TABLE Team_Composed_Of_Users
-- ADD CONSTRAINT FK_Team_Composed_Of_Users_User
-- FOREIGN KEY (User_ID) REFERENCES users(User_ID);
--   
-- ALTER TABLE Team_Composed_Of_Users
-- ADD CONSTRAINT FK_Team_Composed_Of_Users_Team
-- FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID);
--
-- ALTER TABLE User_Plays_Sport
-- ADD CONSTRAINT FK_User_Plays_Sport_User
-- FOREIGN KEY (User_ID) REFERENCES users(User_ID);
--   
-- ALTER TABLE User_Plays_Sport
-- ADD CONSTRAINT FK_User_Plays_Sport_Sport
-- FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID);
--
-- ALTER TABLE Team_Participate_In_Game
-- ADD CONSTRAINT FK_Team_Participate_In_Game_Team
-- FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID);
--   
-- ALTER TABLE Team_Participate_In_Game
-- ADD CONSTRAINT FK_Team_Participate_In_Game_Game
-- FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID);
--
-- ALTER TABLE Staff_Works_For_Event
-- ADD CONSTRAINT FK_Staff_Works_For_Event_User
-- FOREIGN KEY (User_ID) REFERENCES users(User_ID);
--
-- ALTER TABLE Staff_Works_For_Event
-- ADD CONSTRAINT FK_Staff_Works_For_Event_Event
-- FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID);
--   
-- ALTER TABLE Staff_Works_For_Game
-- ADD CONSTRAINT FK_Staff_Works_For_Game_User
-- FOREIGN KEY (User_ID) REFERENCES users(User_ID);
--
-- ALTER TABLE Staff_Works_For_Game
-- ADD CONSTRAINT FK_Staff_Works_For_Game_Game
-- FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID);
--
-- ALTER TABLE Sport_Intervenes_In_Event
-- ADD CONSTRAINT FK_Sport_Intervenes_In_Event_Sport
-- FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID);
--
-- ALTER TABLE Sport_Intervenes_In_Event
-- ADD CONSTRAINT FK_Sport_Intervenes_In_Event_Event
-- FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID);
--
Insert into users (User_ID, Email, Address, First_Name, Last_Name)
values (1, 'issou@issou.com', '1234 rue de la rue', 'Issou', 'C');
Insert into users (User_ID, Email, Address, First_Name, Last_Name)
values (2, 'ganggang@icecream.yesyesyes', '1234 rue de la rue', 'Gang', 'Gang');




