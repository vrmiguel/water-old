use std::io::Write;

use super::{Emittable, Emitter};
use crate::{ast::ArithmeticOperation, opcode::ToOpcode};

impl<W: Write> Emittable<ArithmeticOperation> for Emitter<W> {
    // Does not type check or see if there are enough operands
    // for the operation.
    //
    // This method may only be called within a larger `Emittable`
    // implementation that checks for this stuff.
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
    fn emits_i32_add_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Addition,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6a]);
    }

    #[test]
    fn emits_i64_add_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int64,
            instr: ArithmeticInstruction::Addition,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x7c]);
    }

    #[test]
    fn emits_f32_add_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Float32,
            instr: ArithmeticInstruction::Addition,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x92]);
    }

    #[test]
    fn emits_f64_add_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Float64,
            instr: ArithmeticInstruction::Addition,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0xa0]);
    }

    #[test]
    fn emits_i32_sub_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Subtraction,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6b]);
    }

    #[test]
    fn emits_i32_mul_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::Multiplication,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6c]);
    }

    #[test]
    fn emits_f32_div_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Float32,
            instr: ArithmeticInstruction::FloatDivision,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x95]);
    }

    #[test]
    fn emits_i32_div_s_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::SignedDivision,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6d]);
    }

    #[test]
    fn emits_i32_div_u_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::UnsignedDivision,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6e]);
    }

    #[test]
    fn emits_i32_rem_s_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::SignedRemainder,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x6f]);
    }

    #[test]
    fn emits_i32_rem_u_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 1]);

        let op = ArithmeticOperation {
            type_: NumericalType::Int32,
            instr: ArithmeticInstruction::UnsignedRemainder,
        };

        emitter.emit_element(op).unwrap();

        assert_eq!(&emitter.into_inner().into_inner(), &[0x70]);
    }
}
