use std::{fmt::Display, ops::Add};

pub mod meta {
    pub use library_meta::api_type;
}
pub use library_meta::imports;

pub struct Adder<T>
where
    T: Add + Display + Clone,
{
    _marker: std::marker::PhantomData<T>,
}

impl<T> Adder<T>
where
    T: Add<Output = T> + Display + Clone,
{
    pub fn print_add(a: T, b: T) {
        println!("{} + {} = {}", a.clone(), b.clone(), a + b);
    }
}
