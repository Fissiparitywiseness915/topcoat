use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::Path;

/// A route handler that renders a [`View`] for a specific URL path.
///
/// Created either manually via `#[page("/path")]` or by the file router
/// (which derives the path from the module tree). Registered into a
/// [`Router`](crate::Router) alongside [`Layout`](crate::Layout)s.
#[derive(Clone)]
pub struct Page {
    /// The URL path this page handles.
    path: Cow<'static, Path>,
    /// The async render function that produces the page [`View`].
    render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl Page {
    /// Creates a new page with an explicit path and render function.
    pub const fn new(
        path: Cow<'static, Path>,
        render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { path, render }
    }

    /// Returns the URL path this page handles.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Renders the page, returning a [`View`].
    pub fn render(&self) -> Pin<Box<dyn Future<Output = View> + Send>> {
        (self.render)()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Page);
