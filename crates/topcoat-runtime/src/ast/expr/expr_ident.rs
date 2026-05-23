use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

/// A bare identifier expression. Compiles to a call to
/// [`IntoExpr::into_expr`](::topcoat::runtime::IntoExpr) on the referenced
/// binding, so the surrounding type system picks the correct `Expr` impl.
pub struct ExprIdent {
    ident: Ident,
}

impl ExprIdent {
    pub fn new(ident: Ident) -> Self {
        Self { ident }
    }
}

impl ToTokens for ExprIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        quote! {
            ::topcoat::runtime::IntoExpr::into_expr(#ident)
        }
        .to_tokens(tokens);
    }
}
