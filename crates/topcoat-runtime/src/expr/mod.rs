mod expr_deref;
mod expr_lit;
mod expr_signal_ref;
mod interpreter;

pub use expr_deref::*;
pub use expr_lit::*;
pub use expr_signal_ref::*;
pub use interpreter::*;

use serde::Serialize;

pub trait Expr: Serialize {
    type Output;

    fn eval(self, interp: &mut Interpreter) -> Self::Output;
}

pub trait IntoExpr {
    type Expr;

    fn into_expr(self) -> Self::Expr;
}
