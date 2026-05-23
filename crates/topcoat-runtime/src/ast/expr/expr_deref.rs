use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use super::Expr;

/// A `*inner` expression. Compiles to a wrapping
/// [`ExprDeref`](::topcoat::runtime::ExprDeref) around the inner node so
/// dispatch through the [`ExprDerefTarget`](::topcoat::runtime::ExprDerefTarget)
/// trait decides what dereffing means for each runtime value.
pub struct ExprDeref {
    inner: Box<Expr>,
}

impl ExprDeref {
    pub fn new(inner: Expr) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
}

impl ToTokens for ExprDeref {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.inner;
        quote! {
            ::topcoat::runtime::ExprDeref::new(#inner)
        }
        .to_tokens(tokens);
    }
}
