use topcoat_view::runtime::IntoViewParts;

use crate::runtime::IntoSurrogate;

#[allow(non_camel_case_types)]
pub struct f64(core::primitive::f64);

impl IntoSurrogate for core::primitive::f64 {
    type Surrogate = f64;

    fn into_surrogate(self) -> Self::Surrogate {
        f64(self)
    }
}

impl IntoViewParts for f64 {
    fn into_view_parts(self) -> impl Iterator<Item = topcoat_view::runtime::ViewPart> {
        self.0.into_view_parts()
    }
}
