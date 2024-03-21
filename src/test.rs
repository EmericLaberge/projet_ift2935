use crate::{Staff, Team, User};
use chrono::prelude::*;
use rand;
use random_string::generate;
use tiberius::{Client, Query};

use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub async fn insert_fake_users_with_compatibility(
    client: &mut Client<Compat<TcpStream>>,
) -> anyhow::Result<()> {
    for _i in 0..10 {
        let user = User::generate_fake_user();
        let insert_user_query = user.to_insert_statement();
        client.execute(insert_user_query.as_str(), &[]).await?;
        println!(
            "Successfully executed the query: {}",
            user.to_insert_statement()
        );
    }
    println!("Successfully created 10 fake users");
    Ok(())
}

pub async fn insert_fake_teams_with_compatibility(
    client: &mut Client<Compat<TcpStream>>,
) -> anyhow::Result<()> {
    for _i in 0..2 {
        let team = Team::generate_fake_team();
        let insert_team_query = team.to_insert_statement();
        let query = Query::new(insert_team_query);
        query.query(client).await?;
        println!(
            "Successfully executed the query: {}",
            team.to_insert_statement()
        );
    }
    println!("Successfully created 2 fake teams");
    Ok(())
}

pub async fn insert_fake_staff_with_compatibility(
    client: &mut Client<Compat<TcpStream>>,
) -> anyhow::Result<()> {
    for _i in 0..1 {
        
        let staff = Staff::generate_fake_staff();
        let insert_staff_query = staff.to_insert_statement();
        client.execute(insert_staff_query.as_str(), &[]).await?;
        println!(
            "Successfully executed the query: {}",
            staff.to_insert_statement()
        );
    }
    println!("Successfully inserted fake staff");
    Ok(())
}
