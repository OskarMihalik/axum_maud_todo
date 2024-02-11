use bb8::RunError;
use maud::Markup;
use tokio_postgres::Error;

use crate::components::response::error_response;

pub fn map_err_to_markup(error: RunError<Error>) -> Markup {
    error_response(&error.to_string())
}
