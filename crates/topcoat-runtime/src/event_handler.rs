use topcoat_view::runtime::{IntoViewParts, Unescaped, ViewPart};

use crate::Expr;

/// An event handler attribute. Serializes the handler expression (always an
/// [`ExprClosure`](crate::ExprClosure)) into a `data-topcoat-on:<event>`
/// attribute on the element. The browser's scanner attaches a real
/// `addEventListener` that interprets the serialized body.
#[derive(Debug, Clone)]
pub struct EventHandler<K, V> {
    key: K,
    value: V,
}

impl<K, V> EventHandler<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> IntoViewParts for EventHandler<K, V>
where
    K: IntoViewParts,
    V: Expr<Output = ()>,
{
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        let serialized =
            serde_json::to_string(&self.value).expect("all expressions are serializable");

        Unescaped::new_unchecked(" data-topcoat-on:")
            .into_view_parts()
            .chain(self.key.into_view_parts())
            .chain(Unescaped::new_unchecked("=\"").into_view_parts())
            .chain(serialized.into_view_parts())
            .chain(Unescaped::new_unchecked("\" ").into_view_parts())
    }
}
