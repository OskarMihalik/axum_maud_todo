use maud::{html, Markup};

use crate::{
    cornucopia::queries::todo::SelectTodos,
    target_consts::{TODOS_ID, TODOS_TARGET},
};

pub fn todos_items(todos: Vec<SelectTodos>, page: i64) -> Markup {
    html!(
        @for todo in &todos {
            ul ."flex gap-1"{
                form hx-patch=(format!("/todos/{}", todo.id)) hx-target="#todosSubmitMessage" {
                    input value=(todo.name) name="name" ;
                    button
                        ."border-black border max-w-xs" {
                        "Change"
                    }
                }
                button
                    hx-delete=(format!("/todos/{}", todo.id)) hx-target="#todosSubmitMessage"
                    ."border-black border max-w-xs" {
                        "Delete"
                }
            }
        }
        div ."flex flex-row justify-between"{
            @if page > 0 {
                button hx-get=(format!("/todos?page={}", page-1)) hx-target=(TODOS_TARGET){
                    "Previous"
                }

            }
            @if todos.len() != 0 {
                button hx-get=(format!("/todos?page={}", page+1)) hx-target=(TODOS_TARGET){
                    "Next"
                }
            }
        }
    )
}
