use std::result;

use crate::cornucopia::queries::todo::delete_todo as delete_todo_q;
use crate::cornucopia::queries::todo::update_todo as update_todo_q;
use crate::ConnectionPool;
use crate::{
    components::{
        response::{error_response, ok_response},
        todo::{todos_items, Todo},
    },
    cornucopia::queries::todo::UpdateTodoParams,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Form,
};

use axum_htmx::HxResponseTrigger;
use axum_macros::debug_handler;
use deadpool_postgres::{Manager, Object};
use maud::Markup;
use serde::Deserialize;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio_postgres::GenericClient;
// }, cornucopia::queries::todo::update_todo};

#[derive(Deserialize)]
pub struct NewTodo {
    pub name: String,
}

// #[debug_handler]
pub async fn update_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<ConnectionPool>,
    Form(new_todo): Form<NewTodo>,
) -> impl IntoResponse {
    let client = pool.get().await.unwrap();
    let result = update_todo_q()
        .bind(client.client(), &new_todo.name, &todo_id)
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

// #[debug_handler]
// pub async fn delete_todo(
//     Path(todo_id): Path<i32>,
//     State(db_pool): State<Object>,
// ) -> impl IntoResponse {
//     let result = delete_todo_q().bind(&db_pool, &todo_id).await;
//     match result {
//         Ok(_) => {
//             return (
//                 HxResponseTrigger(vec!["updateTodos".to_string()]),
//                 ok_response("deleted"),
//             )
//         }
//         Err(error) => {
//             return (
//                 HxResponseTrigger(vec!["updateTodos".to_string()]),
//                 error_response(&error.to_string()),
//             )
//         }
//     }
// }

// pub async fn create_new_todo(
//     State(db_pool): State<Object>,
//     Form(new_todo): Form<NewTodo>,
// ) -> impl IntoResponse {
//     let result = sqlx::query!("INSERT INTO todo (name) values (?1)", new_todo.name)
//         .execute(&db_pool)
//         .await;
//     match result {
//         Ok(_) => {
//             return (
//                 HxResponseTrigger(vec!["updateTodos".to_string()]),
//                 ok_response(""),
//             )
//         }
//         Err(error) => {
//             return (
//                 HxResponseTrigger(vec!["updateTodos".to_string()]),
//                 error_response(&error.to_string()),
//             )
//         }
//     }
// }

// pub async fn get_todos(State(db_pool): State<Object>) -> Markup {
//     let result = sqlx::query_as!(Todo, "SELECT id, name FROM todo")
//         .fetch_all(&db_pool)
//         .await;
//     match result {
//         Ok(todos) => {
//             return todos_items(todos);
//         }
//         Err(error) => return error_response(&error.to_string()),
//     }
// }
