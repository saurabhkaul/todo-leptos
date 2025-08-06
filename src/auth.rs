use crate::models::{LoginUser, RegisterUser, User};
use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::database::queries;
#[cfg(feature = "ssr")]
use axum::http::{header::SET_COOKIE, HeaderValue};
#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;
#[cfg(feature = "ssr")]
use sqlx::SqlitePool;

#[server(Register, "/api")]
pub async fn register(user_data: RegisterUser) -> Result<User, ServerFnError> {
    let pool = expect_context::<SqlitePool>();
    // Check if username or email already exists
    let existing = sqlx::query!(
        "SELECT id FROM users WHERE username = ? OR email = ?",
        user_data.username,
        user_data.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if existing.is_some() {
        return Err(ServerFnError::new(
            "Username or email already exists".to_string(),
        ));
    }

    queries::create_user(&pool, user_data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(Login, "/api")]
pub async fn login(login_data: LoginUser) -> Result<User, ServerFnError> {
    let pool = expect_context::<SqlitePool>();
    let response = expect_context::<ResponseOptions>();

    let user = queries::authenticate_user(&pool, login_data)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(user) = user {
        // Create session
        let session = queries::create_session(&pool, user.id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        // Set session cookie
        let cookie = format!(
            "session_id={}; Path=/; HttpOnly; SameSite=Strict; Max-Age=2592000",
            session.id
        );
        response.insert_header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());

        Ok(user)
    } else {
        Err(ServerFnError::new("Invalid credentials".to_string()))
    }
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    let pool = expect_context::<SqlitePool>();
    let response = expect_context::<ResponseOptions>();

    // Get session from cookie
    if let Some(session_id) = get_session_id().await {
        queries::delete_session(&pool, &session_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    // Clear session cookie
    let cookie = "session_id=; Path=/; HttpOnly; SameSite=Strict; Max-Age=0";
    response.insert_header(SET_COOKIE, HeaderValue::from_str(cookie).unwrap());

    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    let pool = expect_context::<SqlitePool>();

    if let Some(session_id) = get_session_id().await {
        let result = queries::get_session(&pool, &session_id)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(result.map(|(_, user)| user))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "ssr")]
async fn get_session_id() -> Option<String> {
    use leptos_axum::extract;

    // Extract headers from the request
    let headers = extract::<axum::http::HeaderMap>().await.ok()?;
    let cookie_header = headers.get("cookie")?;
    let cookie_str = cookie_header.to_str().ok()?;

    for cookie in cookie_str.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with("session_id=") {
            return Some(cookie[11..].to_string());
        }
    }
    None
}
