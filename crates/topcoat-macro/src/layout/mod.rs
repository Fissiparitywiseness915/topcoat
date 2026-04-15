use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
};

use crate::quote_option::QuoteOption;

pub struct LayoutAttr {
    path: Option<LitStr>,
}

impl Parse for LayoutAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.peek(LitStr).then(|| input.parse()).transpose()?,
        })
    }
}

pub struct LayoutItem {
    item: ItemFn,
}

impl Parse for LayoutItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub struct Layout(LayoutAttr, LayoutItem);

impl Layout {
    pub fn new(attr: LayoutAttr, item: LayoutItem) -> Self {
        Self(attr, item)
    }
}

impl ToTokens for Layout {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.0.path.as_ref();
        let item = &self.1.item;
        let ident = &item.sig.ident;

        let path = QuoteOption::from(path);

        quote! {
            #[allow(non_upper_case_globals)]
            const #ident: ::topcoat::router::Layout = ::topcoat::router::Layout::new(
                file!(),
                #path,
                |page| {
                    #item
                    Box::pin(#ident(page))
                }
            );
        }
        .to_tokens(tokens);

        if cfg!(feature = "discover") {
            quote! { ::topcoat::inventory::submit! { #ident } }.to_tokens(tokens);
        }
    }
}
