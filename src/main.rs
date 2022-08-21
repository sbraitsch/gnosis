mod db_operations;
mod datatypes;
mod errorhandler;

use axum::{
    routing::{get, delete},
    Json, 
    http::StatusCode,
    Router, Extension, extract::Path,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use datatypes::{Command, CreateCommand};
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
        .route("/commands/:command_id", delete(delete_command))
        .layer(Extension(pool));


    let addr = SocketAddr::from(([127,0,0,1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

async fn get_commands(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<(StatusCode, Json<Vec<Command>>), (StatusCode, String)> {
    let commands = db_operations::get_commands(pool).await.unwrap();

    Ok((StatusCode::OK, Json(commands)))
}

async fn post_command(Extension(pool): Extension<ConnectionPool>, Json(payload): Json<CreateCommand>) 
-> Result<(StatusCode, Json<Command>), (StatusCode, String)> {
    db_operations::save_command(pool, payload).await
}

async fn delete_command(Extension(pool): Extension<ConnectionPool>, Path(command_id): Path<i32>)
-> Result<StatusCode, (StatusCode, String)> {
    db_operations::delete_command(pool, command_id).await
}