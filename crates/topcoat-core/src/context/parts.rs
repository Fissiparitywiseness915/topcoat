use http::request::Parts;

use super::Cx;

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
