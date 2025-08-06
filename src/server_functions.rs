use crate::models::{CreateTodo, Todo, UpdateTodo};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::auth::get_current_user;
#[cfg(feature = "ssr")]
use crate::database::queries;
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let pool = expect_context::<SqlitePool>();

    let current_user = get_current_user().await?;

    if let Some(user) = current_user {
        queries::get_user_todos(&pool, user.id)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        Err(ServerFnError::ServerError("Not authenticated".to_string()))
    }
}

#[server(AddTodo, "/api")]
pub async fn add_todo(todo: CreateTodo) -> Result<Todo, ServerFnError> {
    let pool = expect_context::<SqlitePool>();

    let current_user = get_current_user().await?;

    if let Some(user) = current_user {
        queries::create_user_todo(&pool, user.id, todo)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        Err(ServerFnError::ServerError("Not authenticated".to_string()))
    }
}

#[server(ToggleTodo, "/api")]
pub async fn toggle_todo(id: i64, completed: bool) -> Result<Option<Todo>, ServerFnError> {
    let pool = expect_context::<SqlitePool>();

    let current_user = get_current_user().await?;

    if let Some(user) = current_user {
        let update = UpdateTodo { completed };
        queries::update_user_todo(&pool, user.id, id, update)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        Err(ServerFnError::ServerError("Not authenticated".to_string()))
    }
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<bool, ServerFnError> {
    let pool = expect_context::<SqlitePool>();

    let current_user = get_current_user().await?;

    if let Some(user) = current_user {
        queries::delete_user_todo(&pool, user.id, id)
            .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
    } else {
        Err(ServerFnError::ServerError("Not authenticated".to_string()))
    }
}
