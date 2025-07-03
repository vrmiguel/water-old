use std::io::Write;

use super::{Emittable, Emitter};
use crate::{ast::ArithmeticOperation, opcode::ToOpcode};

impl<W: Write> Emittable<ArithmeticOperation> for Emitter<W> {
    /// Emits the WebAssembly opcode for the given arithmetic operation.
    ///
    /// This implementation assumes that the operation is valid for the given numerical
    /// type, as type checking is performed at the `to_opcode()` level. Invalid combinations
    /// will trigger a panic in `to_opcode()` when it encounters an impossible operation,
    /// such as `f32.div_s` (since float division doesn't have signed/unsigned variants).
    fn emit_element(
        &mut self,
        element: ArithmeticOperation,
    ) -> std::io::Result<usize> {
        let opcode = element.to_opcode();

        self.emit_byte(opcode)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{ArithmeticInstruction, ArithmeticOperation, NumericalType},
        emitter::{Emittable, Emitter},
    };

    #[test]
    fn emits_arithmetic_operation_opcode() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);
        
        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Addition,
        };
        
        emitter.emit_element(op).unwrap();
        
        // 0x6a is the opcode for i32.add
        assert_eq!(&emitter.into_inner().into_inner(), &[0x6a]);
    }
}
