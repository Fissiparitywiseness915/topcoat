use std::borrow::Cow;
use std::ops::Index;
use std::pin::Pin;

use topcoat_core::runtime::{context::Cx, error::Result};

use crate::runtime::{Body, Endpoint, Path, Response, Route, method_not_allowed, not_found};

/// The future returned by [`Layer::handle`] and [`Next::run`]: a boxed, `Send`
/// future borrowing the chain and the request context.
pub type LayerFuture<'a> = Pin<Box<dyn Future<Output = Result<Response>> + Send + 'a>>;

/// A request-processing layer that wraps the routes nested under its path,
/// similar to a tower middleware.
///
/// A layer wraps every matched route whose path begins with the layer's path
/// (the same prefix rule as layouts), so a layer at `/admin` wraps only routes
/// under `/admin`, while a layer at `/` wraps everything. Each layer receives a
/// mutable [`Cx`] and the request [`Body`], plus a [`Next`] representing the
/// rest of the chain. A layer typically inspects or modifies the context, calls
/// [`Next::run`] to invoke the inner layers and ultimately the route, then
/// inspects or modifies the [`Response`].
///
/// When several layers match a route they nest from least-specific (outermost)
/// to most-specific (innermost), like layouts.
///
/// Register layers with [`RouterBuilder::layer`](crate::runtime::RouterBuilder::layer).
///
/// # Examples
///
/// ```rust
/// use std::borrow::Cow;
/// use topcoat::context::Cx;
/// use topcoat::router::{Body, Layer, LayerFuture, Next, Path};
///
/// struct Timing;
///
/// impl Layer for Timing {
///     fn path(&self) -> &Path {
///         Path::new("/")
///     }
///
///     fn handle<'a>(&'a self, cx: &'a mut Cx, body: Body, next: Next<'a>) -> LayerFuture<'a> {
///         Box::pin(async move {
///             let start = std::time::Instant::now();
///             let response = next.run(cx, body).await?;
///             println!("handled in {:?}", start.elapsed());
///             Ok(response)
///         })
///     }
/// }
/// ```
pub trait Layer: Send + Sync + 'static {
    /// The URL path prefix whose routes this layer wraps.
    fn path(&self) -> &Path;

    /// Handles a request, calling `next` to continue down the chain.
    fn handle<'a>(&'a self, cx: &'a mut Cx, body: Body, next: Next<'a>) -> LayerFuture<'a>;
}

/// The handler function backing a [`LayerFn`].
pub type LayerHandlerFn = for<'a> fn(cx: &'a mut Cx, body: Body, next: Next<'a>) -> LayerFuture<'a>;

/// A [`Layer`] backed by a plain handler function.
///
/// Created either manually via `#[layer("/path")]` or by the module router
/// (which derives the path from the module tree). Registered into a
/// [`RouterBuilder`](crate::runtime::RouterBuilder) with
/// [`layer`](crate::runtime::RouterBuilder::layer).
#[derive(Debug, Clone)]
pub struct LayerFn {
    /// The URL path prefix whose routes this layer wraps.
    path: Cow<'static, Path>,
    /// The handler function that wraps the inner chain.
    handle: LayerHandlerFn,
}

impl LayerFn {
    /// Creates a new layer with an explicit path prefix and handler function.
    pub const fn new(path: Cow<'static, Path>, handle: LayerHandlerFn) -> Self {
        Self { path, handle }
    }
}

impl Layer for LayerFn {
    fn path(&self) -> &Path {
        &self.path
    }

    fn handle<'a>(&'a self, cx: &'a mut Cx, body: Body, next: Next<'a>) -> LayerFuture<'a> {
        (self.handle)(cx, body, next)
    }
}

#[cfg(feature = "discover")]
inventory::collect!(LayerFn);

/// The identifier of a [`Layer`] registered on a router.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LayerId(usize);

/// The layers registered on a router, in registration order, indexed by
/// [`LayerId`].
///
/// Layers are [`push`](Self::push)ed as the router is built, then only queried:
/// [`for_path`](Self::for_path) selects the layers wrapping a request path, and
/// indexing by [`LayerId`] resolves a selected id back to its layer.
#[derive(Default)]
pub(crate) struct Layers {
    layers: Vec<Box<dyn Layer>>,
}

impl Layers {
    /// Registers `layer`, returning the [`LayerId`] that now identifies it.
    pub(crate) fn push(&mut self, layer: Box<dyn Layer>) -> LayerId {
        let id = LayerId(self.layers.len());
        self.layers.push(layer);
        id
    }

    /// Selects the layers whose path is a prefix of `path`, ordered least- to
    /// most-specific so the outermost layer runs first. Among layers that share a
    /// path, the most recently registered runs first.
    pub(crate) fn for_path(&self, path: &Path) -> Vec<LayerId> {
        let mut ids: Vec<LayerId> = (0..self.layers.len())
            .map(LayerId)
            .filter(|&LayerId(i)| path.starts_with(self.layers[i].path()))
            .rev()
            .collect();
        ids.sort_by_key(|&LayerId(i)| self.layers[i].path().len());
        ids
    }
}

impl Index<LayerId> for Layers {
    type Output = dyn Layer;

    fn index(&self, LayerId(index): LayerId) -> &Self::Output {
        &*self.layers[index]
    }
}

/// What a [`Next`] chain runs once its layers are exhausted.
///
/// The layers wrapping a path are the same whether or not the request resolves
/// to a route, so 404 and 405 responses flow through them too: a layer sees a
/// matched route handler's result, or the not-found / method-not-allowed error,
/// uniformly as the `Result` returned by [`Next::run`].
#[derive(Clone, Copy)]
pub(crate) enum Terminal<'a> {
    /// A matched route handles the request.
    Route(&'a dyn Route),
    /// No route matched the path; the chain resolves to a not-found error.
    NotFound,
    /// The path matched but the method did not; the chain resolves to a
    /// method-not-allowed error listing the endpoint's supported methods.
    MethodNotAllowed(&'a Endpoint),
}

/// The continuation of a [`Layer`] chain: the remaining layers followed by the
/// chain's terminal handler.
///
/// Passed as the `next` argument to [`Layer::handle`]. Call [`run`](Self::run)
/// to invoke the next layer, or the terminal once the layers are exhausted.
pub struct Next<'a> {
    /// The router's full layer table, indexed by the ids in `indices`.
    layers: &'a Layers,
    /// The layers wrapping this request, as ids into `layers`, ordered from
    /// least- to most-specific so the outermost layer runs first.
    indices: &'a [LayerId],
    /// What runs once the layers are exhausted.
    terminal: Terminal<'a>,
}

impl<'a> Next<'a> {
    /// Creates a chain that runs `indices` (in order) into `layers`, then
    /// `terminal`.
    ///
    /// `indices` must be ordered from least- to most-specific (ascending path
    /// length), so the outermost layer runs first.
    pub(crate) fn new(
        layers: &'a Layers,
        indices: &'a [LayerId],
        terminal: Terminal<'a>,
    ) -> Self {
        Self {
            layers,
            indices,
            terminal,
        }
    }

    /// Runs the next layer in the chain, or the terminal handler once no layers
    /// remain.
    pub fn run(self, cx: &'a mut Cx, body: Body) -> LayerFuture<'a> {
        match self.indices.split_first() {
            Some((&id, rest)) => self.layers[id].handle(
                cx,
                body,
                Next {
                    indices: rest,
                    ..self
                },
            ),
            None => match self.terminal {
                Terminal::Route(route) => route.handle(cx, body),
                Terminal::NotFound => Box::pin(async move { Err(not_found().into()) }),
                Terminal::MethodNotAllowed(endpoint) => {
                    let error = method_not_allowed(endpoint.methods().cloned());
                    Box::pin(async move { Err(error.into()) })
                }
            },
        }
    }
}
