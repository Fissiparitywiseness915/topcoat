mod attribute;
mod attribute_key;
mod attribute_node;
mod attribute_nodes;
mod attribute_value;
mod bind_attribute;
mod event_handler;

pub use attribute::*;
pub use attribute_key::*;
pub use attribute_node::*;
pub use attribute_nodes::*;
pub use attribute_value::*;
pub use bind_attribute::*;
pub use event_handler::*;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};

use crate::ast::{
    ParseOption,
    template::{TemplateElse, TemplateIf, TemplateMatch},
    view::{ViewWriter, WriteView},
};

/// The full list of attributes attached to a single tag.
pub struct Attributes {
    pub items: Vec<AttributeNode>,
}

impl Attributes {
    /// Returns `true` if `self` has no attributes.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl WriteView for Attributes {
    fn write(&self, writer: &mut ViewWriter) {
        for item in &self.items {
            item.write(writer);
        }
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let capacity = capacity_hint_for_nodes(&self.items);
        let statements = tokens_for_nodes(&self.items);

        quote! {{
            let mut __attrs = ::topcoat::view::Attributes::with_capacity(#capacity);
            #statements
            __attrs
        }}
        .to_tokens(tokens);
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while let Some(item) = input.call(AttributeNode::parse_option)? {
            items.push(item);
        }
        Ok(Self { items })
    }
}

#[cfg(feature = "pretty")]
impl topcoat_pretty::PrettyPrint for Attributes {
    fn pretty_print(&self, printer: &mut topcoat_pretty::Printer<'_>) {
        if self.items.is_empty() {
            return;
        }
        for item in &self.items {
            printer.scan_break();
            " ".pretty_print(printer);
            item.pretty_print(printer);
        }
    }
}

fn tokens_for_nodes(nodes: &[AttributeNode]) -> TokenStream {
    let statements = nodes.iter().map(tokens_for_node);
    quote! { #(#statements)* }
}

fn tokens_for_attribute_nodes(nodes: &AttributeNodes) -> TokenStream {
    tokens_for_nodes(nodes)
}

fn tokens_for_node(node: &AttributeNode) -> TokenStream {
    match node {
        AttributeNode::Attribute(inner) => {
            let key = &inner.key;
            let value = &inner.value;
            quote! {
                __attrs.insert(#key, #value);
            }
        }
        AttributeNode::BindAttribute(inner) => {
            let key = &inner.key;
            let value = &inner.value;
            quote! {
                {
                    let __key = ::std::convert::Into::<::std::string::String>::into(#key);
                    let (__evaluated, __js) = #value.into_evaluated_and_js();
                    __attrs.insert(__key.clone(), __evaluated);
                    __attrs.insert(::std::format!("data-topcoat-bind:{}", __key), __js);
                }
            }
        }
        AttributeNode::EventHandler(inner) => {
            let key = &inner.key;
            let value = &inner.value;
            quote! {
                {
                    let __key = ::std::convert::Into::<::std::string::String>::into(#key);
                    let (_, __js) = #value.into_evaluated_and_js();
                    __attrs.insert(::std::format!("data-topcoat-on:{}", __key), __js);
                }
            }
        }
        AttributeNode::If(inner) => tokens_for_template_if(inner),
        AttributeNode::Let(inner) => {
            let expr_let = &inner.expr_let;
            quote! { #expr_let; }
        }
        AttributeNode::ForLoop(inner) => {
            let pat = &inner.pat;
            let expr = &inner.expr;
            let body = tokens_for_attribute_nodes(&inner.body.children);
            quote! {
                for #pat in #expr {
                    #body
                }
            }
        }
        AttributeNode::Continue(inner) => {
            let expr_continue = &inner.expr_continue;
            quote! { #expr_continue; }
        }
        AttributeNode::Break(inner) => {
            let expr_break = &inner.expr_break;
            quote! { #expr_break; }
        }
        AttributeNode::Match(inner) => tokens_for_template_match(inner),
    }
}

fn tokens_for_template_if(template_if: &TemplateIf<AttributeNodes>) -> TokenStream {
    let cond = &template_if.cond;
    let then_branch = tokens_for_attribute_nodes(&template_if.then_branch.children);
    let else_branch = template_if
        .else_branch
        .as_ref()
        .map(tokens_for_template_else);

    quote! {
        if #cond {
            #then_branch
        }
        #else_branch
    }
}

fn tokens_for_template_else(template_else: &TemplateElse<AttributeNodes>) -> TokenStream {
    match template_else {
        TemplateElse::ElseIf { template_if, .. } => {
            let template_if = tokens_for_template_if(template_if);
            quote! { else #template_if }
        }
        TemplateElse::Else { then_branch, .. } => {
            let then_branch = tokens_for_attribute_nodes(&then_branch.children);
            quote! {
                else {
                    #then_branch
                }
            }
        }
    }
}

fn tokens_for_template_match(template_match: &TemplateMatch<AttributeNode>) -> TokenStream {
    let expr = &template_match.expr;
    let arms = template_match.arms.iter().map(|arm| {
        let pat = &arm.pat;
        let guard = arm.guard.as_ref().map(|(_, expr)| quote! { if #expr });
        let body = tokens_for_node(&arm.body);
        quote! {
            #pat #guard => {
                #body
            }
        }
    });

    quote! {
        match #expr {
            #(#arms,)*
        }
    }
}

fn capacity_hint_for_nodes(nodes: &[AttributeNode]) -> usize {
    nodes.iter().map(capacity_hint_for_node).sum()
}

fn capacity_hint_for_attribute_nodes(nodes: &AttributeNodes) -> usize {
    capacity_hint_for_nodes(nodes)
}

fn capacity_hint_for_node(node: &AttributeNode) -> usize {
    match node {
        AttributeNode::Attribute(_) => 1,
        AttributeNode::BindAttribute(_) => 2,
        AttributeNode::EventHandler(_) => 1,
        AttributeNode::Let(_) | AttributeNode::Continue(_) | AttributeNode::Break(_) => 0,
        AttributeNode::If(inner) => capacity_hint_for_template_if(inner),
        AttributeNode::ForLoop(inner) => capacity_hint_for_attribute_nodes(&inner.body.children),
        AttributeNode::Match(inner) => inner
            .arms
            .iter()
            .map(|arm| capacity_hint_for_node(&arm.body))
            .max()
            .unwrap_or_default(),
    }
}

fn capacity_hint_for_template_if(template_if: &TemplateIf<AttributeNodes>) -> usize {
    let then_capacity = capacity_hint_for_attribute_nodes(&template_if.then_branch.children);
    let else_capacity = template_if
        .else_branch
        .as_ref()
        .map(capacity_hint_for_template_else)
        .unwrap_or_default();
    then_capacity.max(else_capacity)
}

fn capacity_hint_for_template_else(template_else: &TemplateElse<AttributeNodes>) -> usize {
    match template_else {
        TemplateElse::ElseIf { template_if, .. } => capacity_hint_for_template_if(template_if),
        TemplateElse::Else { then_branch, .. } => {
            capacity_hint_for_attribute_nodes(&then_branch.children)
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    fn parse(source: &str) -> Attributes {
        syn::parse_str(source).unwrap()
    }

    #[test]
    fn tokens_construct_runtime_attributes_with_capacity() {
        let attrs = parse(r#"class="button" id=(id) :value=$(value) @input="handle()""#);

        assert_eq!(capacity_hint_for_nodes(&attrs.items), 5);

        let tokens = attrs.to_token_stream().to_string();
        assert!(tokens.contains(":: topcoat :: view :: Attributes :: with_capacity"));
        assert!(tokens.contains("__attrs . insert"));
        assert!(tokens.contains("\"class\""));
        assert!(tokens.contains("\"button\""));
        assert!(tokens.contains("\"id\""));
        assert!(tokens.contains("data-topcoat-bind:"));
        assert!(tokens.contains("data-topcoat-on:"));
        assert!(tokens.contains("into_evaluated_and_js"));
    }

    #[test]
    fn tokens_support_attribute_control_flow() {
        let attrs = parse(
            r#"
                let active = true;
                if active { class="active" } else { class="inactive" }
                for (key, value) in attrs {
                    if key == "skip" { continue; }
                    (key)=(value)
                    if key == "last" { break; }
                }
                match kind {
                    "button" => role="button",
                    _ => data-kind=(kind),
                }
            "#,
        );

        assert_eq!(capacity_hint_for_nodes(&attrs.items), 3);

        let tokens = attrs.to_token_stream().to_string();
        assert!(tokens.contains("let active = true"));
        assert!(tokens.contains("if active"));
        assert!(tokens.contains("else"));
        assert!(tokens.contains("for (key , value) in attrs"));
        assert!(tokens.contains("continue ;"));
        assert!(tokens.contains("break ;"));
        assert!(tokens.contains("match kind"));
        assert!(tokens.contains("__attrs . insert"));
    }
}
