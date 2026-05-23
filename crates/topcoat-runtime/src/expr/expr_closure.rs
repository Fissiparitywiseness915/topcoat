use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{Expr, Interpreter};

/// A handler closure `|p1, p2, ...| body`. The body is required to have unit
/// output — handler bodies are statements, not value expressions. Server-side
/// `eval` is unreachable.
pub struct ExprClosure<Body> {
    params: &'static [&'static str],
    body: Body,
}

impl<Body> ExprClosure<Body> {
    pub fn new(params: &'static [&'static str], body: Body) -> Self {
        Self { params, body }
    }
}

impl<Body> Expr for ExprClosure<Body>
where
    Body: Expr<Output = ()>,
{
    type Output = ();

    fn eval(self, _interpreter: &mut Interpreter) -> Self::Output {
        unreachable!("ExprClosure::eval called server-side; handlers do not run during SSR")
    }
}

impl<Body> Serialize for ExprClosure<Body>
where
    Body: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ExprClosure", 3)?;
        s.serialize_field("type", "Closure")?;
        s.serialize_field("params", &self.params)?;
        s.serialize_field("body", &self.body)?;
        s.end()
    }
}
