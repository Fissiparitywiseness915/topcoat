use syn::{
    LitStr,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{Element, NodeExpr, NodeForLoop, NodeIf, ParseOption},
    output::ViewWriter,
};

pub enum Node {
    Text(LitStr),
    Element(Element),
    Expr(NodeExpr),
    If(NodeIf),
    ForLoop(NodeForLoop),
}

impl Node {
    pub fn write(&self, writer: &mut ViewWriter) {
        match self {
            Self::Text(inner) => writer.push_escaped(&inner.value()),
            Self::Element(inner) => inner.write(writer),
            Self::Expr(inner) => inner.write(writer),
            Self::If(inner) => inner.write(writer),
            Self::ForLoop(inner) => inner.write(writer),
        }
    }
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Self::Text(input.parse()?))
        } else if Element::peek(input) {
            Ok(Self::Element(input.parse()?))
        } else if NodeExpr::peek(input) {
            Ok(Self::Expr(input.parse()?))
        } else if NodeIf::peek(input) {
            Ok(Self::If(input.parse()?))
        } else if NodeForLoop::peek(input) {
            Ok(Self::ForLoop(input.parse()?))
        } else {
            Err(syn::Error::new(input.span(), "expected view node"))
        }
    }
}
