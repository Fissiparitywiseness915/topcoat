mod _f64;

pub use _f64::*;

pub trait IntoSurrogate {
    type Surrogate;

    fn into_surrogate(self) -> Self::Surrogate;
}
