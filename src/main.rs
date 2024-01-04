use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::patch;
use axum::Form;
use axum::{routing::get, Router};
use axum_htmx::HxResponseTrigger;
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{migrate::MigrateDatabase, Sqlite};
use sqlx::{Pool, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

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
        .route("/todos/:id", patch(update_todo))
        // .route("/todos/:id", post())
        .route("/html", get(head))
        .with_state(db_pool)
        .nest("/static", axum_static::static_router("static"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct Todo {
    id: i64,
    name: String,
}

async fn get_todos(State(db_pool): State<SqlitePool>) -> Markup {
    let result = sqlx::query_as!(Todo, "SELECT id, name FROM todo")
        .fetch_all(&db_pool)
        .await;
    // println!("result: {:?}", result)
    match result {
        Ok(todos) => {
            // let new_todo: Vec<&str> = todos.iter().map(|record| record.name.as_str()).collect();
            return todos_items(todos);
        }
        Err(error) => return error_response(&error.to_string()),
    }
}

#[derive(Deserialize)]
struct NewTodo {
    name: String,
}

async fn create_new_todo(
    State(db_pool): State<SqlitePool>,
    Form(new_todo): Form<NewTodo>,
) -> impl IntoResponse {
    let result = sqlx::query!("INSERT INTO todo (name) values (?1)", new_todo.name)
        .execute(&db_pool)
        .await;
    match result {
        Ok(_) => {
            return (
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response(""),
            )
        }
        Err(error) => {
            return (
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                error_response(&error.to_string()),
            )
        }
    }
}

async fn setup_db(db: &Pool<Sqlite>) {
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS todo (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(db).await.unwrap();
    println!("Create user table result: {:?}", result);
}

fn error_response(error: &str) -> Markup {
    html!(
        div {
            (error)
        }
    )
}

fn ok_response(message: &str) -> Markup {
    html!(
        div {
            "ok " (message)
        }
    )
}

async fn head() -> Markup {
    html! {
        (DOCTYPE)
            html lang="en";
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                link rel="stylesheet" type="text/css" href="static/style.css";
                title { "todo"  }
                script type="module" src="static/script-twind.js" {};
                script src="https://unpkg.com/htmx.org@1.9.8" integrity="sha384-rgjA7mptc2ETQqXoYC3/zJvkU7K/aP44Y+z7xQuJiVnB/422P/Ak+F/AqFR7E4Wr" crossorigin="anonymous" {};
            }
            body
                ."flex justify-center flex-col items-center" {
                form
                    hx-post="/todos"
                    hx-target="#todosSubmitMessage" {
                    input placeholder="New todo" name="name";
                    button
                        ."border-black border max-w-xs" {
                        "Submit"
                    }
                }
                p ."text-center" {
                    "Todos:"
                }
                div hx-get="/todos" hx-trigger="load, updateTodos from:body" id="todos" {

                }
                p id="todosSubmitMessage" {

                }
            }
    }
}

async fn update_todo(
    Path(todo_id): Path<String>,
    State(db_pool): State<SqlitePool>,
    Form(new_todo): Form<NewTodo>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "UPDATE todo SET name=(?1) WHERE id=(?2)",
        new_todo.name,
        todo_id
    )
    .execute(&db_pool)
    .await;
    println!("{:?} {}", todo_id, new_todo.name);
    match result {
        Ok(_) => {
            return (
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response(""),
            )
        }
        Err(error) => {
            return (
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                error_response(&error.to_string()),
            )
        }
    }
}

fn todos_items(todos: Vec<Todo>) -> Markup {
    html!(
        @for todo in todos{
            ul {
                form hx-patch=(format!("/todos/{}", todo.id)) hx-target="#todosSubmitMessage" {
                    input value=(todo.name) name="name" ;
                    button
                        ."border-black border max-w-xs" {
                        "Change"
                    }
                }
            }
        }
    )
}
