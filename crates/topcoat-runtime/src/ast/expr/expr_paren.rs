use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::ExprParen;

use crate::ast::expr::Expr;

impl Expr {
    pub(super) fn expr_paren(
        paren: &ExprParen,
        rust: &mut TokenStream,
        js: &mut String,
    ) -> syn::Result<()> {
        js.push('(');
        let mut nested = TokenStream::new();
        Self::dispatch(&paren.expr, &mut nested, js)?;
        quote! { (#nested) }.to_tokens(rust);
        js.push(')');
        Ok(())
    }
}
