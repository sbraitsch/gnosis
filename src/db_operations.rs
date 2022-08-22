use axum::{http::StatusCode, Json};

use crate::{datatypes::{Snippet, CreateSnippet, ConnectionPool}, errorhandler::internal_error};

static DB_INIT_SCRIPT: &'static str = include_str!("../db.sql");

pub async fn init_db(pool: &ConnectionPool) -> Result<(), (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    
    conn.batch_execute(DB_INIT_SCRIPT).await.map_err(internal_error)?;

    Ok(())
}

pub async fn get_snippet(pool: ConnectionPool, snippet_id: i32)
-> Result<Snippet, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query_one("SELECT * FROM snippets WHERE id = $1", &[&snippet_id])
        .await
        .map_err(internal_error)?;
    
    Ok(Snippet::from_row(result))
}

pub async fn get_snippets_by_category(pool: ConnectionPool, category: String)
-> Result<Vec<Snippet>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM snippets WHERE category = $1", &[&category])
        .await
        .map_err(internal_error)?;
    
    let snippet_list = result.into_iter().map(
        |row| Snippet::from_row(row)).collect();

    Ok(snippet_list)
}

pub async fn get_random_snippet(pool: ConnectionPool)
-> Result<Snippet, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query_one("SELECT * FROM snippets ORDER BY random() LIMIT 1", &[])
        .await
        .map_err(internal_error)?;
    
    Ok(Snippet::from_row(result))
}

pub async fn get_snippets(pool: ConnectionPool)
->  Result<Vec<Snippet>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let result = conn
        .query("SELECT * FROM snippets LEFT JOIN categories ON category_id = categories.id", &[])
        .await
        .map_err(internal_error)?;

    let snippet_list = result.into_iter().map(
        |row| Snippet::from_row(row)).collect();

    Ok(snippet_list)
}

pub async fn save_snippet(pool: ConnectionPool, payload: CreateSnippet)
-> Result<(StatusCode, Json<Snippet>), (StatusCode, String)> {

    let saved_cat: i32 = if payload.category.id.is_none() {
        if payload.category.name.is_none() {
            return Err((StatusCode::BAD_REQUEST, "Either id or name for category is needed.".to_string()))
        } else {
            save_category(&pool, &payload.category.name.unwrap()).await.unwrap()
        }
    } else {
        payload.category.id.unwrap()
    };

    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("INSERT INTO snippets (category_id, description, code) VALUES ($1, $2, $3)", &[&saved_cat, &payload.description, &payload.code])
        .await
        .map_err(internal_error)?;   
    
    let created = conn
        .query_one("SELECT * FROM snippets LEFT JOIN categories ON category_id = categories.id ORDER BY snippets.id DESC LIMIT 1", &[])
        .await
        .map_err(internal_error);    

    Ok((StatusCode::CREATED, Json(Snippet::from_row(created.unwrap()))))
}

pub async fn save_category(pool: &ConnectionPool, name: &String)
-> Result<i32, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn
    .execute("INSERT INTO categories (name) VALUES ($1)", &[&name])
    .await
    .map_err(internal_error)?;

    let created = conn
        .query_one("SELECT * FROM categories ORDER BY id DESC LIMIT 1", &[])
        .await
        .map_err(internal_error)?;

    Ok(created.get("id"))
}

pub async fn delete_snippet(pool: ConnectionPool, snippet_id: i32)
-> Result<StatusCode, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn
        .execute("DELETE FROM snippets WHERE id = $1", &[&snippet_id])
        .await
        .map_err(internal_error)?;
    
    Ok(StatusCode::NO_CONTENT)
}