mod expr_deref;
mod expr_ident;

pub use expr_deref::*;
pub use expr_ident::*;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    UnOp,
    parse::{Parse, ParseStream},
};

/// The top-level `expr! { ... }` AST. A whitelist of `syn::Expr` shapes is
/// translated into a tree of runtime expression nodes; anything outside that
/// whitelist is rejected at compile time.
pub enum Expr {
    Ident(ExprIdent),
    Deref(ExprDeref),
}

impl Expr {
    fn from_syn(expr: syn::Expr) -> syn::Result<Self> {
        match expr {
            syn::Expr::Path(path) => {
                let Some(ident) = path.path.get_ident() else {
                    return Err(syn::Error::new_spanned(
                        path,
                        "expected a bare identifier",
                    ));
                };
                Ok(Self::Ident(ExprIdent::new(ident.clone())))
            }
            syn::Expr::Unary(unary) => {
                let UnOp::Deref(_) = unary.op else {
                    return Err(syn::Error::new_spanned(
                        unary.op,
                        "unsupported unary operator",
                    ));
                };
                let inner = Self::from_syn(*unary.expr)?;
                Ok(Self::Deref(ExprDeref::new(inner)))
            }
            other => Err(syn::Error::new_spanned(other, "unsupported expression")),
        }
    }
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Self::from_syn(input.parse()?)
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(node) => node.to_tokens(tokens),
            Self::Deref(node) => node.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Expr {
        syn::parse_str(source).unwrap()
    }

    fn parse_err(source: &str) -> String {
        match syn::parse_str::<Expr>(source) {
            Ok(_) => panic!("expected parse error for `{source}`"),
            Err(err) => err.to_string(),
        }
    }

    #[test]
    fn parses_bare_identifier() {
        assert!(matches!(parse("signal"), Expr::Ident(_)));
    }

    #[test]
    fn parses_deref_of_identifier() {
        assert!(matches!(parse("*signal"), Expr::Deref(_)));
    }

    #[test]
    fn parses_nested_deref() {
        let Expr::Deref(_) = parse("**signal") else {
            panic!("expected deref")
        };
    }

    #[test]
    fn literal_is_rejected() {
        assert!(parse_err("42").contains("unsupported expression"));
    }

    #[test]
    fn binary_op_is_rejected() {
        assert!(parse_err("a + b").contains("unsupported expression"));
    }

    #[test]
    fn path_with_segments_is_rejected() {
        assert!(parse_err("foo::bar").contains("bare identifier"));
    }

    #[test]
    fn non_deref_unary_is_rejected() {
        assert!(parse_err("-x").contains("unary"));
    }
}
