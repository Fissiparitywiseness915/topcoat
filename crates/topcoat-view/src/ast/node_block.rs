use syn::{
    braced,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{
    ast::{Node, parse_option::ParseOption},
    output::ViewWriter,
};

pub struct NodeBlock {
    _brace: Brace,
    children: Vec<Node>,
}

impl NodeBlock {
    pub fn write(&self, writer: &mut ViewWriter) {
        for child in &self.children {
            child.write(writer);
        }
    }
}

impl Parse for NodeBlock {
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

impl ParseOption for NodeBlock {
    fn peek(input: ParseStream) -> bool {
        input.peek(Brace)
    }
}
