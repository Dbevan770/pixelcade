mod auth;
mod db;
mod router;

use db::connect;
use router::app as router;

#[tokio::main]
async fn main() {
    let conn = match connect().await {
        Ok(pool) => Some(pool),
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            None
        }
    };

    if conn.is_none() {
        return;
    }

    let router = router().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
