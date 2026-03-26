use crate::parse::{Span, Token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Root,
    Element,
    AttributeList,
    Attribute,
    Error,
}

#[derive(Debug, Clone)]
pub enum Child<'a> {
    Token(&'a Token<'a>),
    Node(Node<'a>),
}

impl<'a> Child<'a> {
    pub fn span(&self) -> Span {
        match self {
            Child::Token(token) => token.span(),
            Child::Node(node) => node.span,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node<'a> {
    kind: NodeKind,
    children: Vec<Child<'a>>,
    span: Span,
}

impl<'a> Node<'a> {
    pub fn kind(&self) -> NodeKind {
        self.kind
    }

    pub fn children(&self) -> &[Child<'a>] {
        &self.children
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Default)]
pub struct NodeBuilder<'a> {
    stack: Vec<(NodeKind, Vec<Child<'a>>)>,
    root: Option<Node<'a>>,
}

impl<'a> NodeBuilder<'a> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            root: None,
        }
    }

    pub fn begin(&mut self, kind: NodeKind) {
        self.stack.push((kind, Vec::new()));
    }

    pub fn token(&mut self, token: &'a Token<'a>) {
        self.stack
            .last_mut()
            .expect("called `token` on empty tree")
            .1
            .push(Child::Token(token));
    }

    pub fn end(&mut self) {
        let (kind, children) = self.stack.pop().expect("called `end` without `begin`");

        let span = match (children.first(), children.last()) {
            (Some(first), Some(last)) => Span::new(first.span().start(), last.span().end()),
            _ => Span::new(0, 0),
        };

        let node = Node {
            kind,
            children,
            span,
        };

        match self.stack.last_mut() {
            Some(parent) => parent.1.push(Child::Node(node)),
            None => self.root = Some(node),
        }
    }

    pub fn finish(&mut self) -> Node<'a> {
        self.root
            .take()
            .expect("called `finish` on an incomplete tree")
    }
}
