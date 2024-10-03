use auth_data::BearerData;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use std::future::IntoFuture;
use tokio::select;
use tracing::error;

mod auth_data;
mod auth_middleware;
mod bug_report_data;
mod bug_report_handlers;
mod bug_report_mailer;
mod transmit_via_async_smtp;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    tracing::info!("Starting up the Accredo tool Server");

    tracing::info!("Reading the environment");
    dotenv::dotenv().ok();
    let dbhost = std::env::var("PG_HOST").expect("PG_HOST not set");
    let dbport: u16 = std::env::var("PG_PORT")
        .expect("PG_PORT not set")
        .parse()
        .unwrap();

    tracing::info!("Creating Postgres Pool");
    let pool_options = sqlx::postgres::PgConnectOptions::new()
        .host(&dbhost)
        .port(dbport)
        .database("postgres")
        .username("postgres")
        .password("postgres");
    let pool = sqlx::PgPool::connect_with(pool_options)
        .await
        .expect("Error with pool connection");

    tracing::info!("Creating table bug_report if it does not exist");

    // Add table if not existing
    let _ = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS bug_report (
            id serial,
            mail_address text,
            subject text,
            mail_body text,
            detail text,
            screenshot bytea
        );"#,
    )
    .execute(&pool)
    .await;

    tracing::info!("Building the routing");

    let data = BearerData {
        token: "00000".into(),
    };

    let app = Router::new()
        .route(
            "/api/bug_report",
            post(bug_report_handlers::receive_bug_report),
        )
        .route_layer(axum::middleware::from_fn(auth_middleware::authenticate))
        .layer(Extension(data))
        .route("/", get(root))
        .with_state(pool.clone());

    let app = app.fallback(handler_404);

    tracing::info!("Listening on {}", "0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let serve_future = axum::serve(listener, app).into_future();

    select! {
        res = serve_future => {
            if let Err(e) = res {
                error!("Metrics endpoint serve failure: {:?}", e);
            }
        },
        res = bug_report_mailer::do_monitoring(&pool) => {
            if let Err(e) = res {
                error!("Login or Parse error, double check credentials and connectivity: {:?}", e);
            }
        },
    }

    tracing::info!("Accredo tool Server now running & listening");

    println!("Hello, world!");
}

async fn handler_404() -> impl IntoResponse {
    println!("404 Response - Invalid URI presented");
    (StatusCode::NOT_FOUND, "nothing to see here")
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    println!("root handler");
    "Hello from the Accredo Tool Server!"
}
