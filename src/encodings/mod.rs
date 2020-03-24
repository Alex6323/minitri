pub mod t1b1;
pub mod t3b1;
//mod t5b1;
//mod t9b2;

pub trait Encoding {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn len(&self) -> usize;
    fn add(&mut self, trits: t1b1::T1B1); // use bytes
}
