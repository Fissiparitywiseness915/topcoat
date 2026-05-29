mod expr_lit;
mod expr_paren;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

/// The top-level `expr! { ... }` AST. A thin wrapper around `syn::Expr`; the
/// whitelist of supported shapes is enforced when lowering to tokens.
pub struct Expr {
    inner: syn::Expr,
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            inner: input.parse()?,
        })
    }
}

impl Expr {
    pub fn expr_to_tokens(&self) -> syn::Result<TokenStream> {
        let mut rust = TokenStream::new();
        let mut js = String::new();
        Self::dispatch(&self.inner, &mut rust, &mut js)?;

        Ok(quote! { ::topcoat::runtime::Expr::new(#rust, #js) })
    }

    fn dispatch(expr: &syn::Expr, rust: &mut TokenStream, js: &mut String) -> syn::Result<()> {
        match expr {
            syn::Expr::Lit(inner) => Self::expr_lit(inner, rust, js)?,
            syn::Expr::Paren(inner) => Self::expr_paren(inner, rust, js)?,
            other => return Err(syn::Error::new_spanned(other, "unsupported expression")),
        }
        Ok(())
    }
}
