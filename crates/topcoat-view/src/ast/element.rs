use syn::{
    Ident, braced,
    parse::{Parse, ParseStream},
};

use crate::ast::{Attributes, Node, ParseOption, ViewWriter};

pub struct Element {
    name: Ident,
    attributes: Attributes,
    body: ElementBody,
}

impl Element {
    pub fn write(&self, writer: &mut ViewWriter) {
        writer.push_str("<");
        let name = self.name.to_string();
        writer.push_str(&name);
        self.attributes.write(writer);
        writer.push_str(">");

        self.body.write(writer);

        writer.push_str("</");
        writer.push_str(&name);
        writer.push_str(">");
    }
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
    children: Vec<Node>,
}

impl ElementBody {
    pub fn write(&self, writer: &mut ViewWriter) {
        for child in &self.children {
            child.write(writer);
        }
    }
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
