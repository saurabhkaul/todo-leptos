use crate::auth::*;
use crate::models::User;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Navigation() -> impl IntoView {
    let user_context = expect_context::<crate::app::UserContext>();
    let logout_action = ServerAction::<Logout>::new();
    let navigate = use_navigate();

    let handle_logout = move |_| {
        logout_action.dispatch(Logout {});
    };

    let user_context_clone = user_context.clone();
    Effect::new(move |_| {
        if let Some(result) = logout_action.value().get() {
            if result.is_ok() {
                user_context_clone.logout();
                navigate("/login", Default::default());
            }
        }
    });

    view! {
        <nav class="navbar">
            <div class="nav-brand">
                <a href="/">"Todo App"</a>
            </div>

            <div class="nav-menu">
                {move || {
                    if let Some(user_data) = user_context.user.get() {
                        view! {
                            <span class="user-info">"Welcome, " {user_data.username}</span>
                            <button
                                class="logout-btn"
                                on:click=handle_logout
                                disabled=move || logout_action.pending().get()
                            >
                                {move || if logout_action.pending().get() { "Logging out..." } else { "Logout" }}
                            </button>
                        }.into_any()
                    } else {
                        view! {
                            <div class="auth-links">
                                <a href="/login">"Login"</a>
                                <a href="/signup">"Sign Up"</a>
                            </div>
                        }.into_any()
                    }
                }}
            </div>
        </nav>
    }
}
