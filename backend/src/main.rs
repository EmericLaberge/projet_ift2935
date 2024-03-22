mod event;
mod game;
mod sport;
mod staff;
mod team;
mod test;
mod users;
mod db;
mod config;

use staff::Staff;
use crate::test::{insert_fake_teams, insert_fake_staff, insert_fake_users};
use std::fs;
use team::Team;
use users::User;
use db::connect_to_db;
use config::get_config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Try to connect to the database using db module (see db.rs)
    let config: tiberius::Config = get_config().await?;
    let mut client = connect_to_db(config).await?;
    println!("Successfully read the file");
    insert_fake_users(&mut client).await?;
    insert_fake_staff(&mut client).await?;
    insert_fake_teams(&mut client).await?;
    Ok(())
}
