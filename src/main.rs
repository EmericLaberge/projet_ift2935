use tiberius::{AuthMethod, Client, Config, Query};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
use std::fs;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::new();

    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Rust4life"));
    config.encryption(tiberius::EncryptionLevel::NotSupported);

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    // To be able to use Tokio's tcp, we're using the `compat_write` from
    // the `TokioAsyncWriteCompatExt` to get a stream compatible with the
    // traits from the `futures` crate.

    let mut client = Client::connect(config, tcp.compat_write()).await?;
    println!("Successfully connected to the server");
    let sql_file : String = fs::read_to_string("script.sql")?;
    println!("Successfully read the file");

    let mut selectTest = Query::new("INSERT INTO users (User_ID, Email,Address,First_Name,Last_Name) VALUES (5, 'Rust4life', 'Rust4life', 'Rust4life', 'Rust4life')");

    let result = selectTest.query(&mut client).await?;


    println!("Successfully created a new user");

    Ok(())
}

