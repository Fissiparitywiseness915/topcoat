use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::{Page, Path};

/// A page discovered by the file router, produced by the `#[page]` macro.
///
/// Holds the source file path (for deriving the URL path from the module tree)
/// and the render function. The file router converts each `FilePage` into a
/// [`Page`] once the URL path has been computed.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct FilePage {
    /// Source file where `#[page]` was declared, used to derive the URL path.
    file: &'static str,
    /// The page's async render function, returning a [`View`].
    pub(super) render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl FilePage {
    /// Creates a new file page. Called by the expanded `#[page]` macro.
    pub const fn new(
        file: &'static str,
        render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { file, render }
    }

    /// Converts into a [`Page`] with the given resolved URL path.
    pub fn into_page(self, path: Cow<'static, Path>) -> Page {
        Page::new(path, self.render)
    }

    /// Returns the source file path used to derive the URL.
    pub fn file(&self) -> &'static str {
        self.file
    }
}

#[cfg(feature = "discover")]
inventory::collect!(FilePage);
