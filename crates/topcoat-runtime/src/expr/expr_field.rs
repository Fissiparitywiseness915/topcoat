use std::marker::PhantomData;

use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{Expr, Interpreter};

/// A `receiver.field` access on a handler-internal value. The accessor closure
/// passed to `new` exists purely so rustc resolves `T` from the receiver's
/// real type — it is never invoked. Server-side `eval` is unreachable.
pub struct ExprField<R, T> {
    receiver: R,
    name: &'static str,
    _phantom: PhantomData<fn() -> T>,
}

impl<R, T> ExprField<R, T>
where
    R: Expr,
{
    pub fn new<F>(receiver: R, name: &'static str, _accessor: F) -> Self
    where
        F: FnOnce(R::Output) -> T,
    {
        Self {
            receiver,
            name,
            _phantom: PhantomData,
        }
    }
}

impl<R, T> Expr for ExprField<R, T>
where
    R: Expr,
{
    type Output = T;

    fn eval(self, _interpreter: &mut Interpreter) -> Self::Output {
        unreachable!("ExprField::eval called server-side; handler bodies do not run during SSR")
    }
}

impl<R, T> Serialize for ExprField<R, T>
where
    R: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ExprField", 3)?;
        s.serialize_field("type", "Field")?;
        s.serialize_field("receiver", &self.receiver)?;
        s.serialize_field("name", self.name)?;
        s.end()
    }
}
