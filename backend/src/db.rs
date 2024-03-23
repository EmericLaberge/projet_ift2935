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
