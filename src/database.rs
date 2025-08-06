#[cfg(feature = "ssr")]
use bcrypt::{hash, verify, DEFAULT_COST};
#[cfg(feature = "ssr")]
use chrono::{Duration, Utc};
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;
#[cfg(feature = "ssr")]
use uuid::Uuid;

#[cfg(feature = "ssr")]
pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:Todos.db".to_string());

    let pool = SqlitePool::connect(&database_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("db is being provided");

    Ok(pool)
}

#[cfg(feature = "ssr")]
pub mod queries {
    use super::*;
    use crate::models::{CreateTodo, LoginUser, RegisterUser, Session, Todo, UpdateTodo, User};

    // User queries
    pub async fn create_user(
        pool: &SqlitePool,
        user_data: RegisterUser,
    ) -> Result<User, sqlx::Error> {
        let password_hash = hash(&user_data.password, DEFAULT_COST)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let row = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES (?, ?, ?) RETURNING id, username, email, password_hash, created_at",
            user_data.username,
            user_data.email,
            password_hash
        )
        .fetch_one(pool)
        .await?;

        Ok(User {
            id: row.id,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        })
    }

    pub async fn authenticate_user(
        pool: &SqlitePool,
        login_data: LoginUser,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = ?",
            login_data.username
        )
        .fetch_optional(pool)
        .await?;

        let user = row.map(|row| User {
            id: row.id.unwrap_or(0),
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        });

        if let Some(user) = user {
            let is_valid = verify(&login_data.password, &user.password_hash)
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

            if is_valid {
                Ok(Some(user))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_id(
        pool: &SqlitePool,
        user_id: i64,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|row| User {
            id: row.id,
            username: row.username,
            email: row.email,
            password_hash: row.password_hash,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
        }))
    }

    // Session queries
    pub async fn create_session(pool: &SqlitePool, user_id: i64) -> Result<Session, sqlx::Error> {
        let session_id = Uuid::new_v4().to_string();
        let expires_at = (Utc::now() + Duration::days(30))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();

        let row = sqlx::query!(
            "INSERT INTO sessions (id, user_id, expires_at) VALUES (?, ?, ?) RETURNING id, user_id, created_at, expires_at",
            session_id,
            user_id,
            expires_at
        )
        .fetch_one(pool)
        .await?;

        Ok(Session {
            id: row.id.unwrap_or_default(),
            user_id: row.user_id,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
            expires_at: row.expires_at.to_string(),
        })
    }

    pub async fn get_session(
        pool: &SqlitePool,
        session_id: &str,
    ) -> Result<Option<(Session, User)>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT s.id, s.user_id, s.created_at, s.expires_at, u.username, u.email, u.password_hash, u.created_at as user_created_at
             FROM sessions s
             JOIN users u ON s.user_id = u.id
             WHERE s.id = ? AND s.expires_at > datetime('now')",
            session_id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(row) = row {
            let session = Session {
                id: row.id.unwrap_or_default(),
                user_id: row.user_id,
                created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                expires_at: row.expires_at.to_string(),
            };

            let user = User {
                id: row.user_id,
                username: row.username,
                email: row.email,
                password_hash: row.password_hash,
                created_at: row
                    .user_created_at
                    .map(|dt| dt.to_string())
                    .unwrap_or_default(),
            };

            Ok(Some((session, user)))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!("DELETE FROM sessions WHERE id = ?", session_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    // Updated todo queries with user filtering
    pub async fn get_user_todos(pool: &SqlitePool, user_id: i64) -> Result<Vec<Todo>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, title, completed, created_at, updated_at, user_id FROM todos WHERE user_id = ? ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Todo {
                id: row.id,
                title: row.title,
                completed: row.completed,
                created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
                updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
                user_id: row.user_id,
            })
            .collect())
    }
    
    pub async fn get_user_todo_by_id(
        pool: &SqlitePool,
        user_id:i64,
        todo_id:i64,
    ) ->Result<Option<Todo>,sqlx::Error>{
        let row = sqlx::query!(
            "SELECT id, title, completed, created_at, updated_at, user_id FROM todos WHERE user_id = ? AND id = ? ORDER BY created_at DESC",
            user_id,
            todo_id
        )
        .fetch_optional(pool)
        .await?;
        
        Ok(row.map(|row| Todo {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            user_id: row.user_id,
        }))
        
    }

    pub async fn create_user_todo(
        pool: &SqlitePool,
        user_id: i64,
        todo: CreateTodo,
    ) -> Result<Todo, sqlx::Error> {
        let row = sqlx::query!(
            "INSERT INTO todos (title, user_id) VALUES (?, ?) RETURNING id, title, completed, created_at, updated_at, user_id",
            todo.title,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(Todo {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            user_id: row.user_id,
        })
    }

    pub async fn update_user_todo(
        pool: &SqlitePool,
        user_id: i64,
        todo_id: i64,
        update: UpdateTodo,
    ) -> Result<Option<Todo>, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "UPDATE todos SET completed = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND user_id = ?",
            update.completed,
            todo_id,
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            return Ok(None);
        }

        let row = sqlx::query!(
            "SELECT id, title, completed, created_at, updated_at, user_id FROM todos WHERE id = ? AND user_id = ?",
            todo_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|row| Todo {
            id: row.id,
            title: row.title,
            completed: row.completed,
            created_at: row.created_at.map(|dt| dt.to_string()).unwrap_or_default(),
            updated_at: row.updated_at.map(|dt| dt.to_string()).unwrap_or_default(),
            user_id: row.user_id,
        }))
    }

    pub async fn delete_user_todo(
        pool: &SqlitePool,
        user_id: i64,
        todo_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query!(
            "DELETE FROM todos WHERE id = ? AND user_id = ?",
            todo_id,
            user_id
        )
        .execute(pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}
