use azure_core::auth::TokenCredential;
use azure_identity::AzureCliCredential;
use azure_security_keyvault::SecretClient;
use std::{error::Error, sync::Arc};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // log into Azure using the Azure CLI credential 
    // ie; run 'az login' first, this is cached
    let sub_id = AzureCliCredential::get_subscription()?;
    println!("Azure subscription: {sub_id}");

    let creds = AzureCliCredential::new();
    let res = creds
        .get_token(&["https://management.azure.com/.default"])
        .await?;
    println!("{res:?}");

    // get AKV stuff
    let keyvault_url = env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    
    // get the secret
    let client = SecretClient::new(&keyvault_url, Arc::new(creds))?;
    let secret_name = "ContosoHRSQLStringwithAE";
    let secret = client.get(secret_name).await?;
    print!("{} is '{}'",secret_name, secret.value);

    Ok(())
}

/*
use azure_identity::AzureCliCredential;
use azure_core::TokenCredential;
use azure_identity;
//use futures::stream::StreamExt;
use std::env;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    tracing_subscriber::fmt().init();

    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");

    let credential = azure_identity::create_credential()?;

    let client = SecretClient::new(&keyvault_url, credential)?;

    let mut stream = client.list_secrets().into_stream();
    while let Some(response) = stream.next().await {
        println!("{:#?}", response?);
    }

    Ok(())
}
 */