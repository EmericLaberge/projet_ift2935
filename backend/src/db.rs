use tokio;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use tokio_util::compat::Compat;
use tokio::io::AsyncWriteExt;






pub async fn connect_to_db(config: Config) -> Result<Client<Compat<TcpStream>>, tiberius::error::Error> {
    // Establish the tcp connection
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // Connect to the MSSQL server
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("Successfully connected to the server");

    // Read the SQL file
    client.execute("USE MyDatabase", &[]).await?;

    // return the client 
    Ok(client)
    
}
