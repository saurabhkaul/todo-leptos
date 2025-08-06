use crate::auth::*;
use crate::components::auth::{LoginForm, SignupForm};
use crate::components::nav::Navigation;
use crate::models::{CreateTodo, Todo, User};
use crate::server_functions::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    StaticSegment,
};

// Global user state context
#[derive(Clone)]
pub struct UserContext {
    pub user: RwSignal<Option<User>>,
    pub loading: RwSignal<bool>,
}

impl UserContext {
    pub fn new() -> Self {
        Self {
            user: RwSignal::new(None),
            loading: RwSignal::new(false),
        }
    }

    pub async fn check_user(&self) {
        self.loading.set(true);
        match get_current_user().await {
            Ok(user) => self.user.set(user),
            Err(_) => self.user.set(None),
        }
        self.loading.set(false);
    }

    pub fn logout(&self) {
        self.user.set(None);
    }

    pub fn login(&self, user: User) {
        self.user.set(Some(user));
    }
}

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Create and provide global user context
    let user_context = UserContext::new();
    provide_context(user_context.clone());

    // Initial user check
    OnceResource::new({
        let user_context = user_context.clone();
        async move {
            user_context.check_user().await;
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/todo-leptos.css"/>
        <Title text="Todo App - Leptos"/>

        <Router>
            <Navigation/>
            <main class="main-content">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("login") view=LoginPage/>
                    <Route path=StaticSegment("signup") view=SignupPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    let user_context = expect_context::<UserContext>();

    view! {
        {move || {
            if user_context.loading.get() {
                view! { <div>"Loading..."</div> }.into_any()
            } else if user_context.user.get().is_some() {
                view! { <Redirect path="/"/> }.into_any()
            } else {
                view! { <LoginForm/> }.into_any()
            }
        }}
    }
}

#[component]
fn SignupPage() -> impl IntoView {
    let user_context = expect_context::<UserContext>();

    view! {
        {move || {
            if user_context.loading.get() {
                view! { <div>"Loading..."</div> }.into_any()
            } else if user_context.user.get().is_some() {
                view! { <Redirect path="/"/> }.into_any()
            } else {
                view! { <SignupForm/> }.into_any()
            }
        }}
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let user_context = expect_context::<UserContext>();

    view! {
        {move || {
            if user_context.loading.get() {
                view! { <div class="loading">"Loading..."</div> }.into_any()
            } else if let Some(user_data) = user_context.user.get() {
                view! { <TodoApp user=user_data/> }.into_any()
            } else {
                view! { <Redirect path="/login"/> }.into_any()
            }
        }}
    }
}

#[component]
fn TodoApp(user: User) -> impl IntoView {
    let todos = Resource::new(|| (), |_| get_todos());
    let add_todo_action = ServerAction::<AddTodo>::new();
    let toggle_todo_action = ServerAction::<ToggleTodo>::new();
    let delete_todo_action = ServerAction::<DeleteTodo>::new();

    let (new_todo_title, set_new_todo_title) = signal(String::new());
    let (search_id, set_search_id) = signal(String::new());
    let (search_result, set_search_result) = signal(None::<Option<Todo>>);
    let (search_error, set_search_error) = signal(None::<String>);

    let submit_todo = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let title = new_todo_title.get();
        if !title.trim().is_empty() {
            add_todo_action.dispatch(AddTodo {
                todo: CreateTodo {
                    title: title.trim().to_string(),
                },
            });
            set_new_todo_title.set(String::new());
        }
    };

    let search_todo = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let id_str = search_id.get();
        if !id_str.trim().is_empty() {
            if let Ok(id) = id_str.trim().parse::<i64>() {
                spawn_local(async move {
                    set_search_error.set(None);
                    match get_todo_by_id(id).await {
                        Ok(todo) => {
                            set_search_result.set(Some(todo));
                        }
                        Err(e) => {
                            set_search_result.set(None);
                            set_search_error.set(Some(e.to_string()));
                        }
                    }
                });
            } else {
                set_search_error.set(Some("Please enter a valid number".to_string()));
                set_search_result.set(None);
            }
        }
    };

    Effect::new(move |_| {
        add_todo_action.version().get();
        toggle_todo_action.version().get();
        delete_todo_action.version().get();
        todos.refetch();
    });

    view! {
        <div class="container">
            <div class="welcome-header">
                <h1>"Your Todos"</h1>
                <p class="user-welcome">"Welcome back, " {user.username.clone()}</p>
            </div>

            <form on:submit=submit_todo class="todo-form">
                <div class="input-group">
                    <input
                        type="text"
                        placeholder="Add a new todo..."
                        prop:value=new_todo_title
                        on:input=move |ev| set_new_todo_title.set(event_target_value(&ev))
                    />
                    <button type="submit">"Add"</button>
                </div>
            </form>

            <form on:submit=search_todo class="search-form">
                <div class="input-group">
                    <input
                        type="text"
                        placeholder="Search by todo ID..."
                        prop:value=search_id
                        on:input=move |ev| set_search_id.set(event_target_value(&ev))
                    />
                    <button type="submit">"Search"</button>
                </div>
            </form>

            {move || {
                if let Some(error) = search_error.get() {
                    view! { <p class="error">"Search error: " {error}</p> }.into_any()
                } else if let Some(result) = search_result.get() {
                    if let Some(todo) = result {
                        view! {
                            <div class="search-result">
                                <h3>"Search Result:"</h3>
                                <div class="todo-item">
                                    <span class="todo-id">"ID: " {todo.id}</span>
                                    <span class="todo-title" class:completed=todo.completed>{todo.title}</span>
                                    <span class="todo-status">{if todo.completed { "✓ Completed" } else { "○ Pending" }}</span>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <p class="no-result">"No todo found with that ID"</p> }.into_any()
                    }
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            <Suspense fallback=move || view! { <p class="loading">"Loading todos..."</p> }>
                {move || {
                    match todos.get() {
                        Some(Ok(todos_list)) => {
                            if todos_list.is_empty() {
                                view! { <p class="empty-state">"No todos yet. Add one above!"</p> }.into_any()
                            } else {
                                view! {
                                    <ul class="todo-list">
                                        <For
                                            each=move || todos_list.clone()
                                            key=|todo| todo.id
                                            children=move |todo: Todo| {
                                                let todo_id = todo.id;
                                                let is_completed = todo.completed;
                                                let todo_title = todo.title.clone();

                                                view! {
                                                    <li class:completed=is_completed>
                                                        <div class="todo-content">
                                                            <input
                                                                type="checkbox"
                                                                checked=is_completed
                                                                on:change=move |_| {
                                                                    toggle_todo_action.dispatch(ToggleTodo {
                                                                        id: todo_id,
                                                                        completed: !is_completed,
                                                                    });
                                                                }
                                                            />
                                                            <span class="todo-id">"ID: " {todo_id} " - "</span>
                                                            <span class="todo-title">{todo_title}</span>
                                                        </div>
                                                        <button
                                                            class="delete-btn"
                                                            on:click=move |_| {
                                                                delete_todo_action.dispatch(DeleteTodo { id: todo_id });
                                                            }
                                                        >
                                                            "✕"
                                                        </button>
                                                    </li>
                                                }
                                            }
                                        />
                                    </ul>
                                }.into_any()
                            }
                        }
                        Some(Err(e)) => view! { <p class="error">"Error loading todos: " {e.to_string()}</p> }.into_any(),
                        None => view! { <p class="loading">"Loading todos..."</p> }.into_any()
                    }
                }}
            </Suspense>
        </div>
    }
}
