use maud::{html, Markup};

pub fn error_response(error: &str) -> Markup {
    html!(
        div {
            (error)
        }
    )
}

pub fn ok_response(message: &str) -> Markup {
    html!(
        div {
            "ok " (message)
        }
    )
}
