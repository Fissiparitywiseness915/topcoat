mod param;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    LitStr, Token, parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
};
use topcoat_view::ast::ParseOption;

use crate::quote_option::QuoteOption;
use param::Param;

pub struct Segment {
    variant: SegmentVariant,
    file: String,
}

impl Segment {
    /// Returns the rust module name for the file the macro was invoked from.
    fn module(&self) -> &str {
        let file_or_folder = self
            .file
            .split(&['/', '\\'])
            .rev()
            .find(|v| *v != "mod.rs")
            .expect("failed to extract module name from rust source file path");
        file_or_folder.strip_suffix(".rs").unwrap_or(file_or_folder)
    }
}

impl Parse for Segment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant = input.parse()?;
        if !input.is_empty() {
            return Err(input.error("unexpected trailing tokens"));
        }
        Ok(Self {
            variant,
            file: input.span().file(),
        })
    }
}

impl ToTokens for Segment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if cfg!(feature = "discover") {
            let kind_ident = self.variant.kind_ident();
            let rename = QuoteOption::new(self.variant.rename_lit().map(|lit| {
                quote! { ::std::borrow::Cow::Borrowed(#lit) }
            }));

            quote! {
                ::topcoat::inventory::submit! {
                    ::topcoat::router::Segment::new(
                        file!(),
                        ::core::option::Option::Some(::topcoat::router::SegmentKind::#kind_ident),
                        #rename,
                    )
                }
            }
            .to_tokens(tokens);
        }

        if let SegmentVariant::Param(param) = &self.variant {
            param.emit_function(self.module(), Span::call_site(), tokens);
        }
    }
}

enum SegmentVariant {
    Group { _kw: kw::group },
    Static {
        _kw: Token![static],
        rename: Option<StaticRename>,
    },
    CatchAll { _kw: kw::catch_all },
    Param(Param),
}

impl SegmentVariant {
    fn kind_ident(&self) -> syn::Ident {
        let name = match self {
            Self::Group { .. } => "Group",
            Self::Static { .. } => "Static",
            Self::CatchAll { .. } => "CatchAll",
            Self::Param(_) => "Param",
        };
        syn::Ident::new(name, Span::call_site())
    }

    fn rename_lit(&self) -> Option<LitStr> {
        match self {
            Self::Static {
                rename: Some(r), ..
            } => Some(r.name.clone()),
            Self::Param(param) => param
                .explicit_url_name()
                .map(|ident| LitStr::new(&ident.to_string(), ident.span())),
            _ => None,
        }
    }
}

impl Parse for SegmentVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::group) {
            Ok(Self::Group {
                _kw: input.parse()?,
            })
        } else if lookahead.peek(Token![static]) {
            Ok(Self::Static {
                _kw: input.parse()?,
                rename: input.call(StaticRename::parse_option)?,
            })
        } else if lookahead.peek(kw::catch_all) {
            Ok(Self::CatchAll {
                _kw: input.parse()?,
            })
        } else if lookahead.peek(kw::param) {
            Ok(Self::Param(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

struct StaticRename {
    _paren: Paren,
    name: LitStr,
}

impl Parse for StaticRename {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            name: content.parse()?,
        })
    }
}

impl ParseOption for StaticRename {
    fn peek(input: ParseStream) -> bool {
        input.peek(Paren)
    }
}

pub(crate) mod kw {
    use syn::custom_keyword;

    custom_keyword!(group);
    custom_keyword!(catch_all);
    custom_keyword!(param);
}
