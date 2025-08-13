//! Parsing functions specific to instructions

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        i32 as parse_i32, i64 as parse_i64, multispace0,
    },
    combinator::value,
    error::context,
    multi::many0,
    number::complete::double as parse_f64,
    sequence::preceded,
    Parser,
};

use super::{
    utils::{parse_index, parse_numerical_type},
    IResult,
};
use crate::{
    ast::{
        Constant, Index, Instruction, NumericalType,
        NumericalValue, Opcode, ScopeKind, Unreachable,
        VariableInstruction, VariableOperation,
    },
    parser::utils::parse_parenthesis_enclosed,
};

pub fn parse_instruction(input: &str) -> IResult<Instruction> {
    fn parse_plain_instruction(
        input: &str,
    ) -> IResult<Instruction> {
        let (rest, opcode) = parse_opcode(input)?;

        let instr = Instruction {
            opcode,
            arguments: Vec::new(),
        };

        Ok((rest, instr))
    }

    fn parse_instruction_with_arguments(
        input: &str,
    ) -> IResult<Instruction> {
        let (rest, opcode) = parse_opcode(input)?;

        let (rest, arguments) = many0(preceded(
            multispace0,
            parse_parenthesis_enclosed(parse_instruction),
        ))(rest)?;

        let instr = Instruction { opcode, arguments };

        Ok((rest, instr))
    }

    alt((
        parse_plain_instruction,
        parse_parenthesis_enclosed(
            parse_instruction_with_arguments,
        ),
    ))(input)
}

pub fn parse_opcode(input: &str) -> IResult<Opcode> {
    alt((
        parse_variable_instruction
            .map(Opcode::VariableInstruction),
        parse_const
            .map(|value| Constant { value })
            .map(Opcode::Constant),
        parse_unreachable.map(Opcode::Unreachable),
        context("call", parse_call).map(Opcode::Call),
    ))(input)
}

/// Parses a `const` operation, such as `i32.const 20` or
/// `f32.const 2.2`
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{NumericalValue, Instruction};
/// use water::parser::parse_const;
/// use water::parser::parse_instruction;
///
/// assert_eq!(parse_const("i64.const -5"), Ok(("", NumericalValue::Int64(-5))));
/// assert_eq!(parse_const("f64.const 5.5"), Ok(("", NumericalValue::Float64(5.5))));
/// assert_eq!(parse_const("f32.const 2E-3"), Ok(("", NumericalValue::Float32(0.002))));
/// ```
pub fn parse_const(input: &str) -> IResult<NumericalValue> {
    // Parse the numerical type of this instruction: i32, i64,
    // f32 or f64
    let (rest, numerical_type) = parse_numerical_type(input)?;
    // Parse the preceding ".const" opcode
    let (rest, _) = tag(".const")(rest)?;

    match numerical_type {
        NumericalType::Int32 => {
            let (rest, int32) =
                preceded(multispace0, parse_i32)(rest)?;

            Ok((rest, NumericalValue::Int32(int32)))
        }
        NumericalType::Int64 => {
            let (rest, int64) =
                preceded(multispace0, parse_i64)(rest)?;

            Ok((rest, NumericalValue::Int64(int64)))
        }
        NumericalType::Float32 => {
            let (rest, float64) =
                preceded(multispace0, parse_f64)(rest)?;

            // TODO: parsing f32.const as f64 and then casting to
            // f32 is a hack and we should switch to using
            // `nom::number::complete::f32`
            Ok((rest, NumericalValue::Float32(float64 as f32)))
        }
        NumericalType::Float64 => {
            let (rest, float64) =
                preceded(multispace0, parse_f64)(rest)?;

            Ok((rest, NumericalValue::Float64(float64)))
        }
    }
}

/// Parses a `call` instruction alongside its index.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{Index, Instruction};
/// use water::parser::parse_call;
/// use water::parser::parse_instruction;
///
/// assert_eq!(parse_call("call 5"), Ok(("", Index::Numerical(5))));
/// assert!(parse_instruction("call 5").is_ok());
/// assert!(parse_instruction("(call 5 (i32.const 5))").is_ok());
/// assert!(parse_instruction("(call 5").is_err());
/// assert_eq!(parse_call("call $func"), Ok(("", Index::Identifier("func".into()))));
/// ```
pub fn parse_call(input: &str) -> IResult<Index> {
    let (rest, _) = tag("call")(input)?;

    preceded(
        multispace0,
        context("numerical index or identifier", parse_index),
    )(rest)
}

/// Parses an instruction for direct variable access.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{ScopeKind, VariableInstruction, VariableOperation, Opcode, Index};
/// use water::parser::parse_variable_instruction;
///
/// assert_eq!(
///     parse_variable_instruction("local.set $idx"),
///     Ok(("", VariableOperation {
///         scope: ScopeKind::Local,
///         instruction: VariableInstruction::Set,
///         index: Index::Identifier("idx".into()),
///     }))
/// );
/// ```
pub fn parse_variable_instruction(
    input: &str,
) -> IResult<VariableOperation> {
    let (rest, scope) = alt((
        value(ScopeKind::Global, tag("global")),
        value(ScopeKind::Local, tag("local")),
    ))(input)?;

    let parse_set = value(VariableInstruction::Set, tag(".set"));
    let parse_get = value(VariableInstruction::Get, tag(".get"));
    let parse_tee = value(VariableInstruction::Tee, tag(".tee"));

    let (rest, opcode) = match scope {
        ScopeKind::Global => {
            // Ensure we don't parse `global.tee`
            alt((parse_set, parse_get))(rest)?
        }
        ScopeKind::Local => {
            alt((parse_set, parse_get, parse_tee))(rest)?
        }
    };

    let (rest, index) =
        preceded(multispace0, parse_index)(rest)?;

    let operation = VariableOperation {
        scope,
        instruction: opcode,
        index,
    };

    Ok((rest, operation))
}

/// Parses the `unreachable` instruction
pub fn parse_unreachable(input: &str) -> IResult<Unreachable> {
    let (rest, _) = tag("unreachable")(input)?;

    Ok((rest, Unreachable))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction_simple_opcode() {
        let result = parse_instruction("unreachable");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction {
            opcode: Opcode::Unreachable(Unreachable),
            arguments: vec![]
        });
    }

    #[test]
    fn test_parse_instruction_with_parentheses() {
        let result = parse_instruction("(unreachable)");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction {
            opcode: Opcode::Unreachable(Unreachable),
            arguments: vec![]
        });
    }

    #[test]
    fn test_parse_instruction_call_numerical() {
        let result = parse_instruction("call 5");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction {
            opcode: Opcode::Call(Index::Numerical(5)),
            arguments: vec![]
        });
    }

    #[test]
    fn test_parse_instruction_call_identifier() {
        let result = parse_instruction("call $func");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction, Instruction {
            opcode: Opcode::Call(Index::Identifier("func".into())),
            arguments: vec![]
        });
    }

    #[test]
    fn test_parse_instruction_with_arguments() {
        let result = parse_instruction("(call 5 (i32.const 5))");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction.opcode, Opcode::Call(Index::Numerical(5)));
        assert_eq!(instruction.arguments.len(), 1);
        assert_eq!(instruction.arguments[0].opcode, 
                   Opcode::Constant(Constant { value: NumericalValue::Int32(5) }));
    }

    #[test]
    fn test_parse_opcode_unreachable() {
        let result = parse_opcode("unreachable");
        assert!(result.is_ok());
        let (remaining, opcode) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(opcode, Opcode::Unreachable(Unreachable));
    }

    #[test]
    fn test_parse_opcode_call() {
        let result = parse_opcode("call 42");
        assert!(result.is_ok());
        let (remaining, opcode) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(opcode, Opcode::Call(Index::Numerical(42)));
    }

    #[test]
    fn test_parse_const_i32() {
        let result = parse_const("i32.const 42");
        assert!(result.is_ok());
        let (remaining, value) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, NumericalValue::Int32(42));
    }

    #[test]
    fn test_parse_const_i64() {
        let result = parse_const("i64.const -5");
        assert!(result.is_ok());
        let (remaining, value) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, NumericalValue::Int64(-5));
    }

    #[test]
    fn test_parse_const_f32() {
        let result = parse_const("f32.const 2E-3");
        assert!(result.is_ok());
        let (remaining, value) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, NumericalValue::Float32(0.002));
    }

    #[test]
    fn test_parse_const_f64() {
        let result = parse_const("f64.const 5.5");
        assert!(result.is_ok());
        let (remaining, value) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(value, NumericalValue::Float64(5.5));
    }

    #[test]
    fn test_parse_const_with_whitespace() {
        assert!(parse_const("i32.const  42").is_ok());
        assert!(parse_const("i32.const\t42").is_ok());
        assert!(parse_const("i32.const\n42").is_ok());
    }

    #[test]
    fn test_parse_const_failures() {
        assert!(parse_const("i32.const").is_err());
        assert!(parse_const("i32const 42").is_err());
        assert!(parse_const("invalid.const 42").is_err());
        assert!(parse_const("i33.const 42").is_err());
    }

    #[test]
    fn test_parse_call_numerical() {
        let result = parse_call("call 5");
        assert!(result.is_ok());
        let (remaining, index) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(index, Index::Numerical(5));
    }

    #[test]
    fn test_parse_call_identifier() {
        let result = parse_call("call $func");
        assert!(result.is_ok());
        let (remaining, index) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(index, Index::Identifier("func".into()));
    }

    #[test]
    fn test_parse_call_with_whitespace() {
        assert!(parse_call("call  5").is_ok());
        assert!(parse_call("call\t$func").is_ok());
        assert!(parse_call("call\n5").is_ok());
    }

    #[test]
    fn test_parse_call_failures() {
        assert!(parse_call("call").is_err());
        assert!(parse_call("cal 5").is_err());
        assert!(parse_call("CALL 5").is_err());
    }

    #[test]
    fn test_parse_variable_instruction_local_get() {
        let result = parse_variable_instruction("local.get $idx");
        assert!(result.is_ok());
        let (remaining, op) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(op, VariableOperation {
            scope: ScopeKind::Local,
            instruction: VariableInstruction::Get,
            index: Index::Identifier("idx".into()),
        });
    }

    #[test]
    fn test_parse_variable_instruction_local_set() {
        let result = parse_variable_instruction("local.set 0");
        assert!(result.is_ok());
        let (remaining, op) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(op, VariableOperation {
            scope: ScopeKind::Local,
            instruction: VariableInstruction::Set,
            index: Index::Numerical(0),
        });
    }

    #[test]
    fn test_parse_variable_instruction_local_tee() {
        let result = parse_variable_instruction("local.tee $var");
        assert!(result.is_ok());
        let (remaining, op) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(op, VariableOperation {
            scope: ScopeKind::Local,
            instruction: VariableInstruction::Tee,
            index: Index::Identifier("var".into()),
        });
    }

    #[test]
    fn test_parse_variable_instruction_global_get() {
        let result = parse_variable_instruction("global.get 1");
        assert!(result.is_ok());
        let (remaining, op) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(op, VariableOperation {
            scope: ScopeKind::Global,
            instruction: VariableInstruction::Get,
            index: Index::Numerical(1),
        });
    }

    #[test]
    fn test_parse_variable_instruction_global_set() {
        let result = parse_variable_instruction("global.set $global_var");
        assert!(result.is_ok());
        let (remaining, op) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(op, VariableOperation {
            scope: ScopeKind::Global,
            instruction: VariableInstruction::Set,
            index: Index::Identifier("global_var".into()),
        });
    }

    #[test]
    fn test_parse_variable_instruction_with_whitespace() {
        assert!(parse_variable_instruction("local.get  $idx").is_ok());
        assert!(parse_variable_instruction("local.set\t0").is_ok());
        assert!(parse_variable_instruction("global.get\n1").is_ok());
    }

    #[test]
    fn test_parse_variable_instruction_failures() {
        assert!(parse_variable_instruction("global.tee $var").is_err());
        assert!(parse_variable_instruction("local.invalid $var").is_err());
        assert!(parse_variable_instruction("invalid.get $var").is_err());
        assert!(parse_variable_instruction("local.get").is_err());
        assert!(parse_variable_instruction("global.set").is_err());
    }

    #[test]
    fn test_parse_unreachable() {
        let result = parse_unreachable("unreachable");
        assert!(result.is_ok());
        let (remaining, unreachable_instr) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(unreachable_instr, Unreachable);
    }

    #[test]
    fn test_parse_unreachable_with_trailing() {
        let result = parse_unreachable("unreachable rest");
        assert!(result.is_ok());
        let (remaining, _) = result.unwrap();
        assert_eq!(remaining, " rest");
    }

    #[test]
    fn test_parse_unreachable_failures() {
        assert!(parse_unreachable("UNREACHABLE").is_err());
        assert!(parse_unreachable("unreach").is_err());
        assert!(parse_unreachable("unreachables").is_err());
        assert!(parse_unreachable("").is_err());
    }

    #[test]
    fn test_complex_instruction_parsing() {
        let result = parse_instruction("(call $add (i32.const 5) (i32.const 10))");
        assert!(result.is_ok());
        let (remaining, instruction) = result.unwrap();
        assert_eq!(remaining, "");
        assert_eq!(instruction.opcode, Opcode::Call(Index::Identifier("add".into())));
        assert_eq!(instruction.arguments.len(), 2);
        assert_eq!(instruction.arguments[0].opcode, 
                   Opcode::Constant(Constant { value: NumericalValue::Int32(5) }));
        assert_eq!(instruction.arguments[1].opcode,
                   Opcode::Constant(Constant { value: NumericalValue::Int32(10) }));
    }
}
