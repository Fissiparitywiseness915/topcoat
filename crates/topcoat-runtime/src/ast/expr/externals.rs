use std::collections::HashSet;

use proc_macro2::Ident;
use syn::visit::Visit;
use syn::{Block, ExprClosure, ExprPath, Local, Pat};

/// Collects the identifiers an expression references but does not itself
/// declare. Closure parameters and `let` bindings introduce local names; any
/// other identifier is captured from the surrounding Rust scope and recorded,
/// in order of first appearance.
#[derive(Default)]
pub(super) struct Externals {
    scopes: Vec<HashSet<String>>,
    external: Vec<Ident>,
}

impl Externals {
    pub(super) fn collect(expr: &syn::Expr) -> Vec<Ident> {
        let mut visitor = Self::default();
        visitor.visit_expr(expr);
        visitor.external
    }

    /// Declares the identifiers bound by `pat` in the innermost scope.
    fn bind(&mut self, pat: &Pat) {
        match pat {
            Pat::Ident(ident) => {
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(ident.ident.to_string());
                }
            }
            Pat::Type(ty) => self.bind(&ty.pat),
            _ => {}
        }
    }

    fn is_bound(&self, ident: &Ident) -> bool {
        let name = ident.to_string();
        self.scopes.iter().any(|scope| scope.contains(&name))
    }
}

impl<'ast> Visit<'ast> for Externals {
    fn visit_block(&mut self, block: &'ast Block) {
        self.scopes.push(HashSet::new());
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
        self.scopes.pop();
    }

    fn visit_local(&mut self, local: &'ast Local) {
        // The initializer is evaluated before the binding comes into scope, so
        // a `let x = x;` references the outer `x`.
        if let Some(init) = &local.init {
            self.visit_expr(&init.expr);
            if let Some((_, diverge)) = &init.diverge {
                self.visit_expr(diverge);
            }
        }
        self.bind(&local.pat);
    }

    fn visit_expr_closure(&mut self, closure: &'ast ExprClosure) {
        self.scopes.push(HashSet::new());
        for input in &closure.inputs {
            self.bind(input);
        }
        self.visit_expr(&closure.body);
        self.scopes.pop();
    }

    fn visit_expr_path(&mut self, path: &'ast ExprPath) {
        match path.path.get_ident() {
            Some(ident) if !self.is_bound(ident) => {
                if !self.external.iter().any(|i| i == ident) {
                    self.external.push(ident.clone());
                }
            }
            Some(_) => {}
            None => syn::visit::visit_expr_path(self, path),
        }
    }
}
