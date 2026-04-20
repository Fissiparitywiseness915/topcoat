use std::{borrow::Cow, pin::Pin};

use topcoat_view::runtime::View;

use crate::{Layout, Path, Slot};

/// A layout discovered by the file router, produced by the `#[layout]` macro.
///
/// Holds the source file path (for deriving the URL prefix from the module
/// tree) and the render function. The file router converts each `FileLayout`
/// into a [`Layout`] once the URL path has been computed.
#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct FileLayout {
    /// Source file where `#[layout]` was declared, used to derive the URL path.
    file: &'static str,
    /// The layout's async render function, receiving a [`Slot`] and returning a [`View`].
    render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl FileLayout {
    /// Creates a new file layout. Called by the expanded `#[layout]` macro.
    pub const fn new(
        file: &'static str,
        render: fn(slot: Slot) -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { file, render }
    }

    /// Converts into a [`Layout`] with the given resolved URL path.
    pub fn into_layout(self, path: Cow<'static, Path>) -> Layout {
        Layout::new(path, self.render)
    }

    /// Returns the source file path used to derive the URL.
    pub fn file(&self) -> &'static str {
        self.file
    }
}

#[cfg(feature = "discover")]
inventory::collect!(FileLayout);
