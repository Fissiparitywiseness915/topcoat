// pub struct Router<S = ()> {
//     s: S,
// }
//
// impl<S> Into<axum::Router<S>> for Router<S> {
//     fn into(self) -> axum::Router<S> {}
// }

#[macro_export]
macro_rules! file_router {
    () => {};
}
