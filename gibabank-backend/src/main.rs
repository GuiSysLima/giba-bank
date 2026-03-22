use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod handlers;
mod models;

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
        .route("/users", post(handlers::user::create_user))
        .route("/accounts", post(handlers::account::create_account))
        .route("/accounts/:id/deposit", post(handlers::account::deposit))
        .route(
            "/users/:id/accounts",
            get(handlers::account::list_accounts_by_user),
        )
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("🚀 Giba Bank rodando em http://{}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
