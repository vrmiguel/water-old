use crate::ast::{
    ArithmeticInstruction, ArithmeticOperation,
    ComparisonInstruction, ComparisonOperation, Constant,
    NumericalType, NumericalValue, Opcode, ScopeKind,
    Unreachable, VariableInstruction, VariableOperation,
};
use crate::error::{Error, Result};

pub trait ToOpcode {
    fn to_opcode(&self) -> Result<u8>;
}

impl ToOpcode for Unreachable {
    fn to_opcode(&self) -> Result<u8> {
        Ok(0x00)
    }
}

impl ToOpcode for NumericalValue {
    fn to_opcode(&self) -> Result<u8> {
        match self {
            NumericalValue::Int32(_) => Ok(0x41),
            NumericalValue::Int64(_) => Ok(0x42),
            NumericalValue::Float32(_) => Ok(0x43),
            NumericalValue::Float64(_) => Ok(0x44),
        }
    }
}

impl ToOpcode for ArithmeticOperation {
    fn to_opcode(&self) -> Result<u8> {
        let Self { type_, instr } = self;
        match (type_, instr) {
            (
                NumericalType::Int32,
                ArithmeticInstruction::Addition,
            ) => Ok(0x6a),
            (
                NumericalType::Int32,
                ArithmeticInstruction::Subtraction,
            ) => Ok(0x6b),
            (
                NumericalType::Int32,
                ArithmeticInstruction::Multiplication,
            ) => Ok(0x6c),
            (
                NumericalType::Int32,
                ArithmeticInstruction::SignedDivision,
            ) => Ok(0x6d),
            (
                NumericalType::Int32,
                ArithmeticInstruction::UnsignedDisivion,
            ) => Ok(0x6e),
            (
                NumericalType::Int32,
                ArithmeticInstruction::SignedRemainder,
            ) => Ok(0x6f),
            (
                NumericalType::Int32,
                ArithmeticInstruction::UnsignedRemainder,
            ) => Ok(0x70),
            (
                NumericalType::Int64,
                ArithmeticInstruction::Addition,
            ) => Ok(0x7c),
            (
                NumericalType::Int64,
                ArithmeticInstruction::Subtraction,
            ) => Ok(0x7d),
            (
                NumericalType::Int64,
                ArithmeticInstruction::Multiplication,
            ) => Ok(0x7e),
            (
                NumericalType::Int64,
                ArithmeticInstruction::SignedDivision,
            ) => Ok(0x7f),
            (
                NumericalType::Int64,
                ArithmeticInstruction::UnsignedDisivion,
            ) => Ok(0x80),
            (
                NumericalType::Int64,
                ArithmeticInstruction::SignedRemainder,
            ) => Ok(0x81),
            (
                NumericalType::Int64,
                ArithmeticInstruction::UnsignedRemainder,
            ) => Ok(0x82),
            (
                NumericalType::Int32
                | NumericalType::Int64,
                ArithmeticInstruction::FloatDivision,
            ) => Err(Error::InvalidOperation {
                message: "no float division for integers".to_string(),
            }),
            (
                NumericalType::Float32,
                ArithmeticInstruction::Addition,
            ) => Ok(0x92),
            (
                NumericalType::Float32,
                ArithmeticInstruction::Subtraction,
            ) => Ok(0x93),
            (
                NumericalType::Float32,
                ArithmeticInstruction::Multiplication,
            ) => Ok(0x94),
            (
                NumericalType::Float32,
                ArithmeticInstruction::FloatDivision,
            ) => Ok(0x95),
            (
                NumericalType::Float64,
                ArithmeticInstruction::Addition,
            ) => Ok(0xa0),
            (
                NumericalType::Float64,
                ArithmeticInstruction::Subtraction,
            ) => Ok(0xa1),
            (
                NumericalType::Float64,
                ArithmeticInstruction::Multiplication,
            ) => Ok(0xa2),
            (
                NumericalType::Float64,
                ArithmeticInstruction::FloatDivision,
            ) => Ok(0xa3),
            (
                NumericalType::Float32
                | NumericalType::Float64,
                ArithmeticInstruction::UnsignedDisivion
                | ArithmeticInstruction::SignedDivision,
            ) => Err(Error::InvalidOperation {
                message: "no signed or unsigned division for floating numbers".to_string(),
            }),
            (
                NumericalType::Float32 | NumericalType::Float64,
                ArithmeticInstruction::SignedRemainder | ArithmeticInstruction::UnsignedRemainder,
            ) => Err(Error::InvalidOperation {
                message: "no remainder instruction for floating numbers".to_string(),
            }),
        }
    }
}

impl ToOpcode for ComparisonOperation {
    fn to_opcode(&self) -> Result<u8> {
        let Self { type_, instr } = self;
        match (type_, instr) {
            (
                NumericalType::Int32,
                ComparisonInstruction::Equal,
            ) => Ok(0x45),
            (
                NumericalType::Int32,
                ComparisonInstruction::NotEqual,
            ) => Ok(0x47),
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterThan,
            ) => Err(Error::InvalidOperation {
                message: "Int32 GreaterThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessThan,
            ) => Err(Error::InvalidOperation {
                message: "Int32 LessThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Int32 GreaterOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Int32 LessOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Int64,
                ComparisonInstruction::Equal,
            ) => Ok(0x51),
            (
                NumericalType::Int64,
                ComparisonInstruction::NotEqual,
            ) => Ok(0x52),
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterThan,
            ) => Err(Error::InvalidOperation {
                message: "Int64 GreaterThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessThan,
            ) => Err(Error::InvalidOperation {
                message: "Int64 LessThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Int64 GreaterOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Int64 LessOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Float32,
                ComparisonInstruction::Equal,
            ) => Ok(0x5b),
            (
                NumericalType::Float32,
                ComparisonInstruction::NotEqual,
            ) => Ok(0x5c),
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterThan,
            ) => Err(Error::InvalidOperation {
                message: "Float32 GreaterThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessThan,
            ) => Err(Error::InvalidOperation {
                message: "Float32 LessThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Float32 GreaterOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Float32 LessOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Float64,
                ComparisonInstruction::Equal,
            ) => Ok(0x61),
            (
                NumericalType::Float64,
                ComparisonInstruction::NotEqual,
            ) => Ok(0x62),
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterThan,
            ) => Err(Error::InvalidOperation {
                message: "Float64 GreaterThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessThan,
            ) => Err(Error::InvalidOperation {
                message: "Float64 LessThan not implemented yet".to_string(),
            }),
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Float64 GreaterOrEqual not implemented yet".to_string(),
            }),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessOrEqual,
            ) => Err(Error::InvalidOperation {
                message: "Float64 LessOrEqual not implemented yet".to_string(),
            }),
        }
    }
}

impl ToOpcode for Opcode {
    fn to_opcode(&self) -> Result<u8> {
        match self {
            Opcode::Unreachable(unreachable) => {
                unreachable.to_opcode()
            }
            Opcode::Call(_) => Ok(0x10),
            Opcode::VariableInstruction(variable_operation) => {
                variable_operation.to_opcode()
            }
            Opcode::Constant(Constant { value }) => {
                value.to_opcode()
            }
            Opcode::Arithmetic(op) => op.to_opcode(),
            Opcode::Comparison(op) => op.to_opcode(),
        }
    }
}

impl ToOpcode for VariableOperation {
    fn to_opcode(&self) -> Result<u8> {
        use VariableInstruction as Instr;

        let Self {
            scope, instruction, ..
        } = self;

        match (scope, instruction) {
            // local.get
            (ScopeKind::Local, Instr::Get) => Ok(0x20),
            // local.set
            (ScopeKind::Local, Instr::Set) => Ok(0x21),
            // local.tee
            (ScopeKind::Local, Instr::Tee) => Ok(0x22),
            // global.get
            (ScopeKind::Global, Instr::Get) => Ok(0x23),
            // global.set
            (ScopeKind::Global, Instr::Set) => Ok(0x24),
            (ScopeKind::Global, Instr::Tee) => {
                Err(Error::InvalidOperation {
                    message: "global.tee is not supported".to_string(),
                })
            }
        }
    }
}
