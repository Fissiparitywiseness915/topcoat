use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, LitStr, Path, Token, Visibility, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
};
use topcoat_view::ast::ParseOption;

use super::kw;

pub struct Param {
    _kw: kw::param,
    body: Option<ParamBody>,
}

impl Parse for Param {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _kw: input.parse()?,
            body: input.call(ParamBody::parse_option)?,
        })
    }
}

struct ParamBody {
    _paren: Paren,
    inner: ParamInner,
}

impl Parse for ParamBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren = parenthesized!(content in input);
        let inner = content.parse()?;
        Ok(Self {
            _paren: paren,
            inner,
        })
    }
}

impl ParseOption for ParamBody {
    fn peek(input: ParseStream) -> bool {
        input.peek(Paren)
    }
}

enum ParamInner {
    /// `param(<type>)`
    TypeOnly(Path),
    /// `param(<vis> <name>)`, optionally with `: <type>` and/or `as <fn_name>`.
    Named {
        vis: Visibility,
        name: Ident,
        ty: Option<ParamType>,
        fn_name: Option<ParamFnName>,
    },
}

impl Parse for ParamInner {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis: Visibility = input.parse()?;
        let is_named = !matches!(vis, Visibility::Inherited)
            || (input.peek(Ident) && input.peek2(Token![:]) && !input.peek3(Token![:]));
        if is_named {
            Ok(Self::Named {
                vis,
                name: input.parse()?,
                ty: input.call(ParamType::parse_option)?,
                fn_name: input.call(ParamFnName::parse_option)?,
            })
        } else {
            Ok(Self::TypeOnly(input.parse()?))
        }
    }
}

struct ParamType {
    _colon: Token![:],
    path: Path,
}

impl Parse for ParamType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _colon: input.parse()?,
            path: input.parse()?,
        })
    }
}

impl ParseOption for ParamType {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![:])
    }
}

struct ParamFnName {
    _as: Token![as],
    name: Ident,
}

impl Parse for ParamFnName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            _as: input.parse()?,
            name: input.parse()?,
        })
    }
}

impl ParseOption for ParamFnName {
    fn peek(input: ParseStream) -> bool {
        input.peek(Token![as])
    }
}

impl Param {
    /// The explicit URL parameter name, if one was given via `pub <name>: ...`.
    /// Returns `None` when the name should be inferred from the module name.
    pub fn explicit_url_name(&self) -> Option<&Ident> {
        match self.body.as_ref()?.inner {
            ParamInner::Named { ref name, .. } => Some(name),
            ParamInner::TypeOnly(_) => None,
        }
    }

    /// Emits the path-parameter accessor function.
    ///
    /// `module_name` is the rust module name extracted from `file!()`, used as
    /// the default for both the URL parameter name and the generated function
    /// name when neither is given explicitly.
    pub fn emit_function(&self, module_name: &str, span: Span, tokens: &mut TokenStream) {
        let module_ident = Ident::new(module_name, span);
        let module_lit = LitStr::new(module_name, span);

        let (vis, name_lit, fn_name, ty) = match self.body.as_ref() {
            None => (Visibility::Inherited, module_lit, module_ident, None),
            Some(ParamBody {
                inner: ParamInner::TypeOnly(path),
                ..
            }) => (Visibility::Inherited, module_lit, module_ident, Some(path)),
            Some(ParamBody {
                inner:
                    ParamInner::Named {
                        vis,
                        name,
                        ty,
                        fn_name,
                    },
                ..
            }) => {
                let name_lit = LitStr::new(&name.to_string(), name.span());
                let fn_name = fn_name
                    .as_ref()
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| name.clone());
                (vis.clone(), name_lit, fn_name, ty.as_ref().map(|t| &t.path))
            }
        };

        let panic = quote! {
            panic!("path parameter \"{}\" was not found in request path", #name_lit);
        };

        if let Some(ty) = ty {
            quote! {
                #[::topcoat::context::memoize]
                #vis fn #fn_name(cx: &::topcoat::context::Cx) -> #ty {
                    for (key, value) in ::topcoat::context::raw_path_params(cx) {
                        if key == #name_lit {
                            return str::parse::<#ty>(value).unwrap();
                        }
                    }
                    #panic
                }
            }
            .to_tokens(tokens);
        } else {
            quote! {
                #vis fn #fn_name(cx: &::topcoat::context::Cx) -> &str {
                    for (key, value) in ::topcoat::context::raw_path_params(cx) {
                        if key == #name_lit {
                            return value;
                        }
                    }
                    #panic
                }
            }
            .to_tokens(tokens);
        }
    }
}
