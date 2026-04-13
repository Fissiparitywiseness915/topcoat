mod component;
mod page;
mod route;

use proc_macro::TokenStream;
use quote::quote;
use topcoat_view::ast::View;

use crate::page::Page;

#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(tokens as View);
    quote! { #parsed }.into()
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as component::ComponentAttr);
    let item = syn::parse_macro_input!(item as component::ComponentItem);
    quote! { #item }.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as route::RouteAttr);
    let item = syn::parse_macro_input!(item as route::RouteItem);
    quote! { #item }.into()
}

#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as page::PageAttr);
    let item = syn::parse_macro_input!(item as page::PageItem);
    let page = Page::new(attr, item);
    quote! { #page }.into()
}
