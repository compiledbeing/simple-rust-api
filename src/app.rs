use crate::{router::create_router, state::AppState};
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::fmt::init;


pub async fn run() {
    init();
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL")
        .unwrap();

    let db = PgPoolOptions ::new()
        .max_connections(5)    
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState {db};

    let app = create_router(state);

    println!("Listening at http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap(),
        app,
    )
    .await
    .unwrap();

}