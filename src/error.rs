use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub struct AppError(eyre::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("{}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An error has occured:\n{:?}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<eyre::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
