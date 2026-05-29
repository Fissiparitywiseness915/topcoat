use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::fmt::Write;
use syn::{ExprLit, Lit};

use crate::ast::expr::Expr;

impl Expr {
    pub(super) fn expr_lit(
        lit: &ExprLit,
        rust: &mut TokenStream,
        js: &mut String,
    ) -> syn::Result<()> {
        match &lit.lit {
            Lit::Float(inner) => {
                quote! { ::topcoat::runtime::IntoSurrogate::into_surrogate(#inner) }
                    .to_tokens(rust);
                write!(js, "{inner}").unwrap();
            }
            other => return Err(syn::Error::new_spanned(other, "unsupported literal type")),
        }
        Ok(())
    }
}
