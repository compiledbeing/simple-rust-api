mod app;
mod router;
mod state;
mod handlers;
mod models;
mod error;
mod auth;
mod middleware;


#[tokio::main]
async fn main () {
    
    app::run().await
   
}




