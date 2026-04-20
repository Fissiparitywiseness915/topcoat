use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::Path;

/// A future that resolves to the inner page (or nested layout) [`View`].
///
/// Every layout render function receives a `Slot` and `.await`s it to embed
/// the child content at the desired location.
pub type Slot = Pin<Box<dyn Future<Output = View> + Send>>;

/// A layout that wraps pages whose path starts with the layout's path prefix.
///
/// When multiple layouts match a page, they nest from most-specific (innermost)
/// to least-specific (outermost). For example, layouts at `/` and `/settings`
/// both match `/settings/profile`, rendering as: root → settings → page.
#[derive(Clone)]
pub struct Layout {
    /// The path prefix this layout applies to.
    path: Cow<'static, Path>,
    /// The async render function that wraps child content via a [`Slot`].
    render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl Layout {
    /// Creates a new layout with an explicit path and render function.
    pub const fn new(
        path: Cow<'static, Path>,
        render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { path, render }
    }

    /// Returns the path prefix this layout applies to.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Renders the layout, embedding the given [`Slot`] as child content.
    pub fn render(&self, slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>> {
        (self.render)(slot)
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Layout);
