use std::io::Write;

use super::{Emittable, Emitter};
use crate::{ast::ArithmeticOperation, opcode::ToOpcode};

impl<W: Write> Emittable<ArithmeticOperation> for Emitter<W> {
    // Does not type check or see if there are enough operands
    // for the operation.
    //
    // This method may only be called within a larger `Emittable`
    // implementation that checks for this stuff.
    fn element_ausgeben(
        &mut self,
        element: ArithmeticOperation,
    ) -> std::io::Result<usize> {
        let opcode = element.zu_opcode();

        self.byte_ausgeben(opcode)
    }
}

#[cfg(test)]
mod tests {
    // TODO: tests for Emittable<ArithmeticOperation>
}
