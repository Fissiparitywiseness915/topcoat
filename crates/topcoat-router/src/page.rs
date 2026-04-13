use std::pin::Pin;

use topcoat_view::runtime::View;

pub trait Page: Sync {
    fn path(&self) -> &str;
    fn render(&self) -> Pin<Box<dyn Future<Output = View> + Send>>;
}
