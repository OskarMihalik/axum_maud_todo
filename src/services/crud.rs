use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};
use axum_htmx::HxResponseTrigger;
use maud::Markup;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::components::{
    response::{error_response, ok_response},
    todo::{todos_items, Todo},
};

#[derive(Deserialize)]
pub struct NewTodo {
    pub name: String,
}

pub async fn update_todo(
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

pub async fn delete_todo(
    Path(todo_id): Path<String>,
    State(db_pool): State<SqlitePool>,
) -> impl IntoResponse {
    let result = sqlx::query!("DELETE FROM todo WHERE id=(?1)", todo_id)
        .execute(&db_pool)
        .await;
    println!("{:?}", todo_id);
    match result {
        Ok(_) => {
            return (
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response("deleted"),
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

pub async fn create_new_todo(
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

pub async fn get_todos(State(db_pool): State<SqlitePool>) -> Markup {
    let result = sqlx::query_as!(Todo, "SELECT id, name FROM todo")
        .fetch_all(&db_pool)
        .await;
    match result {
        Ok(todos) => {
            return todos_items(todos);
        }
        Err(error) => return error_response(&error.to_string()),
    }
}
