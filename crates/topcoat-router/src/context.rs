use axum::{body::Body, extract::Request};
use http::request::Parts;
use tokio::task_local;

#[derive(Debug)]
pub struct Cx {
    parts: Parts,
}

task_local! {
    static CX: Cx;
}

pub(crate) async fn scope_context<F: Future>(request: Request<Body>, f: F) -> F::Output {
    let (parts, _body) = request.into_parts();
    CX.scope(Cx { parts }, f).await
}

pub async fn with_context<F, R>(f: F) -> R
where
    F: FnOnce(&Cx) -> R,
{
    CX.with(f)
}

#[inline]
#[must_use]
pub fn parts(cx: &Cx) -> &Parts {
    &cx.parts
}

#[inline]
#[must_use]
pub fn method(cx: &Cx) -> &http::Method {
    &parts(cx).method
}

#[inline]
#[must_use]
pub fn uri(cx: &Cx) -> &http::Uri {
    &parts(cx).uri
}

#[inline]
#[must_use]
pub fn version(cx: &Cx) -> &http::Version {
    &parts(cx).version
}

#[inline]
#[must_use]
pub fn headers(cx: &Cx) -> &http::HeaderMap {
    &parts(cx).headers
}

#[inline]
#[must_use]
pub fn extensions(cx: &Cx) -> &http::Extensions {
    &parts(cx).extensions
}
