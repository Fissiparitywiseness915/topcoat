use syn::{
    Ident, braced,
    parse::{Parse, ParseStream},
};

use crate::parse::{Attributes, ParseOption};

pub struct Element {
    name: Ident,
    attributes: Attributes,
    body: ElementBody,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            attributes: input.parse()?,
            body: input.parse()?,
        })
    }
}

impl ParseOption for Element {
    fn peek(input: ParseStream) -> bool {
        input.peek(Ident)
    }
}

pub struct ElementBody {
    _brace: syn::token::Brace,
    children: Vec<Element>,
}

impl Parse for ElementBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace: braced!(content in input),
            children: {
                let mut children = Vec::new();
                while !content.is_empty() {
                    children.push(content.parse()?)
                }
                children
            },
        })
    }
}
