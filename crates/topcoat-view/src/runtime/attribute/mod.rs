mod attributes;
mod key;
mod value;

pub use attributes::*;
pub use key::*;
pub use value::*;

use crate::runtime::{Unescaped, ViewParts};

#[derive(Debug, Clone)]
pub struct Attribute<K, V> {
    key: K,
    value: V,
}

impl<K, V> Attribute<K, V> {
    #[inline]
    pub fn new(key: K, value: V) -> Self {
        Self { key, value }
    }
}

pub trait AttributeViewParts {
    fn into_view_parts(self, parts: &mut ViewParts);
}

impl<K, V> AttributeViewParts for Attribute<K, V>
where
    K: AttributeKeyViewParts,
    V: AttributeValueViewParts,
{
    #[inline]
    fn into_view_parts(self, parts: &mut ViewParts) {
        if self.value.attribute_present() {
            parts.push(Unescaped::new_unchecked(" "));
            self.key.into_view_parts(parts);
            parts.push(Unescaped::new_unchecked("=\""));
            self.value.into_view_parts(parts);
            parts.push(Unescaped::new_unchecked("\""));
        }
    }
}

impl<T> AttributeViewParts for Option<T>
where
    T: AttributeViewParts,
{
    #[inline]
    fn into_view_parts(self, parts: &mut ViewParts) {
        if let Some(value) = self {
            value.into_view_parts(parts);
        }
    }
}

impl<T> AttributeViewParts for Vec<T>
where
    T: AttributeViewParts,
{
    #[inline]
    fn into_view_parts(self, parts: &mut ViewParts) {
        for value in self {
            value.into_view_parts(parts);
        }
    }
}
