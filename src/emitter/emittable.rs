use std::io;

pub trait Emittable<T> {
    /// Emit `element` to WebAssembly.
    fn emit_element(&mut self, element: T) -> io::Result<usize>;
}
