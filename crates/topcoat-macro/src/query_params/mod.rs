use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    DeriveInput,
    parse::{Parse, ParseStream},
};

pub struct QueryParams {
    derive_input: DeriveInput,
}

impl Parse for QueryParams {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            derive_input: input.parse()?,
        })
    }
}

impl ToTokens for QueryParams {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let input = &self.derive_input;
        let ident = &input.ident;

        quote! {
            impl ::topcoat::router::QueryParams for #ident {
                type Error = ::topcoat::internal::serde_urlencoded::de::Error;

                fn of(cx: &::topcoat::context::Cx) -> topcoat::context::Memoized<'_, ::core::result::Result<Self, Self::Error>> {
                    #[::topcoat::context::memoize]
                    fn parse(cx: &::topcoat::context::Cx) -> Result<#ident,::topcoat::internal::serde_urlencoded::de::Error>  {
                        ::topcoat::internal::serde_urlencoded::from_str(
                            ::topcoat::context::uri(cx).path_and_query().map(|pq| pq.query().unwrap_or("")).unwrap_or("")
                        )
                    }
                    parse(cx)
                }
            }
        }
        .to_tokens(tokens);
    }
}
