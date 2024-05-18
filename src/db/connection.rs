use dotenv::dotenv;
use std::env::var;
use tokio_postgres::{connect, Client, Error, NoTls};

pub async fn establish_connection() -> Result<Client, Error> {
    //LOAD ENVIRONMENT VARIABLES
    dotenv().ok();

    //CONSTRUCT DATABASE URL
    let db_user = var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let db_password = var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let db_name = var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let db_hostname = var("POSTGRES_HOSTNAME").expect("POSTGRES_HOSTNAME must be set");
    let db_port = var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");

    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_hostname, db_port, db_name
    );

    //CONNECT TO DATABASE
    let (client, connection) = connect(&database_url, NoTls).await?;

    //Spawn a new task that runs the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("{}", database_url);

    Ok(client)
}
