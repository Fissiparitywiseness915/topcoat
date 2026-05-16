use std::{
    collections::HashSet,
    pin::Pin,
    task::{Context, Poll},
};

use http::{HeaderValue, Request, Response, Uri, header, uri::PathAndQuery};
use tower_http::services::{ServeDir, fs::DefaultServeDirFallback};
use tower_service::Service;

use crate::AssetBundle;

/// `tower` service that serves the files in an [`AssetBundle`] over HTTP.
///
/// Only filenames present in the bundle are served; any other path
/// receives a 404 (or is forwarded to the configured fallback, if any).
#[derive(Clone, Debug)]
pub struct ServeAssetBundle<F = DefaultServeDirFallback> {
    inner: ServeDir<F>,
    files: HashSet<String>,
}

impl ServeAssetBundle {
    /// Build a service that serves `bundle`.
    pub fn new(bundle: &AssetBundle) -> Self {
        let files = bundle
            .assets()
            .filter_map(|asset| asset.path().file_name()?.to_str().map(String::from))
            .collect();
        Self {
            inner: ServeDir::new(bundle.dir()),
            files,
        }
    }
}

impl<F> ServeAssetBundle<F> {
    /// Set a fallback service to handle requests for paths that aren't
    /// part of the bundle.
    pub fn fallback<F2>(self, fallback: F2) -> ServeAssetBundle<F2> {
        ServeAssetBundle {
            inner: self.inner.fallback(fallback),
            files: self.files,
        }
    }
}

trait MaybeAddCacheHeader {
    fn maybe_add_cache_header(&mut self, is_asset: bool);
}

impl<B> MaybeAddCacheHeader for Response<B> {
    fn maybe_add_cache_header(&mut self, is_asset: bool) {
        if is_asset && self.status().is_success() {
            self.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            );
        }
    }
}

pin_project_lite::pin_project! {
    pub struct ServeAssetFuture<F> {
        #[pin]
        inner: F,
        is_asset: bool,
    }
}

impl<F, R, E> std::future::Future for ServeAssetFuture<F>
where
    F: std::future::Future<Output = Result<R, E>>,
    R: MaybeAddCacheHeader,
{
    type Output = Result<R, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let is_asset = *this.is_asset;

        match this.inner.poll(cx) {
            Poll::Ready(Ok(mut res)) => {
                res.maybe_add_cache_header(is_asset);
                Poll::Ready(Ok(res))
            }
            other => other,
        }
    }
}

impl<B, F> Service<Request<B>> for ServeAssetBundle<F>
where
    ServeDir<F>: Service<Request<B>>,
    <ServeDir<F> as Service<Request<B>>>::Response: MaybeAddCacheHeader,
{
    type Response = <ServeDir<F> as Service<Request<B>>>::Response;
    type Error = <ServeDir<F> as Service<Request<B>>>::Error;
    type Future = ServeAssetFuture<<ServeDir<F> as Service<Request<B>>>::Future>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let path = req.uri().path().trim_start_matches('/');
        let is_asset = self.files.contains(path);

        if !is_asset {
            let mut parts = req.uri().clone().into_parts();
            parts.path_and_query = Some(PathAndQuery::from_static("/.."));
            *req.uri_mut() = Uri::from_parts(parts).unwrap();
        }

        ServeAssetFuture {
            inner: self.inner.call(req),
            is_asset,
        }
    }
}
