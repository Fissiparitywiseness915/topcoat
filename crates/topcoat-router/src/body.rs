use crate::Result;
use topcoat_core::context::Cx;

#[must_use]
#[derive(Debug)]
pub struct Body {
    inner: axum::body::Body,
}

impl Body {
    pub fn into_data_stream(self) -> axum::body::BodyDataStream {
        self.inner.into_data_stream()
    }
}

impl From<axum::body::Body> for Body {
    fn from(value: axum::body::Body) -> Self {
        Self { inner: value }
    }
}

pub trait FromBody: Sized {
    fn from_body(cx: &Cx, body: Body) -> impl Future<Output = Result<Self>> + Send;
}
