use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
};

pub struct RouteAttr {
    path: Option<LitStr>,
}

impl Parse for RouteAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.peek(LitStr).then(|| input.parse()).transpose()?,
        })
    }
}

pub struct RouteItem {
    item: ItemFn,
}

impl Parse for RouteItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

impl ToTokens for RouteItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}
