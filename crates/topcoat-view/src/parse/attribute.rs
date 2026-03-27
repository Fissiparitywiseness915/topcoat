use syn::{
    Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

use crate::parse::ParseOption;

pub struct Attribute {
    name: Ident,
    _eq: Token![=],
    value: LitStr,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

impl ParseOption for Attribute {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident) && input.peek2(Token![=])
    }
}

pub struct Attributes {
    items: Vec<Attribute>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while let Some(attribute) = input.call(Attribute::parse_option)? {
            items.push(attribute);
        }
        Ok(Self { items })
    }
}
