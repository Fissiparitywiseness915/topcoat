#[cfg(feature = "discover")]
#[macro_export]
macro_rules! file_router {
    () => {
        ::topcoat::router::Router::new()
            .file_root(file!())
            .discover()
    };
}
