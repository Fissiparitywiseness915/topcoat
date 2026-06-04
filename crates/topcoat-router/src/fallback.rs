//! Fallbacks modeled as errors.
//!
//! A handler can short-circuit by returning one of the fallback errors in
//! this module as the `Err` variant of its [`Result`]. [`redirect`],
//! [`redirect_permanent`], [`not_found`], [`unauthorized`], and
//! [`forbidden`] construct one directly, and [`FallbackExt`] lets `Option`
//! and `Result` values fall through to a fallback via the `?` operator.

use axum::response::Redirect;
use http::StatusCode;
use topcoat_core::error::Error;

use crate::{IntoResponse, Response, Result};
