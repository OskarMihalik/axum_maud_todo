use maud::{html, Markup};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Todo {
    pub id: i64,
    pub name: String,
}

pub fn todos_items(todos: Vec<Todo>) -> Markup {
    html!(
        @for todo in todos {
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
    )
}
