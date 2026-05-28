use std::borrow::Cow;

use crate::Expr;

pub struct ExprRaw {
    js: &'static [&'static str],
    slots: Vec<Box<dyn Expr>>,
}

impl ExprRaw {
    pub fn new(js: &'static [&'static str], slots: Vec<Box<dyn Expr>>) -> Self {
        Self { js }
    }
}
