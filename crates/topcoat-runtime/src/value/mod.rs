pub mod string;

pub trait Value {
    type Surrogate;

    fn ref_cast(&self) -> &Self::Surrogate;
}
