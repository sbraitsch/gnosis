use axum::{
    routing::get,
    routing::post,
    Json, 
    http::StatusCode,
    Router, Extension,
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // set up connection pool
    let manager =
    PostgresConnectionManager::new_from_stringlike("host=localhost port=5432 user=postgres password=admin", NoTls)
        .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let app = Router::new()
        .route("/commands", get(get_commands))
        .route("/commands", post(post_command))
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
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM commandtable", &[])
        .await
        .map_err(internal_error)?;
        
    let parsed_result = result.into_iter()
    .map(
        |row| Command {
        id: row.get(0),
        category: row.get(1),
        description: row.get(2),
        code: row.get(3),
    }).collect();

    Ok((StatusCode::OK, Json(parsed_result)))
}

async fn post_command(Extension(pool): Extension<ConnectionPool>, Json(payload): Json<Command>) 
-> Result<(StatusCode, Json<Command>), (StatusCode, String)> {

    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("INSERT INTO commandtable (id, category, description, code) VALUES ($1, $2, $3, $4)", &[&payload.id, &payload.category, &payload.description, &payload.code])
        .await
        .map_err(internal_error)?;

    let command = Command { 
        id: payload.id, 
        category: payload.category, 
        description: payload.description,
        code: payload.code
    }; 

    Ok((StatusCode::CREATED, Json(command)))
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Command {
    id: i32,
    category: String,
    description: String,
    code: String,
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}