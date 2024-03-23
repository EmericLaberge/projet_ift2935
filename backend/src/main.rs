mod config;
mod db;
mod models;
mod test;
use crate::models::{staff, team, users};

use crate::test::{insert_fake_staff, insert_fake_teams, insert_fake_users};
use config::get_config;
use db::connect_to_db;
use staff::Staff;
use std::fs;
use team::Team;
use users::User;

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
