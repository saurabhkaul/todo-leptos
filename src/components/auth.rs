use crate::auth::*;
use crate::models::{LoginUser, RegisterUser, User};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn LoginForm() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();
    let navigate = use_navigate();
    let user_context = expect_context::<crate::app::UserContext>();

    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);

    let submit_login = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error_message.set(None);

        let login_data = LoginUser {
            username: username.get(),
            password: password.get(),
        };

        login_action.dispatch(Login { login_data });
    };

    Effect::new(move |_| {
        if let Some(result) = login_action.value().get() {
            match result {
                Ok(user) => {
                    user_context.login(user);
                    navigate("/", Default::default());
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
        }
    });

    view! {
        <div class="auth-container">
            <div class="auth-form">
                <h2>"Login"</h2>

                {move || error_message.get().map(|msg|
                    view! { <div class="error-message">{msg}</div> }
                )}

                <form on:submit=submit_login>
                    <div class="form-group">
                        <label for="username">"Username:"</label>
                        <input
                            id="username"
                            type="text"
                            required
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">"Password:"</label>
                        <input
                            id="password"
                            type="password"
                            required
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    <button type="submit" disabled=move || login_action.pending().get()>
                        {move || if login_action.pending().get() { "Logging in..." } else { "Login" }}
                    </button>
                </form>

                <p class="auth-link">
                    "Don't have an account? "
                    <a href="/signup">"Sign up"</a>
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn SignupForm() -> impl IntoView {
    let register_action = ServerAction::<Register>::new();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);

    let submit_register = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_error_message.set(None);

        if password.get() != confirm_password.get() {
            set_error_message.set(Some("Passwords do not match".to_string()));
            return;
        }

        if username.get().trim().is_empty()
            || email.get().trim().is_empty()
            || password.get().len() < 6
        {
            set_error_message.set(Some(
                "Please fill all fields. Password must be at least 6 characters.".to_string(),
            ));
            return;
        }

        let register_data = RegisterUser {
            username: username.get().trim().to_string(),
            email: email.get().trim().to_string(),
            password: password.get(),
        };

        register_action.dispatch(Register {
            user_data: register_data,
        });
    };

    Effect::new(move |_| {
        if let Some(result) = register_action.value().get() {
            match result {
                Ok(_) => {
                    navigate("/login", Default::default());
                }
                Err(e) => {
                    set_error_message.set(Some(e.to_string()));
                }
            }
        }
    });

    view! {
        <div class="auth-container">
            <div class="auth-form">
                <h2>"Sign Up"</h2>

                {move || error_message.get().map(|msg|
                    view! { <div class="error-message">{msg}</div> }
                )}

                <form on:submit=submit_register>
                    <div class="form-group">
                        <label for="username">"Username:"</label>
                        <input
                            id="username"
                            type="text"
                            required
                            prop:value=username
                            on:input=move |ev| set_username.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-group">
                        <label for="email">"Email:"</label>
                        <input
                            id="email"
                            type="email"
                            required
                            prop:value=email
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-group">
                        <label for="password">"Password:"</label>
                        <input
                            id="password"
                            type="password"
                            required
                            prop:value=password
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="form-group">
                        <label for="confirm_password">"Confirm Password:"</label>
                        <input
                            id="confirm_password"
                            type="password"
                            required
                            prop:value=confirm_password
                            on:input=move |ev| set_confirm_password.set(event_target_value(&ev))
                        />
                    </div>

                    <button type="submit" disabled=move || register_action.pending().get()>
                        {move || if register_action.pending().get() { "Creating account..." } else { "Sign Up" }}
                    </button>
                </form>

                <p class="auth-link">
                    "Already have an account? "
                    <a href="/login">"Login"</a>
                </p>
            </div>
        </div>
    }
}
