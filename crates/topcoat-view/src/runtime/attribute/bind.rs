use crate::runtime::{IntoViewParts, ViewPart};

#[derive(Debug, Clone)]
pub struct BindAttribute<K, V> {
    key: K,
    value: V,
}

impl<K, V> BindAttribute<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

impl<K, V> IntoViewParts for BindAttribute<K, V>
where
    K: IntoViewParts,
    V: IntoViewParts,
{
    fn into_view_parts(self) -> impl Iterator<Item = ViewPart> {
        " ".into_view_parts()
            .chain(self.key.into_view_parts())
            .chain("=\"".into_view_parts())
            .chain(self.value.into_view_parts())
            .chain("\"".into_view_parts())
    }
}
