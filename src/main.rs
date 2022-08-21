mod db_operations;
mod datatypes;
mod errorhandler;
mod command_service;

use axum::{
    routing::get,
    Router, 
    Extension,
};
use command_service::{get_command, get_commands, get_commands_by_category, get_random_command, post_command, delete_command};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();


    let manager = PostgresConnectionManager::new_from_stringlike("host=localhost port=5432 user=postgres password=admin", NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    db_operations::init_db(&pool).await.expect("database initialization");

    let app = Router::new()
        .route("/commands", get(get_commands).post(post_command))
        .route("/commands/:command_id", get(get_command).delete(delete_command))
        .route("/commands/random", get(get_random_command))
        .route("/commands/category/:category_id", get(get_commands_by_category))
        .layer(Extension(pool));


    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}