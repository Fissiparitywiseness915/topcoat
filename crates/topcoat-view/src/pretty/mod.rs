mod delim;
mod r#macro;
mod printer;
mod ring_buffer;
mod rust;
mod span;
mod text;
mod token;
mod trivia;
mod visitor;

pub use delim::*;
pub use r#macro::*;
pub use printer::*;
pub use ring_buffer::*;
pub use span::*;
pub use token::*;
pub use trivia::*;
pub use visitor::*;

use syn::parse::Parse;

/// Parses `source_text` as `T` and pretty-prints it back, preserving the
/// surrounding comments and whitespace from the source.
///
/// `initial_space` is the number of columns left on the first line; `initial_indent`
/// is the indentation level the formatter starts at (in [`INDENT`]-wide steps).
pub fn pretty_print_str<T>(
    source_text: &str,
    initial_space: isize,
    initial_indent: isize,
) -> syn::Result<String>
where
    T: Parse + PrettyPrint,
{
    let ast: T = syn::parse_str(source_text)?;
    let trivia = Lexer::new(source_text).collect::<Vec<_>>();

    let mut printer = Printer::new(&trivia, initial_space, initial_indent);
    ast.pretty_print(&mut printer);
    Ok(printer.eof())
}

/// Pretty-prints an already-parsed AST. Unlike [`pretty_print_str`], this
/// has no source text to consult, so trivia (comments, blank lines) is lost.
pub fn pretty_print_ast<T>(ast: &T) -> String
where
    T: PrettyPrint,
{
    let mut printer = Printer::new(&[], 0, 0);
    ast.pretty_print(&mut printer);
    printer.eof()
}

/// Implemented by anything that knows how to emit itself as formatted text
/// through a [`Printer`]. The printer takes care of line breaking and
/// indentation; implementors only describe the desired layout.
pub trait PrettyPrint {
    fn pretty_print(&self, printer: &mut Printer<'_>);
}

impl<T> PrettyPrint for Option<T>
where
    T: PrettyPrint,
{
    fn pretty_print(&self, printer: &mut Printer<'_>) {
        if let Some(inner) = self {
            inner.pretty_print(printer);
        }
    }
}

impl<T> PrettyPrint for [T]
where
    T: PrettyPrint,
{
    fn pretty_print(&self, printer: &mut Printer<'_>) {
        for item in self {
            item.pretty_print(printer);
        }
    }
}

impl<T> PrettyPrint for syn::punctuated::Punctuated<T, syn::Token![,]>
where
    T: PrettyPrint,
{
    fn pretty_print(&self, printer: &mut Printer<'_>) {
        for (index, item) in self.pairs().enumerate() {
            item.value().pretty_print(printer);
            if item.punct().is_some() {
                printer.scan_no_break_trivia();
            }
            if index == self.len() - 1 {
                printer.scan_text(",".into(), TextMode::Break);
                printer.advance_cursor(",");
            } else {
                item.punct().unwrap().pretty_print(printer);
                printer.scan_same_line_trivia();
                printer.scan_break();
                " ".pretty_print(printer);
                printer.scan_trivia(true, true);
            }
        }
    }
}
