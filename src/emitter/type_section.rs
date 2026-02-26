use std::io::{self, Write};

use super::{Emittable, Emitter};
use crate::{
    ast::FuncType,
    leb128::UnsignedLeb128,
    opcode::ToOpcode,
};

/// The byte that identifies a function type constructor in the
/// WebAssembly binary format.
const FUNC_TYPE_TAG: u8 = 0x60;

/// The section identifier for the Type Section.
const TYPE_SECTION_ID: u8 = 0x01;

impl<W: Write> Emittable<&FuncType> for Emitter<W> {
    /// Emits a single function type entry:
    /// `0x60 <param_count:u32> <param_types...> <result_count:u32> <result_types...>`
    fn emit_element(
        &mut self,
        func_type: &FuncType,
    ) -> io::Result<usize> {
        let mut bytes_written = 0;

        // Emit the function type constructor tag
        bytes_written += self.emit_byte(FUNC_TYPE_TAG)?;

        // Emit parameter count
        bytes_written += self.emit_element(
            UnsignedLeb128::from(func_type.params.len() as u64),
        )?;

        // Emit each parameter type
        for param in &func_type.params {
            bytes_written += self.emit_byte(param.to_opcode())?;
        }

        // Emit result count
        bytes_written += self.emit_element(
            UnsignedLeb128::from(func_type.results.len() as u64),
        )?;

        // Emit each result type
        for result in &func_type.results {
            bytes_written +=
                self.emit_byte(result.to_opcode())?;
        }

        Ok(bytes_written)
    }
}

impl<W: Write> Emittable<&[FuncType]> for Emitter<W> {
    /// Emits the complete Type Section (0x01):
    /// `0x01 <section_size:u32> <type_count:u32> <func_type>*`
    ///
    /// If the slice is empty, nothing is emitted (the section
    /// is omitted entirely).
    fn emit_element(
        &mut self,
        func_types: &[FuncType],
    ) -> io::Result<usize> {
        if func_types.is_empty() {
            return Ok(0);
        }

        // First, compute the section body into a temporary
        // buffer so we know the byte size for the section
        // header.
        let mut body_emitter = Emitter::new(Vec::new());

        // Emit the count of function types
        body_emitter.emit_element(UnsignedLeb128::from(
            func_types.len() as u64,
        ))?;

        // Emit each function type
        for func_type in func_types {
            body_emitter.emit_element(func_type)?;
        }

        let body = body_emitter.into_inner();

        let mut bytes_written = 0;

        // Emit section id
        bytes_written += self.emit_byte(TYPE_SECTION_ID)?;

        // Emit section size as LEB128
        bytes_written += self.emit_element(
            UnsignedLeb128::from(body.len() as u64),
        )?;

        // Emit the section body
        self.emit_bytes(&body)?;
        bytes_written += body.len();

        Ok(bytes_written)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{FuncType, NumericalType, Type},
        emitter::{Emittable, Emitter},
    };

    #[test]
    fn emits_empty_func_type() {
        // () -> ()
        let mut emitter = Emitter::new(Vec::new());
        let func_type = FuncType {
            params: vec![],
            results: vec![],
        };

        emitter.emit_element(&func_type).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x60, // func type tag
                0x00, // 0 params
                0x00, // 0 results
            ]
        );
    }

    #[test]
    fn emits_func_type_with_params_and_result() {
        // (i32, i32) -> i32
        let mut emitter = Emitter::new(Vec::new());
        let func_type = FuncType {
            params: vec![
                Type::Numerical(NumericalType::Int32),
                Type::Numerical(NumericalType::Int32),
            ],
            results: vec![Type::Numerical(NumericalType::Int32)],
        };

        emitter.emit_element(&func_type).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x60, // func type tag
                0x02, // 2 params
                0x7F, // i32
                0x7F, // i32
                0x01, // 1 result
                0x7F, // i32
            ]
        );
    }

    #[test]
    fn emits_func_type_with_all_numerical_types() {
        // (i32, i64, f32, f64) -> f64
        let mut emitter = Emitter::new(Vec::new());
        let func_type = FuncType {
            params: vec![
                Type::Numerical(NumericalType::Int32),
                Type::Numerical(NumericalType::Int64),
                Type::Numerical(NumericalType::Float32),
                Type::Numerical(NumericalType::Float64),
            ],
            results: vec![Type::Numerical(
                NumericalType::Float64,
            )],
        };

        emitter.emit_element(&func_type).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x60, // func type tag
                0x04, // 4 params
                0x7F, // i32
                0x7E, // i64
                0x7D, // f32
                0x7C, // f64
                0x01, // 1 result
                0x7C, // f64
            ]
        );
    }

    #[test]
    fn emits_empty_type_section() {
        let mut emitter = Emitter::new(Vec::new());
        let types: &[FuncType] = &[];

        emitter.emit_element(types).unwrap();

        // Empty type section should emit nothing
        assert!(emitter.into_inner().is_empty());
    }

    #[test]
    fn emits_type_section_with_single_type() {
        // Type section with one entry: (i32, i32) -> i32
        let mut emitter = Emitter::new(Vec::new());
        let types = vec![FuncType {
            params: vec![
                Type::Numerical(NumericalType::Int32),
                Type::Numerical(NumericalType::Int32),
            ],
            results: vec![Type::Numerical(NumericalType::Int32)],
        }];

        emitter.emit_element(types.as_slice()).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x01, // Type section id
                0x07, // section size: 7 bytes
                0x01, // 1 type entry
                0x60, // func type tag
                0x02, // 2 params
                0x7F, // i32
                0x7F, // i32
                0x01, // 1 result
                0x7F, // i32
            ]
        );
    }

    #[test]
    fn emits_type_section_with_multiple_types() {
        // Type section with two entries:
        //   (i32, i32) -> i32
        //   (f64) -> ()
        let mut emitter = Emitter::new(Vec::new());
        let types = vec![
            FuncType {
                params: vec![
                    Type::Numerical(NumericalType::Int32),
                    Type::Numerical(NumericalType::Int32),
                ],
                results: vec![Type::Numerical(
                    NumericalType::Int32,
                )],
            },
            FuncType {
                params: vec![Type::Numerical(
                    NumericalType::Float64,
                )],
                results: vec![],
            },
        ];

        emitter.emit_element(types.as_slice()).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x01, // Type section id
                0x0B, // section size: 11 bytes
                0x02, // 2 type entries
                // First type: (i32, i32) -> i32
                0x60, // func type tag
                0x02, // 2 params
                0x7F, // i32
                0x7F, // i32
                0x01, // 1 result
                0x7F, // i32
                // Second type: (f64) -> ()
                0x60, // func type tag
                0x01, // 1 param
                0x7C, // f64
                0x00, // 0 results
            ]
        );
    }

    #[test]
    fn emits_func_type_with_no_params_and_one_result() {
        // () -> i64
        let mut emitter = Emitter::new(Vec::new());
        let func_type = FuncType {
            params: vec![],
            results: vec![Type::Numerical(NumericalType::Int64)],
        };

        emitter.emit_element(&func_type).unwrap();

        assert_eq!(
            emitter.into_inner(),
            vec![
                0x60, // func type tag
                0x00, // 0 params
                0x01, // 1 result
                0x7E, // i64
            ]
        );
    }
}
