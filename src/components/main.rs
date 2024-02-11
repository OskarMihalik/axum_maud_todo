use maud::{html, Markup, DOCTYPE};

use crate::target_consts::TODOS_ID;

pub async fn head() -> Markup {
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
                div hx-get="/todos?page=0" hx-trigger="load, updateTodos from:body" id=(TODOS_ID) {

                }
                p id="todosSubmitMessage" {

                }
            }
    }
}
