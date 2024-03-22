
use tiberius::{AuthMethod, Config};
pub async fn get_config() -> Result<tiberius::Config, tiberius::error::Error> {
    let mut config = Config::new();
    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Rust4life"));
    // NOTE: Removed the encryption for now because tcp handshake is failing
    // and I dont know how to fix it
    config.encryption(tiberius::EncryptionLevel::NotSupported);
    Ok(config)
}
