use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use super::Expr;

/// A `*place = value` assignment. The macro lowers this from
/// `syn::Expr::Assign` whose LHS is a deref; `place` is the inner expression
/// (a signal-like producing a deref target), not the deref itself.
pub struct ExprAssignDeref {
    place: Box<Expr>,
    value: Box<Expr>,
}

impl ExprAssignDeref {
    pub fn new(place: Expr, value: Expr) -> Self {
        Self {
            place: Box::new(place),
            value: Box::new(value),
        }
    }
}

impl ToTokens for ExprAssignDeref {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let place = &self.place;
        let value = &self.value;
        quote! {
            ::topcoat::runtime::ExprAssignDeref::new(#place, #value)
        }
        .to_tokens(tokens);
    }
}
