use std::io::{self};

pub trait Emittable<T> {
    /// Emit `element` to WebAssembly.
    fn element_ausgeben(&mut self, element: T) -> io::Result<usize>;
}
