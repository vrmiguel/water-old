use std::io;

/// Trait for types that can emit WebAssembly bytecode elements
pub trait Emittable<T> {
    /// Emit `element` to WebAssembly.
    fn emit_element(&mut self, element: T) -> io::Result<usize>;
}
