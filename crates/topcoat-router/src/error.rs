//! Error types returned from page, layout and route handlers.

use http::StatusCode;
use topcoat_core::error::Error;

use crate::{IntoResponse, Response, Result, fallback_error_into_response};

/// Turns a Result into a response.
///
/// The IntoResponse trait unfortunately cannot be implemented on [`Result`] because it would clash
/// with the axum implementation.
pub(crate) fn result_into_response<T: IntoResponse>(result: Result<T>) -> Response {
    match result {
        Ok(value) => value.into_response(),
        Err(error) => error_into_response(error),
    }
}

/// Turns an Error into a response.
///
/// The IntoResponse trait unfortunately cannot be implemented on [`Error`] because it would clash
/// with the axum implementation.
pub(crate) fn error_into_response(error: Error) -> Response {
    match fallback_error_into_response(error) {
        Ok(error) => error.into_response(),
        Err(error) => InternalServerError::from(error).into_response(),
    }
}

pub(crate) struct InternalServerError {
    _inner: Error,
}

impl From<Error> for InternalServerError {
    fn from(value: Error) -> Self {
        Self { _inner: value }
    }
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}
