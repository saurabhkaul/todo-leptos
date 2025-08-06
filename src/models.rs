#[cfg(feature = "ssr")]
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}
