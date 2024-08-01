use std::sync::Arc;

use axum::{routing::get, Router};
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
pub struct AppState {
    db: Client
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    tracing_subscriber::fmt::init();

    let uri = "mongodb://root:example@localhost/";
    let mut client_options =
        ClientOptions::parse(uri)
            .await?;
            
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    let app = Router::new().route("/", get(root)).with_state(Arc::new(AppState { db: client.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, world!"
}