use std::marker::PhantomData;

use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{Expr, Interpreter};

/// References a closure parameter by name. The user-annotated parameter type
/// flows in as `T`, so field accesses against this expression resolve against
/// the real type. Server-side `eval` is unreachable — handlers do not run
/// during SSR.
pub struct ExprParam<T> {
    name: &'static str,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> ExprParam<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            _phantom: PhantomData,
        }
    }
}

impl<T> Expr for ExprParam<T> {
    type Output = T;

    fn eval(self, _interpreter: &mut Interpreter) -> Self::Output {
        unreachable!("ExprParam::eval called server-side; handler bodies do not run during SSR")
    }
}

impl<T> Serialize for ExprParam<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ExprParam", 2)?;
        s.serialize_field("type", "Param")?;
        s.serialize_field("name", self.name)?;
        s.end()
    }
}
