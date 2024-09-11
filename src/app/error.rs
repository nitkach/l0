use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub(crate) struct AppError {
    code: StatusCode,
    source: anyhow::Error,
}

impl AppError {
    pub(crate) fn new(status_code: StatusCode, source: anyhow::Error) -> Self {
        Self {
            code: status_code,
            source,
        }
    }

    pub(crate) fn with_status_404(source: anyhow::Error) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            source,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.source.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            source: err.into(),
        }
    }
}
