use proc_macro::TokenStream;
use quote::quote;
use topcoat_view::parse::View;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let parsed = syn::parse_macro_input!(input as View);
    quote! { parsed }.into()
}
