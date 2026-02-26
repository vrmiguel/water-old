use std::io::Write;

use super::{Emittable, Emitter};
use crate::{ast::ArithmeticOperation, opcode::ToOpcode};

impl<W: Write> Emittable<ArithmeticOperation> for Emitter<W> {
    // Does not type check or see if there are enough operands
    // for the operation.
    //
    // This method may only be called within a larger `Emittable`
    // implementation that checks for this stuff.
    fn emit_element(&mut self, element: ArithmeticOperation) -> std::io::Result<usize> {
        let opcode = element.to_opcode();

        self.emit_byte(opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ArithmeticInstruction, NumericalType};

    #[test]
    fn test_emit_i32_add() {
        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Addition,
        };

        let mut emitter = Emitter::new(Vec::new());
        emitter.emit_element(op).unwrap();

        assert_eq!(emitter.into_inner(), vec![0x6a]);
    }

    #[test]
    fn test_emit_i32_sub() {
        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Subtraction,
        };

        let mut emitter = Emitter::new(Vec::new());
        emitter.emit_element(op).unwrap();

        assert_eq!(emitter.into_inner(), vec![0x6b]);
    }

    #[test]
    fn test_emit_i64_mul() {
        let op = ArithmeticOperation {
            type_: NumericalType::Int64,
            instr: ArithmeticInstruction::Multiplication,
        };

        let mut emitter = Emitter::new(Vec::new());
        emitter.emit_element(op).unwrap();

        assert_eq!(emitter.into_inner(), vec![0x7e]);
    }

    #[test]
    fn test_emit_f32_div() {
        let op = ArithmeticOperation {
            type_: NumericalType::Float32,
            instr: ArithmeticInstruction::FloatDivision,
        };

        let mut emitter = Emitter::new(Vec::new());
        emitter.emit_element(op).unwrap();

        assert_eq!(emitter.into_inner(), vec![0x95]);
    }

    #[test]
    fn test_emit_f64_add() {
        let op = ArithmeticOperation {
            type_: NumericalType::Float64,
            instr: ArithmeticInstruction::Addition,
        };

        let mut emitter = Emitter::new(Vec::new());
        emitter.emit_element(op).unwrap();

        assert_eq!(emitter.into_inner(), vec![0xa0]);
    }
}
