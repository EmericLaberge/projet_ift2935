use tiberius::{AuthMethod, Config};
/// Get the configuration for the MSSQL server
/// # Returns
/// * A Result containing `tiberius::Config` if successful
/// * A Result containing `tiberius::error::Error` if unsuccessful
/// # Example
/// ```rust
/// let config: tiberius::Config = get_config().await?;
///

pub async fn get_config() -> Result<tiberius::Config, tiberius::error::Error> {
    let mut config = Config::new();

    // Change these values to match your MSSQL server configuration and add a
    // certificate if needed
    config.host("localhost");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("SA", "Rust4life"));


    // NOTE: Removed the encryption because I wasn't able to get the TLS 
    // certificate to work properly
    // FIX: Add the certificate and change to tiberius::EncryptionLevel::Required
    config.encryption(tiberius::EncryptionLevel::NotSupported);
    Ok(config)
}
