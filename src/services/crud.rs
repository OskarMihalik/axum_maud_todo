use crate::cornucopia::queries::todo::delete_todo as delete_todo_q;
use crate::cornucopia::queries::todo::insert_todo;
use crate::cornucopia::queries::todo::select_todos;
use crate::cornucopia::queries::todo::update_todo as update_todo_q;
use crate::cornucopia::queries::todo::SelectTodosParams;
use crate::ConnectionPool;
use crate::{
    components::{
        response::{error_response, ok_response},
        todo::todos_items,
    },
    cornucopia::queries::todo::SelectTodosQuery,
};

use axum::{
    extract::{Path, Query, State},
    Form,
};

use axum_htmx::HxResponseTrigger;

use cornucopia_async::Params;
use maud::Markup;
use serde::Deserialize;
use tokio_postgres::GenericClient;

use super::utils::map_err_to_markup;

#[derive(Deserialize)]
pub struct NewTodo {
    pub name: String,
}

pub async fn update_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<ConnectionPool>,
    Form(new_todo): Form<NewTodo>,
) -> Result<(HxResponseTrigger, Markup), Markup> {
    let connection = pool.get().await.map_err(map_err_to_markup)?;

    let result = update_todo_q()
        .bind(connection.client(), &new_todo.name, &todo_id)
        .await;

    match result {
        Ok(_) => {
            return Ok((
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response(""),
            ))
        }
        Err(error) => return Err(error_response(&error.to_string())),
    }
}

pub async fn delete_todo(
    Path(todo_id): Path<i32>,
    State(pool): State<ConnectionPool>,
) -> Result<(HxResponseTrigger, Markup), Markup> {
    let connection = pool.get().await.map_err(map_err_to_markup)?;

    let result = delete_todo_q().bind(connection.client(), &todo_id).await;
    match result {
        Ok(_) => {
            return Ok((
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response("deleted"),
            ))
        }
        Err(error) => return Err(error_response(&error.to_string())),
    }
}

pub async fn create_new_todo(
    State(pool): State<ConnectionPool>,
    Form(new_todo): Form<NewTodo>,
) -> Result<(HxResponseTrigger, Markup), Markup> {
    let connection = pool.get().await.map_err(map_err_to_markup)?;

    let result = insert_todo()
        .bind(connection.client(), &new_todo.name)
        .await;

    match result {
        Ok(_) => {
            return Ok((
                HxResponseTrigger(vec!["updateTodos".to_string()]),
                ok_response(""),
            ))
        }
        Err(error) => return Err(error_response(&error.to_string())),
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    page: i64,
}

const PAGE_SIZE: i64 = 10;

pub async fn get_todos(
    pagination: Query<Pagination>,
    State(pool): State<ConnectionPool>,
) -> Result<Markup, Markup> {
    let connection = pool.get().await.map_err(map_err_to_markup)?;

    let result = select_todos()
        .params(
            connection.client(),
            &SelectTodosParams {
                limit: PAGE_SIZE,
                offset: pagination.page * PAGE_SIZE,
            },
        )
        .all()
        .await;
    match result {
        Ok(todos) => {
            return Ok(todos_items(todos, pagination.page));
        }
        Err(error) => return Err(error_response(&error.to_string())),
    }
}
