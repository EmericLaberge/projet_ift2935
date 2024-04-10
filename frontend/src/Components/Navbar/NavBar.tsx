// NavBar.tsx
import React from "react";
import { NavLink, useLocation } from "react-router-dom";

import "./NavBar.css";


const NavBar: React.FC = () => {
  const location = useLocation();

  const getLinkStyle = (path: string) => {
    return location.pathname === path ? { color: "#ff9900" } : { color: "#ffffff" };
  };


  return (
    <div className={"navbar"}>

      <ul id="menu">
        <li id="home">
          <NavLink to="/" style={getLinkStyle("/")}>
            Home
          </NavLink>
        </li>
        <li id="users">
          <NavLink to="/users" style={getLinkStyle("/users")}>
            Users
          </NavLink>
        </li>
        <li id="Teams">
          <NavLink to="/teams" style={getLinkStyle("/teams")}>
            Teams
          </NavLink>
        </li>

        <li id="Events">
          <NavLink to="/events" style={getLinkStyle("/events")}>
            Events
          </NavLink>
          </li>
          <li id="Games">
          <NavLink to="/games" style={getLinkStyle("/games")}>
            Games 
          </NavLink>
        </li>


      </ul>
    </div>
  );
};
export default NavBar;
