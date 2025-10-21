use std::io::Write;

use super::{Emittable, Emitter};
use crate::{ast::ComparisonOperation, opcode::ToOpcode};

impl<W: Write> Emittable<ComparisonOperation> for Emitter<W> {
    // Does not type check or see if there are enough operands
    // for the operation.
    //
    // This method may only be called within a larger `Emittable`
    // implementation that checks for this stuff.
    fn emit_element(
        &mut self,
        element: ComparisonOperation,
    ) -> std::io::Result<usize> {
        let opcode = element.to_opcode();

        self.emit_byte(opcode)
    }
}

#[cfg(test)]
mod tests {
    // TODO: tests for Emittable<ComparisonOperation>
}
