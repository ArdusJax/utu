use anyhow::Result;
use mongodb::{bson::doc, options::ClientOptions, Client};

pub async fn new() -> Result<Client> {
 let mut client_options =
        ClientOptions::parse("mongodb://localhost:27017")
            .await?;
    // Manually set an option
    client_options.app_name = Some("phoenix-fire".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;
    // Ping the server to see if you can connect to the cluster
    client
        .database("attestation")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");
    Ok(client)
}