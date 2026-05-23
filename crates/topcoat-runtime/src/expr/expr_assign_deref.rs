use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{Expr, ExprDerefAssignTarget, Interpreter};

/// Assignment through a dereferenced place: `*place = value`. The macro
/// recognises the `*X = Y` pattern and lowers it here rather than wrapping `X`
/// in an [`ExprDeref`](super::ExprDeref) — the latter is a read.
///
/// Type compatibility between place and value is enforced via the
/// [`ExprDerefAssignTarget`] bound, so `*signal = wrong_type` fails to
/// compile.
pub struct ExprAssignDeref<P, V> {
    place: P,
    value: V,
}

impl<P, V> ExprAssignDeref<P, V> {
    pub fn new(place: P, value: V) -> Self {
        Self { place, value }
    }
}

impl<P, V> Expr for ExprAssignDeref<P, V>
where
    P: Expr,
    P::Output: ExprDerefAssignTarget,
    V: Expr<Output = <P::Output as ExprDerefAssignTarget>::Value>,
{
    type Output = ();

    fn eval(self, _interpreter: &mut Interpreter) -> Self::Output {
        unreachable!(
            "ExprAssignDeref::eval called server-side; handler bodies do not run during SSR"
        )
    }
}

impl<P, V> Serialize for ExprAssignDeref<P, V>
where
    P: Serialize,
    V: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("ExprAssignDeref", 3)?;
        s.serialize_field("type", "AssignDeref")?;
        s.serialize_field("place", &self.place)?;
        s.serialize_field("value", &self.value)?;
        s.end()
    }
}
