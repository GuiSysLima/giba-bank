use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let row = sqlx::query!("SELECT 1 + 1 as result")
        .fetch_one(&pool)
        .await?;

    println!(
        "✅ Teste de query SQLx: 1 + 1 = {}",
        row.result.unwrap_or(0)
    );

    let app = Router::new()
        .route("/", get(|| async { "Giba Bank API Online!" }))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("🚀 Giba Bank rodando em http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
