use axum::{routing::get, Json, Router};
use maud::{html, Markup, DOCTYPE};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json", get(json_handler))
        .route("/html", get(head))
        .route("/htmx/getSomething", get(item))
        .nest("/static", axum_static::static_router("static"));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3008".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Ass {
    ass: String,
}

async fn json_handler() -> Json<Ass> {
    let ass = Ass {
        ass: "asdfasfd".to_owned(),
    };
    Json(ass)
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
                hx-get="/htmx/getSomething"
                hx-target="#getHere"
                ."text-red-600" {
                button  {
                    "Click me please"
                }
                "asdfasf"
                div id="getHere" {

                }
            }
    }
}

async fn item() -> Markup {
    html!(
        li {
            "fffffffffffff"
        }
    )
}
