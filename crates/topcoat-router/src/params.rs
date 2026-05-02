use std::sync::Arc;

use axum::extract::RawPathParams;
use topcoat_core::context::{Cx, extensions};

/// This is an internal function, use direct path hooks instead.
#[inline]
#[must_use]
#[doc(hidden)]
pub fn raw_path_params(cx: &Cx) -> &RawPathParams {
    extensions(cx)
        .get::<Arc<RawPathParams>>()
        .expect("`RawPathParams` missing from request extensions")
}
