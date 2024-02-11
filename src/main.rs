use axum::routing::patch;

use axum::{routing::get, Router};
use components::main::head;

use services::crud::{create_new_todo, delete_todo, get_todos, update_todo};

use tokio_postgres::NoTls;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

mod components;
mod cornucopia;
mod services;

#[tokio::main]
async fn main() {
    let manager = PostgresConnectionManager::new_from_stringlike(
        "host=localhost user=admin password=admin dbname=admin",
        NoTls,
    )
    .unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(get_todos).post(create_new_todo))
        .route("/todos/:id", patch(update_todo).delete(delete_todo))
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
