extern crate self as topcoat;

pub mod dev;
mod serve;

pub mod context {
    pub use topcoat_macro::memoize;

    pub use topcoat_core::context::*;
}

pub mod component {
    pub use topcoat_macro::component;

    pub trait Component {
        type Error;

        fn render(self) -> impl Future<Output = Result<crate::view::View, Self::Error>> + Send;
    }
}

pub mod router {
    pub use topcoat_macro::{layout, page, path_param, query_params, route, segment};

    pub use topcoat_router::*;
}

pub mod view {
    pub use topcoat_macro::view;

    pub use topcoat_view::runtime::*;
}

pub use serve::serve;

#[doc(hidden)]
pub mod internal {
    #[cfg(feature = "discover")]
    pub use inventory;
    pub use serde;
    pub use serde_urlencoded;

    pub trait ResultExt {
        type T;
        type E;
    }

    impl<T, E> ResultExt for Result<T, E> {
        type T = T;
        type E = E;
    }
}
