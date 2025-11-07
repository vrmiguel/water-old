use std::io::{self, Write};

use super::{emittable::Emittable, Emitter};
use crate::{ast::Constant, opcode::ToOpcode};

impl<W: Write> Emittable<Constant> for Emitter<W> {
    fn element_ausgeben(
        &mut self,
        element: Constant,
    ) -> io::Result<usize> {
        let opcode = element.value.zu_opcode();

        // Emit the `const` opcode for the given value
        self.byte_ausgeben(opcode)?;

        // .. and then the actual literal
        self.element_ausgeben(element.value)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{
        ast::{Constant, NumericalValue},
        emitter::{Emittable, Emitter},
    };

    #[test]
    fn emits_i32_const_correctly() {
        let mut emitter = Emitter::new_cursored([0_u8; 3]);

        let constant = Constant {
            value: NumericalValue::Int32(128),
        };

        // constant.emit_to(&mut buf.as_mut_slice()).unwrap();
        emitter.element_ausgeben(constant).unwrap();

        assert_eq!(
            &emitter.into_inner().into_inner(),
            &[
                // `i32.const`'s opcode
                0x41, // and the LEB128 for 128
                128, 1
            ]
        );
    }

    #[test]
    fn emits_i64_const_correctly() {
        let mut emitter = Emitter::new(Cursor::new([0_u8; 3]));

        let constant = Constant {
            value: NumericalValue::Int64(505),
        };

        emitter.element_ausgeben(constant).unwrap();
        assert_eq!(
            &emitter.into_inner().into_inner(),
            &[
                // `i64.const`'s opcode
                0x42, // and the LEB128 for 128
                249, 3
            ]
        );
    }

    #[test]
    fn emits_f32_const_correctly() {
        let mut emitter = Emitter::new(Cursor::new([0_u8; 5]));

        let constant = Constant {
            value: NumericalValue::Float32(5.0),
        };

        emitter.element_ausgeben(constant).unwrap();
        assert_eq!(
            &emitter.into_inner().into_inner(),
            &[
                // `f32.const`'s opcode
                0x43,
                // and then the LE bit pattern for 5.0
                0x00, 0x00, 0xa0, 0x40,
            ]
        );
    }

    #[test]
    fn emits_f64_const_correctly() {
        let mut emitter = Emitter::new(Cursor::new([0_u8; 9]));

        let constant = Constant {
            value: NumericalValue::Float64(25.50),
        };

        emitter.element_ausgeben(constant).unwrap();

        assert_eq!(
            &emitter.into_inner().into_inner(),
            &[
                // `f64.const`'s opcode
                0x44,
                // and then the LE bit pattern for 25.50
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x39, 0x40,
            ]
        );
    }
}
