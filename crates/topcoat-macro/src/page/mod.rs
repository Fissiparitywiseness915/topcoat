use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    ItemFn, LitStr,
    parse::{Parse, ParseStream},
};

pub struct PageAttr {
    path: Option<LitStr>,
}

impl Parse for PageAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            path: input.peek(LitStr).then(|| input.parse()).transpose()?,
        })
    }
}

pub struct PageItem {
    item: ItemFn,
}

impl Parse for PageItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            item: input.parse()?,
        })
    }
}

pub struct Page(PageAttr, PageItem);

impl Page {
    pub fn new(page_attr: PageAttr, page_item: PageItem) -> Self {
        Self(page_attr, page_item)
    }
}

impl ToTokens for Page {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.0.path.as_ref();
        let item = &self.1.item;
        let ident = &item.sig.ident;

        let path = path.unwrap();

        quote! {
            struct #ident;

            impl ::topcoat::router::page::Page for #ident {
                fn path(&self) -> &str {
                    #path
                }

                fn render(&self) -> ::std::pin::Pin<Box<dyn Future<Output = View> + Send>> {
                    #item
                    Box::pin(#ident())
                }
            }
        }
        .to_tokens(tokens);
    }
}
