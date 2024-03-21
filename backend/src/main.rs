mod event;
mod game;
mod sport;
mod staff;
mod team;
mod test;
mod users;

use staff::Staff;
use crate::test::{insert_fake_teams, insert_fake_staff, insert_fake_users};
use std::fs;
use team::Team;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use users::User;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // configure the connection
    let mut config = Config::new();
    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Rust4life"));
    // NOTE: Removed the encryption for now because tcp handshake is failing
    // and I dont know how to fix it
    config.encryption(tiberius::EncryptionLevel::NotSupported);

    // Establish the tcp connection
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Connect to the MSSQL server
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("Successfully connected to the server");

    // Read the SQL file
    let _sql_file = fs::read_to_string("../schema.sql")?;
    println!("Successfully read the file");
    insert_fake_users(&mut client).await?;
    insert_fake_staff(&mut client).await?;
    insert_fake_teams(&mut client).await?;
    Ok(())
}
