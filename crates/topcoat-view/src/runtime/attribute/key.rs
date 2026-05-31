use crate::runtime::{Unescaped, ViewParts};

pub trait AttributeKeyViewParts {
    fn into_view_parts(self, parts: &mut ViewParts);
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl AttributeKeyViewParts for $ty {
            #[inline]
            fn into_view_parts(self, parts: &mut ViewParts) {
                parts.push(self);
            }
        }
    };
}

impl_primitive!(&'static str);
impl_primitive!(String);
impl_primitive!(Unescaped<&'static str>);
impl_primitive!(Unescaped<String>);
