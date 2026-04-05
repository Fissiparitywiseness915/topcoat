use syn::{
    Expr, Pat, Token,
    parse::{Parse, ParseStream},
};

use crate::{
    ast::{NodeBlock, parse_option::ParseOption},
    output::ViewWriter,
};

pub struct NodeForLoop {
    pub for_token: Token![for],
    pub pat: Box<Pat>,
    pub in_token: Token![in],
    pub expr: Box<Expr>,
    pub body: NodeBlock,
}

impl NodeForLoop {
    pub(crate) fn write(&self, writer: &mut ViewWriter) {
        let mut writer = writer.begin_for_loop(&self.pat, &self.expr);
        self.body.write(&mut writer);
    }
}

impl Parse for NodeForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            for_token: input.parse()?,
            pat: Box::new(input.call(Pat::parse_single)?),
            in_token: input.parse()?,
            expr: Box::new(input.call(Expr::parse_without_eager_brace)?),
            body: input.parse()?,
        })
    }
}

impl ParseOption for NodeForLoop {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![for])
    }
}
