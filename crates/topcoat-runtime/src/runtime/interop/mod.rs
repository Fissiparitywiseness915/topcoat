mod _f64;

pub use _f64::*;

pub trait Interop {
    type Surrogate;

    fn to_js(&self, out: &mut String);
    fn into_surrogate(self) -> Self::Surrogate;
}
