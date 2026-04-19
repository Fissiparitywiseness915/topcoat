mod layout;
mod page;
mod router;
mod segment;

pub use layout::*;
pub use page::*;
pub use router::*;
pub use segment::*;

#[cfg(feature = "discover")]
#[macro_export]
macro_rules! file_router {
    () => {
        ::topcoat::router::Router::from(::topcoat::router::FileRouter::new(file!()).discover())
    };
}
