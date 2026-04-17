use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    ItemMod,
    parse::{Parse, ParseStream},
};

use crate::quote_option::QuoteOption;

pub struct SegmentAttr {}

impl Parse for SegmentAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {})
    }
}

pub struct SegmentItem {
    item: ItemMod,
}

impl Parse for SegmentItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub struct Segment(SegmentAttr, SegmentItem);

impl Segment {
    pub fn new(attr: SegmentAttr, item: SegmentItem) -> Self {
        Self(attr, item)
    }
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let item = &self.1.item;
        let module_ident = &item.ident.to_string();

        if cfg!(feature = "discover") {
            quote! {
                ::topcoat::inventory::submit! {
                    ::topcoat::router::Segment::new(
                        file!(),
                        #module_ident,
                        None,
                        None,
                    )
                }
            }
            .to_tokens(tokens);
        }
    }
}
