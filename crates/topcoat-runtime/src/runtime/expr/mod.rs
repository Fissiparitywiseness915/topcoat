#[derive(Debug, Clone)]
pub struct Expr<T> {
    pub(crate) evaluated: T,
    pub(crate) js: &'static str,
}

impl<T> Expr<T> {
    #[inline]
    pub fn new(evaluated: T, js: &'static str) -> Self {
        Self { evaluated, js }
    }
}
