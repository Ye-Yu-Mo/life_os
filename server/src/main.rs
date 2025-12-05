mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod routes;
mod services;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = config::get_database_url();
    let db = db::establish_connection(&database_url).await?;

    let state = AppState { db };
    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
