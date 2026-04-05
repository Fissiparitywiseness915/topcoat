use std::ops::Deref;

use syn::{
    Expr, Pat, Token,
    parse::{Parse, ParseStream},
    token::Brace,
};

use crate::{
    ast::{node::Node, parse_option::ParseOption},
    output::{ViewWriter, ViewWriterMatch},
};

pub struct NodeMatch {
    match_token: Token![match],
    expr: Box<Expr>,
    brace_token: Brace,
    arms: Vec<NodeMatchArm>,
}

impl NodeMatch {
    pub fn write(&self, writer: &mut ViewWriter) {
        let mut writer = writer.begin_match(&self.expr);
        for arm in &self.arms {
            arm.write(&mut writer);
        }
    }
}

impl Parse for NodeMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            match_token: input.parse()?,
            expr: Box::new(input.call(Expr::parse_without_eager_brace)?),
            brace_token: syn::braced!(content in input),
            arms: {
                let mut arms = Vec::new();
                while !content.is_empty() {
                    arms.push(content.parse()?);
                }
                arms
            },
        })
    }
}

impl ParseOption for NodeMatch {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![match])
    }
}

pub struct NodeMatchArm {
    pat: Pat,
    guard: Option<(Token![if], Box<Expr>)>,
    fat_arrow_token: Token![=>],
    body: Box<Node>,
    comma: Option<Token![,]>,
}

impl NodeMatchArm {
    pub fn write<'a>(&'a self, writer: &mut ViewWriterMatch<'a>) {
        let mut writer = writer.begin_arm(
            &self.pat,
            self.guard.as_ref().map(|(_, guard)| guard.deref()),
        );
        self.body.write(&mut writer);
    }
}

impl Parse for NodeMatchArm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            guard: {
                if input.peek(Token![if]) {
                    let if_token: Token![if] = input.parse()?;
                    let guard: Expr = input.parse()?;
                    Some((if_token, Box::new(guard)))
                } else {
                    None
                }
            },
            fat_arrow_token: input.parse()?,
            body: Box::new(input.parse()?),
            comma: {
                if !input.is_empty() {
                    Some(input.parse()?)
                } else {
                    input.parse()?
                }
            },
        })
    }
}
