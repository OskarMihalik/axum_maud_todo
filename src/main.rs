use axum::routing::patch;
use axum::{routing::get, Router};
use components::main::head;
// use deadpool_postgres::{Config, Runtime};
use services::crud::update_todo;
// use sqlx::sqlite::SqlitePoolOptions;
// use sqlx::Pool;
// use sqlx::{migrate::MigrateDatabase, Sqlite};
use tokio_postgres::{Config, NoTls};
// const DB_URL: &str = "sqlite://sqlite.db";
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

use bb8::{Pool, PooledConnection};
use bb8_postgres::PostgresConnectionManager;

mod components;
mod cornucopia;
mod services;

#[tokio::main]
async fn main() {
    // db connection
    // let mut cfg = Config::new();
    // cfg.user = Some(String::from("admin"));
    // cfg.password = Some(String::from("admin"));
    // cfg.host = Some(String::from("127.0.0.1"));
    // cfg.port = Some(5432);
    // cfg.dbname = Some(String::from("admin"));
    // let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    // let client = pool.get().await.unwrap();

    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=postgres password=admin dbname=admin",
        NoTls,
    )
    .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .route("/todos", get(get_todos).post(create_new_todo))
        // .route("/todos/:id", patch(update_todo).delete(delete_todo))
        .route("/todos/:id", patch(update_todo))
        .route("/todo_app", get(head))
        .with_state(pool)
        .nest("/static", axum_static::static_router("static"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

// /// # ApiError
// #[derive(Debug, Error)]
// pub enum ApiError {
//     #[error(transparent)]
//     DbError(#[from] sea_orm::DbErr),
//     #[error("not found")]
//     NotFound,
// }

// impl IntoResponse for ApiError {
//     fn into_response(self) -> Response {
//         let status = match self {
//             ApiError::NotFound => StatusCode::NOT_FOUND,
//             _ => StatusCode::BAD_REQUEST,
//         };
//         let body = Json(json!({
//             "error": self.to_string(),
//         }));
//         (status, body).into_response()
//     }
// }
