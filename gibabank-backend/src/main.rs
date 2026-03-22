use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(|| async { "Giba Bank API Online!" }))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Giba Bank rodando em http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}