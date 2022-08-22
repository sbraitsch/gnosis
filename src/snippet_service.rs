use axum::{
    Json, 
    http::StatusCode,
    Extension, 
    extract::Path,
};
use crate::{datatypes::{Snippet, CreateSnippet, ConnectionPool}, db_operations};

pub async fn get_snippet(
    Extension(pool): Extension<ConnectionPool>, 
    Path(snippet_id): Path<i32>
) -> Result<(StatusCode, Json<Snippet>),(StatusCode, String)> {
    let snippet = db_operations::get_snippet(pool, snippet_id).await;

    match snippet {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.1))
    }
}

pub async fn get_random_snippet(
    Extension(pool): Extension<ConnectionPool>
) -> Result<(StatusCode, Json<Snippet>),(StatusCode, String)> {
    let snippet = db_operations::get_random_snippet(pool).await.unwrap();
    Ok((StatusCode::OK, Json(snippet)))
}

pub async fn get_snippets_by_category(
    Extension(pool): Extension<ConnectionPool>, 
    Path(category): Path<String>
) -> Result<(StatusCode, Json<Vec<Snippet>>),(StatusCode, String)> {
    let snippets = db_operations::get_snippets_by_category(pool, category).await;

    match snippets {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(error) => Err((StatusCode::NOT_FOUND, error.1))
    }
}

pub async fn get_snippets(
    Extension(pool): Extension<ConnectionPool>,
) -> Result<(StatusCode, Json<Vec<Snippet>>), (StatusCode, String)> {
    let snippets = db_operations::get_snippets(pool).await.unwrap();
    
    Ok((StatusCode::OK, Json(snippets)))
}

pub async fn post_snippet(
    Extension(pool): Extension<ConnectionPool>, 
    Json(payload): Json<CreateSnippet>
) -> Result<(StatusCode, Json<Snippet>), (StatusCode, String)> {
    db_operations::save_snippet(pool, payload).await
}

pub async fn delete_snippet(
    Extension(pool): Extension<ConnectionPool>,
    Path(snippet_id): Path<i32>
) -> Result<StatusCode, (StatusCode, String)> {
    db_operations::delete_snippet(pool, snippet_id).await
}