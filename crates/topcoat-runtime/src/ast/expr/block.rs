use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Block;

use crate::ast::expr::Expr;

impl Expr {
    /// Lowers the contents of a `{ ... }` block. The trailing expression (a
    /// statement without a semicolon) becomes the block's value.
    pub(super) fn block(
        block: &Block,
        rust: &mut TokenStream,
        js: &mut String,
    ) -> syn::Result<()> {
        js.push_str("{ ");

        let mut stmts = TokenStream::new();
        let last = block.stmts.len().wrapping_sub(1);
        for (i, stmt) in block.stmts.iter().enumerate() {
            Self::stmt(stmt, &mut stmts, js, i == last)?;
        }

        js.push_str(" }");
        quote! { { #stmts } }.to_tokens(rust);
        Ok(())
    }
}
