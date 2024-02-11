# TODO web aplication

this is a proof of concept doing todo aplication with these technologies:

- for server i used Axum: https://github.com/tokio-rs/axum
- for server side type safe templates i used https://maud.lambda.xyz/
- for interactivity and loading templates on frontend i used HTMX https://htmx.org/
- for database i used posgres
- for quering and writing sql i used cornucopia https://github.com/cornucopia-rs/cornucopia

## to run development:

1. run docker with: `docker-compose up`
2. run axum server: `cargo watch -x run` or just `cargo run`
