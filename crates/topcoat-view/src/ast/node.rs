use syn::{
    LitStr,
    parse::{Parse, ParseStream},
};

use crate::ast::{Element, ParseOption, ViewWriter};

pub enum Node {
    Text(LitStr),
    Element(Element),
}

impl Node {
    pub fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Text(inner) => writer.push_str(&inner.value()),
            Self::Element(inner) => inner.write(writer),
        }
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if Element::peek(input) {
            Ok(Self::Element(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected view node"))
        }
    }
}
