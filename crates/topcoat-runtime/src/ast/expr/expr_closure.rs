use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Expr as SynExpr, ExprClosure};

use crate::ast::expr::Expr;

impl Expr {
    pub(super) fn expr_closure(
        closure: &ExprClosure,
        rust: &mut TokenStream,
        js: &mut String,
    ) -> syn::Result<()> {
        js.push('(');
        let mut inputs = Vec::with_capacity(closure.inputs.len());
        for (i, input) in closure.inputs.iter().enumerate() {
            if i > 0 {
                js.push_str(", ");
            }
            let mut tokens = TokenStream::new();
            Self::pat(input, &mut tokens, js)?;
            inputs.push(tokens);
        }
        js.push_str(") => ");

        let mut body = TokenStream::new();
        match &*closure.body {
            // A block body maps directly onto the arrow function body without
            // the IIFE wrapper that a block expression would need.
            SynExpr::Block(block) => Self::block(&block.block, &mut body, js)?,
            other => Self::dispatch(other, &mut body, js)?,
        }

        let capture = &closure.capture;
        let output = &closure.output;
        quote! { #capture |#(#inputs),*| #output #body }.to_tokens(rust);
        Ok(())
    }
}
