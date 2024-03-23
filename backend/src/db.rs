use crate::models::{User, Staff, Game, Team, Sport, Event};
use std::sync::Arc;
use tiberius::ToSql;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::{compat::Compat, compat::TokioAsyncWriteCompatExt};

/// Connect to the MSSQL server
/// # Arguments
/// * `config` - The configuration for the connection (type : *tiberius::Config*)
/// # Returns
/// * A Result containing the client if the connection was successful
/// * An error if the connection was unsuccessful
pub async fn connect_to_db(
    config: Config,
) -> Result<Client<Compat<TcpStream>>, tiberius::error::Error> {
    // Establish the tcp connection
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Connect to the MSSQL server
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("Successfully connected to the server");

    // Set the database to use (MyDatabase)
    client.execute("USE MyDatabase", &[]).await?;

    // return the client
    Ok(client)
}

async fn create_user(
    client: &mut Client<Compat<TcpStream>>,
    user: User,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = user.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}


async fn create_staff(
    client: &mut Client<Compat<TcpStream>>,
    staff: Staff,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = staff.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}
async fn create_team(
    client: &mut Client<Compat<TcpStream>>,
    team: Team,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = team.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}

async fn create_sport(
    client: &mut Client<Compat<TcpStream>>,
    sport: Sport,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = sport.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}

async fn create_event(
    client: &mut Client<Compat<TcpStream>>,
    event: Event,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = event.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}

async fn create_game(
    client: &mut Client<Compat<TcpStream>>,
    game: Game,
) -> Result<(), tiberius::error::Error> {
    let (query, params) = game.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    client.execute(query, &params[..]).await?;
    Ok(())
}



