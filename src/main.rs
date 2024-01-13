use axum::routing::patch;
use axum::{routing::get, Router};
use components::main::head;
use services::crud::{create_new_todo, delete_todo, get_todos, update_todo};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Pool;
use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_URL: &str = "sqlite://sqlite.db";

mod components;
mod services;

#[tokio::main]
async fn main() {
    // db connection
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
    let db_pool = SqlitePoolOptions::new().connect(DB_URL).await.unwrap();

    setup_db(&db_pool).await;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/todos", get(get_todos).post(create_new_todo))
        .route("/todos/:id", patch(update_todo).delete(delete_todo))
        .route("/todo_app", get(head))
        .with_state(db_pool)
        .nest("/static", axum_static::static_router("static"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn setup_db(db: &Pool<Sqlite>) {
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS todo (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(db).await.unwrap();
    println!("Create user table result: {:?}", result);
}
