mod block;
mod expr_binary;
mod expr_block;
mod expr_closure;
mod expr_field;
mod expr_index;
mod expr_lit;
mod expr_method_call;
mod expr_paren;
mod expr_path;
mod externals;
mod pat;
mod stmt;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

use crate::ast::expr::externals::Externals;

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

        // Identifiers referenced but not declared by the expression are
        // captured from the surrounding Rust scope. Their values are encoded
        // into the JavaScript source at runtime as `const` bindings, declared
        // ahead of the returned expression.
        let externals = Externals::collect(&self.inner);
        let js_externals = externals.iter().map(|ident| {
            let prefix = format!("const {ident} = ");
            quote! { __js += #prefix; ::topcoat::runtime::Interop::to_js(&#ident, &mut __js); __js += ";"; }
        });
        let rust_externals = externals.iter().map(|ident| {
            quote! { let #ident = ::topcoat::runtime::Interop::into_surrogate(#ident); }
        });

        let tail = format!(" return {js}; }})()");

        Ok(quote! {{
            let mut __js = String::new();
            __js += "(() => {";
            #(#js_externals)*
            #(#rust_externals)*
            let __rust = #rust;
            __js += #tail;
            ::topcoat::runtime::Expr::new(__rust, __js)
        }})
    }

    /// Lowers a single `syn::Expr` into a Rust value (`rust`) and the
    /// equivalent JavaScript source (`js`), recursing into sub-expressions.
    fn dispatch(expr: &syn::Expr, rust: &mut TokenStream, js: &mut String) -> syn::Result<()> {
        match expr {
            syn::Expr::Lit(inner) => Self::expr_lit(inner, rust, js)?,
            syn::Expr::Paren(inner) => Self::expr_paren(inner, rust, js)?,
            syn::Expr::Binary(inner) => Self::expr_binary(inner, rust, js)?,
            syn::Expr::MethodCall(inner) => Self::expr_method_call(inner, rust, js)?,
            syn::Expr::Field(inner) => Self::expr_field(inner, rust, js)?,
            syn::Expr::Index(inner) => Self::expr_index(inner, rust, js)?,
            syn::Expr::Block(inner) => Self::expr_block(inner, rust, js)?,
            syn::Expr::Closure(inner) => Self::expr_closure(inner, rust, js)?,
            syn::Expr::Path(inner) => Self::expr_path(inner, rust, js)?,
            other => return Err(syn::Error::new_spanned(other, "unsupported expression")),
        }
        Ok(())
    }
}
