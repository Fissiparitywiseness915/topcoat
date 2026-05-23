use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

use super::Expr;

/// A `receiver.field` access. Emits an accessor closure alongside the name so
/// rustc resolves the field's type against the receiver's real type.
pub struct ExprField {
    receiver: Box<Expr>,
    name: Ident,
}

impl ExprField {
    pub fn new(receiver: Expr, name: Ident) -> Self {
        Self {
            receiver: Box::new(receiver),
            name,
        }
    }
}

impl ToTokens for ExprField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let receiver = &self.receiver;
        let name_str = self.name.to_string();
        let name_ident = &self.name;
        quote! {
            ::topcoat::runtime::ExprField::new(
                #receiver,
                #name_str,
                |__receiver| __receiver.#name_ident,
            )
        }
        .to_tokens(tokens);
    }
}
