use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Ident, Type};

/// A reference to a closure parameter. The macro produces this node instead of
/// [`ExprIdent`](super::ExprIdent) when the identifier is bound by an
/// enclosing closure. The annotated type from the closure's parameter list
/// flows in as a turbofish so field accesses against the param resolve
/// against the real type.
pub struct ExprParam {
    name: Ident,
    ty: Option<Type>,
}

impl ExprParam {
    pub fn new(name: Ident, ty: Option<Type>) -> Self {
        Self { name, ty }
    }
}

impl ToTokens for ExprParam {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name.to_string();
        match &self.ty {
            Some(ty) => quote! {
                ::topcoat::runtime::ExprParam::<#ty>::new(#name)
            }
            .to_tokens(tokens),
            None => quote! {
                ::topcoat::runtime::ExprParam::new(#name)
            }
            .to_tokens(tokens),
        }
    }
}
