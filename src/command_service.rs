use axum::{
    Json, 
    http::StatusCode,
    Extension, 
    extract::Path,
};
use crate::{datatypes::{Command, CreateCommand, ConnectionPool}, db_operations};

pub async fn get_command(
    Extension(pool): Extension<ConnectionPool>, 
    Path(command_id): Path<i32>
) -> Result<(StatusCode, Json<Command>),(StatusCode, String)> {
    let command = db_operations::get_command(pool, command_id).await;

    match command {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.1))
    }
}

pub async fn get_random_command(
    Extension(pool): Extension<ConnectionPool>
) -> Result<(StatusCode, Json<Command>),(StatusCode, String)> {
    let command = db_operations::get_random_command(pool).await.unwrap();
    Ok((StatusCode::OK, Json(command)))
}

pub async fn get_commands_by_category(
    Extension(pool): Extension<ConnectionPool>, 
    Path(category): Path<String>
) -> Result<(StatusCode, Json<Vec<Command>>),(StatusCode, String)> {
    let commands = db_operations::get_commands_by_category(pool, category).await;

    match commands {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.1))
    }
}

pub async fn get_commands(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<(StatusCode, Json<Vec<Command>>), (StatusCode, String)> {
    let commands = db_operations::get_commands(pool).await.unwrap();
    
    Ok((StatusCode::OK, Json(commands)))
}

pub async fn post_command(
    Extension(pool): Extension<ConnectionPool>, 
    Json(payload): Json<CreateCommand>
) -> Result<(StatusCode, Json<Command>), (StatusCode, String)> {
    db_operations::save_command(pool, payload).await
}

pub async fn delete_command(
    Extension(pool): Extension<ConnectionPool>,
    Path(command_id): Path<i32>
) -> Result<StatusCode, (StatusCode, String)> {
    db_operations::delete_command(pool, command_id).await
}