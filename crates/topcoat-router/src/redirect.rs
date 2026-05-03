use axum::response::Redirect;

use crate::Result;

pub fn redirect(uri: &str) -> RedirectError {
    RedirectError::new(Redirect::temporary(uri))
}

pub fn redirect_permanent(uri: &str) -> RedirectError {
    RedirectError::new(Redirect::permanent(uri))
}

#[derive(Debug)]
pub struct RedirectError {
    inner: axum::response::Redirect,
}

impl RedirectError {
    fn new(inner: axum::response::Redirect) -> Self {
        Self { inner }
    }
}

impl axum::response::IntoResponse for RedirectError {
    fn into_response(self) -> axum::response::Response {
        self.inner.into_response()
    }
}

pub trait RedirectExt {
    type T;

    fn ok_or_redirect(self, uri: &str) -> Result<Self::T>;
    fn ok_or_redirect_permanent(self, uri: &str) -> Result<Self::T>;
}

impl<T> RedirectExt for Option<T> {
    type T = T;

    fn ok_or_redirect(self, uri: &str) -> Result<Self::T> {
        match self {
            Some(value) => Ok(value),
            None => Err(redirect(uri).into()),
        }
    }

    fn ok_or_redirect_permanent(self, uri: &str) -> Result<Self::T> {
        match self {
            Some(value) => Ok(value),
            None => Err(redirect_permanent(uri).into()),
        }
    }
}

impl<T, E> RedirectExt for Result<T, E> {
    type T = T;

    fn ok_or_redirect(self, uri: &str) -> Result<Self::T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(redirect(uri).into()),
        }
    }

    fn ok_or_redirect_permanent(self, uri: &str) -> Result<Self::T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(redirect_permanent(uri).into()),
        }
    }
}
