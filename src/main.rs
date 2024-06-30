use azure_security_keyvault::KeyvaultClient;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let credential = azure_identity::create_credential().unwrap();
    let client = KeyvaultClient::new("https://akv-sql-ae-tst.vault.azure.net/", credential)
        .unwrap()
        .secret_client();

    let result = client.get("secret_name").await;

    match result {
        Ok(secret) => {
            dbg!(&secret);
        }
        Err(e) => {
            eprintln!("Failed to retrieve secret: {}", e);
        }
    }

    Ok(())
}
