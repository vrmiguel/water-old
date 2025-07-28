use crate::ast::{
    ArithmeticInstruction, ArithmeticOperation,
    ComparisonInstruction, ComparisonOperation, Constant,
    NumericalType, NumericalValue, Opcode, ScopeKind,
    Unreachable, VariableInstruction, VariableOperation,
};

pub trait ToOpcode {
    fn to_opcode(&self) -> u8;
}

impl ToOpcode for Unreachable {
    fn to_opcode(&self) -> u8 {
        0x00
    }
}

impl ToOpcode for NumericalValue {
    fn to_opcode(&self) -> u8 {
        match self {
            NumericalValue::Int32(_) => 0x41,
            NumericalValue::Int64(_) => 0x42,
            NumericalValue::Float32(_) => 0x43,
            NumericalValue::Float64(_) => 0x44,
        }
    }
}

impl ToOpcode for ArithmeticOperation {
    fn to_opcode(&self) -> u8 {
        use NumericalType::*;
        use ArithmeticInstruction::*;
        match (&self.type_, &self.instr) {
            // Integer operations
            (Int32, Addition) => 0x6a,
            (Int32, Subtraction) => 0x6b,
            (Int32, Multiplication) => 0x6c,
            (Int32, SignedDivision) => 0x6d,
            (Int32, UnsignedDivision) => 0x6e,
            (Int32, SignedRemainder) => 0x6f,
            (Int32, UnsignedRemainder) => 0x70,
            (Int64, Addition) => 0x7c,
            (Int64, Subtraction) => 0x7d,
            (Int64, Multiplication) => 0x7e,
            (Int64, SignedDivision) => 0x7f,
            (Int64, UnsignedDivision) => 0x80,
            (Int64, SignedRemainder) => 0x81,
            (Int64, UnsignedRemainder) => 0x82,
            
            // Float operations
            (Float32, Addition) => 0x92,
            (Float32, Subtraction) => 0x93,
            (Float32, Multiplication) => 0x94,
            (Float32, FloatDivision) => 0x95,
            (Float64, Addition) => 0xa0,
            (Float64, Subtraction) => 0xa1,
            (Float64, Multiplication) => 0xa2,
            (Float64, FloatDivision) => 0xa3,
            
            // Invalid combinations
            (Int32 | Int64, FloatDivision) => {
                unreachable!("no float division for integers")
            }
            (
                Float32 | Float64,
                UnsignedDivision | SignedDivision,
            ) => {
                unreachable!(
                    "no signed/unsigned division for floats"
                )
            }
            (
                Float32 | Float64,
                SignedRemainder | UnsignedRemainder,
            ) => {
                unreachable!(
                    "no remainder instruction for floats"
                )
            }
        }
    }
}

impl ToOpcode for ComparisonOperation {
    fn to_opcode(&self) -> u8 {
        let Self { type_, instr } = self;
        match (type_, instr) {
            (
                NumericalType::Int32,
                ComparisonInstruction::Equal,
            ) => 0x45,
            (
                NumericalType::Int32,
                ComparisonInstruction::NotEqual,
            ) => 0x47,
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Int32,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::Equal,
            ) => 0x51,
            (
                NumericalType::Int64,
                ComparisonInstruction::NotEqual,
            ) => 0x52,
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Int64,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::Equal,
            ) => 0x5b,
            (
                NumericalType::Float32,
                ComparisonInstruction::NotEqual,
            ) => 0x5c,
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Float32,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::Equal,
            ) => 0x61,
            (
                NumericalType::Float64,
                ComparisonInstruction::NotEqual,
            ) => 0x62,
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterThan,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessThan,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::GreaterOrEqual,
            ) => todo!(),
            (
                NumericalType::Float64,
                ComparisonInstruction::LessOrEqual,
            ) => todo!(),
        }
    }
}

impl ToOpcode for Opcode {
    fn to_opcode(&self) -> u8 {
        match self {
            Opcode::Unreachable(unreachable) => {
                unreachable.to_opcode()
            }
            Opcode::Call(_) => 0x10,
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
    fn to_opcode(&self) -> u8 {
        use VariableInstruction as Instr;

        let Self {
            scope, instruction, ..
        } = self;

        match (scope, instruction) {
            // local.get
            (ScopeKind::Local, Instr::Get) => 0x20,
            // local.set
            (ScopeKind::Local, Instr::Set) => 0x21,
            // local.tee
            (ScopeKind::Local, Instr::Tee) => 0x22,
            // global.get
            (ScopeKind::Global, Instr::Get) => 0x23,
            // global.set
            (ScopeKind::Global, Instr::Set) => 0x24,
            (ScopeKind::Global, Instr::Tee) => {
                unreachable!("global.tee is not supported")
            }
        }
    }
}
