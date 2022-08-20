use axum::{http::StatusCode, Json};
use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;
use tokio_postgres::NoTls;

use crate::{datatypes::Command, errorhandler::internal_error};

pub async fn save_command(pool: Pool<PostgresConnectionManager<NoTls>>, payload: Command)
-> Result<(StatusCode, Json<Command>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("INSERT INTO commandtable (id, category, description, code) VALUES ($1, $2, $3, $4)", &[&payload.id, &payload.category, &payload.description, &payload.code])
        .await
        .map_err(internal_error)?;
    
    Ok((StatusCode::CREATED, Json(payload)))
}

pub async fn get_commands(pool: Pool<PostgresConnectionManager<NoTls>>)
->  Result<Vec<Command>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM commandtable", &[])
        .await
        .map_err(internal_error)?;

    let command_list = result.into_iter().map(
        |row| Command {
        id: row.get(0),
        category: row.get(1),
        description: row.get(2),
        code: row.get(3),
    }).collect();

    Ok(command_list)
}