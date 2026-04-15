use std::pin::Pin;

use topcoat_view::runtime::View;

#[derive(Clone)]
pub struct Page {
    file: &'static str,
    path: &'static str,
    render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
}

impl Page {
    pub const fn new(
        file: &'static str,
        path: &'static str,
        render: fn() -> Pin<Box<dyn Future<Output = View> + Send>>,
    ) -> Self {
        Self { file, path, render }
    }

    pub fn file(&self) -> &'static str {
        self.file
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn render(&self) -> Pin<Box<dyn Future<Output = View> + Send>> {
        (self.render)()
    }
}

#[cfg(feature = "discover")]
inventory::collect!(Page);
