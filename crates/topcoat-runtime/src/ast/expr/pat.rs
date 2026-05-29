use std::fmt::Write;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Pat;

use crate::ast::expr::Expr;

impl Expr {
    /// Lowers a binding pattern. The full pattern (including any type
    /// annotation) is emitted to Rust; JavaScript receives only the bound
    /// identifier. Only plain identifiers, optionally annotated with a type,
    /// are supported.
    pub(super) fn pat(pat: &Pat, rust: &mut TokenStream, js: &mut String) -> syn::Result<()> {
        let inner = match pat {
            Pat::Type(ty) => &*ty.pat,
            other => other,
        };

        match inner {
            Pat::Ident(ident) if ident.by_ref.is_none() && ident.subpat.is_none() => {
                write!(js, "{}", ident.ident).unwrap();
            }
            other => return Err(syn::Error::new_spanned(other, "unsupported pattern")),
        }

        pat.to_tokens(rust);
        Ok(())
    }
}
