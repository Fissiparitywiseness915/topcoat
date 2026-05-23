use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use super::Expr;

/// A `|p1, p2, ...| body` event-handler closure. The body must reduce to a
/// statement (`Output = ()`); the macro enforces this via the inner expression
/// nodes it accepts.
pub struct ExprClosure {
    params: Vec<Ident>,
    body: Box<Expr>,
}

impl ExprClosure {
    pub fn new(params: Vec<Ident>, body: Expr) -> Self {
        Self {
            params,
            body: Box::new(body),
        }
    }
}

impl ToTokens for ExprClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let params = self.params.iter().map(|i| i.to_string());
        let body = &self.body;
        quote! {
            ::topcoat::runtime::ExprClosure::new(&[#(#params),*], #body)
        }
        .to_tokens(tokens);
    }
}
