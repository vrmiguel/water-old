use std::io::{self, Write};

mod arithmetic_operation;
mod constant;
pub mod emittable;
mod numerical_value;
mod unreachable;

pub use emittable::Emittable;

use crate::ast::{Module, NumericalType, Opcode, Program, Type};
use crate::leb128::UnsignedLeb128;
use crate::opcode::ToOpcode;

const MAGIC: &[u8] = b"\0asm";
const VERSION: &[u8] = &[0x01, 0x00, 0x00, 0x00];

const SECTION_TYPE: u8 = 0x01;
const SECTION_FUNCTION: u8 = 0x03;
const SECTION_EXPORT: u8 = 0x07;
const SECTION_CODE: u8 = 0x0a;

pub struct Emitter<W> {
    /// Where this Emitter will write to
    writer: W,
}

impl<W: Write> Emitter<W> {
    /// Emit a single byte to the writer
    pub fn emit_byte(&mut self, byte: u8) -> io::Result<usize> {
        self.emit_bytes(&[byte]).map(|()| 1)
    }

    /// Emit a sequence of bytes to the writer
    pub fn emit_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.writer.write_all(bytes)
    }

    /// Emits the WASM magic constant
    fn emit_magic(&mut self) -> io::Result<()> {
        self.emit_bytes(MAGIC)
    }

    /// Emits the WASM version tag
    fn emit_version(&mut self) -> io::Result<()> {
        self.emit_bytes(VERSION)
    }

    /// Builds a new emitter with the given writer
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Emit the given program to WASM
    pub fn emit_program(&mut self, program: Program) -> io::Result<()> {
        self.emit_magic()?;
        self.emit_version()?;

        for module in program.modules {
            self.emit_module(module)?;
        }

        Ok(())
    }

    fn emit_module(&mut self, module: Module) -> io::Result<()> {
        if module.functions.is_empty() {
            return Ok(());
        }

        self.emit_type_section(&module)?;
        self.emit_function_section(&module)?;
        self.emit_export_section(&module)?;
        self.emit_code_section(&module)?;

        Ok(())
    }

    fn emit_type_section(&mut self, module: &Module) -> io::Result<()> {
        let mut section_data = Vec::new();
        let mut section_emitter = Emitter::new(&mut section_data);

        section_emitter.emit_element(UnsignedLeb128::from(module.functions.len() as u64))?;

        for function in &module.functions {
            section_emitter.emit_byte(0x60)?;
            section_emitter.emit_element(UnsignedLeb128::from(function.parameters.len() as u64))?;

            for param in &function.parameters {
                section_emitter.emit_byte(type_to_byte(&param.type_))?;
            }

            if let Some(ref return_type) = function.return_type {
                section_emitter.emit_byte(0x01)?;
                section_emitter.emit_byte(type_to_byte(return_type))?;
            } else {
                section_emitter.emit_byte(0x00)?;
            }
        }

        self.emit_section(SECTION_TYPE, &section_data)
    }

    fn emit_function_section(&mut self, module: &Module) -> io::Result<()> {
        let mut section_data = Vec::new();
        let mut section_emitter = Emitter::new(&mut section_data);

        section_emitter.emit_element(UnsignedLeb128::from(module.functions.len() as u64))?;

        for (idx, _) in module.functions.iter().enumerate() {
            section_emitter.emit_element(UnsignedLeb128::from(idx as u64))?;
        }

        self.emit_section(SECTION_FUNCTION, &section_data)
    }

    fn emit_export_section(&mut self, module: &Module) -> io::Result<()> {
        let exports: Vec<_> = module
            .functions
            .iter()
            .enumerate()
            .flat_map(|(idx, func)| func.exports.iter().map(move |name| (idx, name)))
            .collect();

        if exports.is_empty() {
            return Ok(());
        }

        let mut section_data = Vec::new();
        let mut section_emitter = Emitter::new(&mut section_data);

        section_emitter.emit_element(UnsignedLeb128::from(exports.len() as u64))?;

        for (func_idx, export_name) in exports {
            section_emitter.emit_element(UnsignedLeb128::from(export_name.len() as u64))?;
            section_emitter.emit_bytes(export_name.as_bytes())?;
            section_emitter.emit_byte(0x00)?;
            section_emitter.emit_element(UnsignedLeb128::from(func_idx as u64))?;
        }

        self.emit_section(SECTION_EXPORT, &section_data)
    }

    fn emit_code_section(&mut self, module: &Module) -> io::Result<()> {
        let mut section_data = Vec::new();
        let mut section_emitter = Emitter::new(&mut section_data);

        section_emitter.emit_element(UnsignedLeb128::from(module.functions.len() as u64))?;

        for function in &module.functions {
            let mut func_data = Vec::new();
            let mut func_emitter = Emitter::new(&mut func_data);

            func_emitter
                .emit_element(UnsignedLeb128::from(function.local_variables.len() as u64))?;

            for local in &function.local_variables {
                func_emitter.emit_byte(0x01)?;
                func_emitter.emit_byte(type_to_byte(&local.type_))?;
            }

            for instruction in &function.body {
                func_emitter.emit_instruction(instruction)?;
            }

            func_emitter.emit_byte(0x0b)?;

            section_emitter.emit_element(UnsignedLeb128::from(func_data.len() as u64))?;
            section_emitter.emit_bytes(&func_data)?;
        }

        self.emit_section(SECTION_CODE, &section_data)
    }

    fn emit_instruction(&mut self, instruction: &crate::ast::Instruction) -> io::Result<()> {
        let opcode_byte = instruction.opcode.to_opcode();
        self.emit_byte(opcode_byte)?;

        match &instruction.opcode {
            Opcode::Constant(constant) => {
                self.emit_element(constant.value)?;
            }
            Opcode::Call(index) => {
                if let crate::ast::Index::Numerical(idx) = index {
                    self.emit_element(UnsignedLeb128::from(*idx as u64))?;
                }
            }
            Opcode::VariableInstruction(var_op) => {
                if let crate::ast::Index::Numerical(idx) = &var_op.index {
                    self.emit_element(UnsignedLeb128::from(*idx as u64))?;
                }
            }
            _ => {}
        }

        for arg in &instruction.arguments {
            self.emit_instruction(arg)?;
        }

        Ok(())
    }

    fn emit_section(&mut self, section_id: u8, data: &[u8]) -> io::Result<()> {
        self.emit_byte(section_id)?;
        self.emit_element(UnsignedLeb128::from(data.len() as u64))?;
        self.emit_bytes(data)?;
        Ok(())
    }

    #[cfg(test)]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

fn type_to_byte(type_: &Type) -> u8 {
    match type_ {
        Type::Numerical(NumericalType::Int32) => 0x7f,
        Type::Numerical(NumericalType::Int64) => 0x7e,
        Type::Numerical(NumericalType::Float32) => 0x7d,
        Type::Numerical(NumericalType::Float64) => 0x7c,
    }
}

impl<W> Emitter<std::io::Cursor<W>> {
    #[cfg(test)]
    pub fn new_cursored(writer: W) -> Self {
        use std::io::Cursor;

        Self {
            writer: Cursor::new(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MAGIC;

    #[test]
    fn assert_correct_magic() {
        assert_eq!(MAGIC, &[0x00, 0x61, 0x73, 0x6d])
    }
}
