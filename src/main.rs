mod db_operations;
mod datatypes;
mod errorhandler;
mod snippet_service;

use axum::{
    routing::get,
    Router, 
    Extension,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::snippet_service::{get_snippets, get_snippet, delete_snippet, get_snippets_by_category, get_random_snippet, post_snippet};

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
        .route("/snippets", get(get_snippets).post(post_snippet))
        .route("/snippets/:snippet_id", get(get_snippet).delete(delete_snippet))
        .route("/snippets/random", get(get_random_snippet))
        .route("/snippets/category/:category_id", get(get_snippets_by_category))
        .layer(Extension(pool));


    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}