use std::fs;
use axum::{http::StatusCode, Json};
use bb8_postgres::PostgresConnectionManager;
use bb8::Pool;
use tokio_postgres::NoTls;

use crate::{datatypes::{Command, CreateCommand}, errorhandler::internal_error};

pub async fn init_db(pool: &Pool<PostgresConnectionManager<NoTls>>) -> Result<(), (StatusCode, String)> {
    let init_file = fs::read_to_string("./db.sql").map_err(internal_error)?;
    let conn = pool.get().await.map_err(internal_error)?;
    
    conn.batch_execute(init_file.as_str()).await.map_err(internal_error)?;

    Ok(())
}

pub async fn get_commands(pool: Pool<PostgresConnectionManager<NoTls>>)
->  Result<Vec<Command>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM commandtable", &[])
        .await
        .map_err(internal_error)?;

    let command_list = result.into_iter().map(
        |row| Command::from_row(row)).collect();

    Ok(command_list)
}

pub async fn save_command(pool: Pool<PostgresConnectionManager<NoTls>>, payload: CreateCommand)
-> Result<(StatusCode, Json<Command>), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("INSERT INTO commandtable (category, description, code) VALUES ($1, $2, $3)", &[&payload.category, &payload.description, &payload.code])
        .await
        .map_err(internal_error)?;
    
    let created = conn
        .query_one("SELECT * FROM commandtable ORDER BY id DESC LIMIT 1", &[])
        .await
        .map_err(internal_error);

    Ok((StatusCode::CREATED, Json(Command::from_row(created.unwrap()))))
}

pub async fn delete_command(pool: Pool<PostgresConnectionManager<NoTls>>, command_id: i32)
-> Result<StatusCode, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("DELETE FROM commandtable WHERE id = $1", &[&command_id])
        .await
        .map_err(internal_error)?;
    
    Ok(StatusCode::NO_CONTENT)
}