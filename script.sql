IF DB_ID('GANGGANG') IS NULL
BEGIN
    CREATE DATABASE GANGGANG;
END
GO

USE GANGGANG;
GO
CREATE TABLE users (
  User_ID INT PRIMARY KEY,
  Email NVARCHAR(50),
  Address NVARCHAR(50),
  First_Name NVARCHAR(50),
  Last_Name NVARCHAR(50),
);

CREATE TABLE Staff (
  User_ID INT PRIMARY KEY,
  NAS NVARCHAR(50),
  Hiring_Date DATE,
);

CREATE TABLE Team (
  Team_ID INT PRIMARY KEY,
  Team_Name NVARCHAR(50),
);

CREATE TABLE Game (
  Game_ID INT PRIMARY KEY,
  Sport_ID INT,
  Event_ID INT,
  Game_Date DATE,
  Final_Score INT,
);


CREATE TABLE Event (
  Event_ID INT PRIMARY KEY,
  Date_Start DATE,
  Date_End DATE,
);

CREATE TABLE Sport (
  Sport_ID INT PRIMARY KEY,
  Sport_Name NVARCHAR(50),
  Score_Format NVARCHAR(50),
);

CREATE TABLE Team_Composed_Of_Users (
  Team_ID INT,
  User_ID INT,
  Role NVARCHAR(50),
);

CREATE TABLE User_Plays_Sport (
  Sport_ID INT,
  User_ID INT,
);

CREATE TABLE Team_Participate_In_Game (
  Team_ID INT,
  Game_ID INT,
  Final_Score INT,
);

CREATE TABLE Staff_Works_For_Event (
  Event_ID INT,
  User_ID INT,
  Role NVARCHAR(50),
);

CREATE TABLE Staff_Works_For_Game (
  Game_ID INT,
  User_ID INT,
  Role NVARCHAR(50),
  Salary INT,
);

CREATE TABLE Sport_Intervenes_In_Event (
  Sport_ID INT,
  Event_ID INT,
);



ALTER TABLE Team_Composed_Of_Users
ADD CONSTRAINT FK_Team_Composed_Of_Users_User
FOREIGN KEY (User_ID) REFERENCES users(User_ID);
  
ALTER TABLE Team_Composed_Of_Users
ADD CONSTRAINT FK_Team_Composed_Of_Users_Team
FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID);

ALTER TABLE User_Plays_Sport
ADD CONSTRAINT FK_User_Plays_Sport_User
FOREIGN KEY (User_ID) REFERENCES users(User_ID);
  
ALTER TABLE User_Plays_Sport
ADD CONSTRAINT FK_User_Plays_Sport_Sport
FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID);

ALTER TABLE Team_Participate_In_Game
ADD CONSTRAINT FK_Team_Participate_In_Game_Team
FOREIGN KEY (Team_ID) REFERENCES Team(Team_ID);
  
ALTER TABLE Team_Participate_In_Game
ADD CONSTRAINT FK_Team_Participate_In_Game_Game
FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID);

ALTER TABLE Staff_Works_For_Event
ADD CONSTRAINT FK_Staff_Works_For_Event_User
FOREIGN KEY (User_ID) REFERENCES users(User_ID);

ALTER TABLE Staff_Works_For_Event
ADD CONSTRAINT FK_Staff_Works_For_Event_Event
FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID);
  
ALTER TABLE Staff_Works_For_Game
ADD CONSTRAINT FK_Staff_Works_For_Game_User
FOREIGN KEY (User_ID) REFERENCES users(User_ID);

ALTER TABLE Staff_Works_For_Game
ADD CONSTRAINT FK_Staff_Works_For_Game_Game
FOREIGN KEY (Game_ID) REFERENCES Game(Game_ID);

ALTER TABLE Sport_Intervenes_In_Event
ADD CONSTRAINT FK_Sport_Intervenes_In_Event_Sport
FOREIGN KEY (Sport_ID) REFERENCES Sport(Sport_ID);

ALTER TABLE Sport_Intervenes_In_Event
ADD CONSTRAINT FK_Sport_Intervenes_In_Event_Event
FOREIGN KEY (Event_ID) REFERENCES Event(Event_ID);



Insert into users (User_ID, Email, Address, First_Name, Last_Name)
values (1, 'issou@issou.com', '1234 rue de la rue', 'Issou', 'C');
Insert into users (User_ID, Email, Address, First_Name, Last_Name)
values (2, 'ganggang@icecream.yesyesyes', '1234 rue de la rue', 'Gang', 'Gang');




