use crate::{Staff, Team, User};
use chrono::prelude::*;
use rand;
use random_string::generate;
use tiberius::{Client, Query};

use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;


pub async fn insert_fake_users(client: &mut Client<Compat<TcpStream>>) -> anyhow::Result<()> {
    for _ in 0..10 {
        let user = User::generate_fake_user();
        let (query, params) = user.to_insert_query();
        let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        client.execute(query, &params[..]).await?;

        println!("Successfully created a fake user");
    }

    println!("Successfully created 10 fake users");
    Ok(())
}

pub async fn insert_fake_teams(client: &mut Client<Compat<TcpStream>>) -> anyhow::Result<()> {
    for _ in 0..2 {
        let team = Team::generate_fake_team();
        let( query, params) = team.to_insert_query();
        let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        client.execute(query, &params[..]).await?;
        println!("Successfully created a fake team");
    }
    println!("Successfully created 2 fake teams");
    Ok(())
}

pub async fn insert_fake_staff(client: &mut Client<Compat<TcpStream>>) -> anyhow::Result<()> {
    for _i in 0..1 {
        let staff = Staff::generate_fake_staff();
        let (query, params) = staff.to_insert_query();
        let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        client.execute(query, &params[..]).await?;
        println!("Successfully created a fake staff");
        client.execute(query, &params[..]).await?;
        println!("Successfully created a fake staff");
    }
    println!("Successfully created 2 fake staff");
    Ok(())
}
