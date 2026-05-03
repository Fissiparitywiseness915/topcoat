use topcoat_core::context::{Cx, Memoized};

pub trait QueryParams: Sized {
    type Error: 'static;

    fn of(cx: &Cx) -> Memoized<'_, Result<Self, Self::Error>>;
}
