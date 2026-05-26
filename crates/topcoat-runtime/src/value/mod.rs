mod str;
mod string;

pub use str::*;
pub use string::*;

pub trait Value {
    type Surrogate: ?Sized;

    fn ref_cast(&self) -> &Self::Surrogate;
}
