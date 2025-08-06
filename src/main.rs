#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::SqlitePool;
    use todo_leptos::app::*;
    use todo_leptos::database::create_pool;

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let pool = create_pool().await.expect("Failed to create database pool");

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Create app state that includes both leptos options and database pool
    // #[derive(Clone)]
    // struct AppState {
    //     leptos_options: LeptosOptions,
    //     pool: SqlitePool,
    // }

    // let app_state = AppState {
    //     leptos_options: leptos_options.clone(),
    //     pool: pool.clone(),
    // };

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || provide_context(pool.clone())
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
