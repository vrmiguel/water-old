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
    utils::{index_parsen, numerischer_typ_parsen},
    IResult,
};
use crate::{
    ast::{
        Constant, Index, Instruction, NumericalType,
        NumericalValue, Opcode, ScopeKind, Unreachable,
        VariableInstruction, VariableOperation,
    },
    parser::utils::in_klammern_eingeschlossen_parsen,
};

pub fn anweisung_parsen(input: &str) -> IResult<Instruction> {
    fn einfache_anweisung_parsen(
        input: &str,
    ) -> IResult<Instruction> {
        let (rest, opcode) = opcode_parsen(input)?;

        let instr = Instruction {
            opcode,
            arguments: Vec::new(),
        };

        Ok((rest, instr))
    }

    fn anweisung_mit_argumenten_parsen(
        input: &str,
    ) -> IResult<Instruction> {
        let (rest, opcode) = opcode_parsen(input)?;

        let (rest, arguments) = many0(preceded(
            multispace0,
            in_klammern_eingeschlossen_parsen(anweisung_parsen),
        ))(rest)?;

        let instr = Instruction { opcode, arguments };

        Ok((rest, instr))
    }

    alt((
        einfache_anweisung_parsen,
        in_klammern_eingeschlossen_parsen(
            anweisung_mit_argumenten_parsen,
        ),
    ))(input)
}

pub fn opcode_parsen(input: &str) -> IResult<Opcode> {
    alt((
        variablen_anweisung_parsen
            .map(Opcode::VariableInstruction),
        konstante_parsen
            .map(|value| Constant { value })
            .map(Opcode::Constant),
        unerreichbar_parsen.map(Opcode::Unreachable),
        context("call", aufruf_parsen).map(Opcode::Call),
    ))(input)
}

/// Parses a `const` operation, such as `i32.const 20` or
/// `f32.const 2.2`
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{NumericalValue, Instruction};
/// use water::parser::konstante_parsen;
/// use water::parser::anweisung_parsen;
///
/// assert_eq!(konstante_parsen("i64.const -5"), Ok(("", NumericalValue::Int64(-5))));
/// assert_eq!(konstante_parsen("f64.const 5.5"), Ok(("", NumericalValue::Float64(5.5))));
/// assert_eq!(konstante_parsen("f32.const 2E-3"), Ok(("", NumericalValue::Float32(0.002))));
/// ```
pub fn konstante_parsen(input: &str) -> IResult<NumericalValue> {
    // Parse the numerical type of this instruction: i32, i64,
    // f32 or f64
    let (rest, numerical_type) = numerischer_typ_parsen(input)?;
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
/// use water::parser::aufruf_parsen;
/// use water::parser::anweisung_parsen;
///
/// assert_eq!(aufruf_parsen("call 5"), Ok(("", Index::Numerical(5))));
/// assert!(anweisung_parsen("call 5").is_ok());
/// assert!(anweisung_parsen("(call 5 (i32.const 5))").is_ok());
/// assert!(anweisung_parsen("(call 5").is_err());
/// assert_eq!(aufruf_parsen("call $func"), Ok(("", Index::Identifier("func".into()))));
/// ```
pub fn aufruf_parsen(input: &str) -> IResult<Index> {
    let (rest, _) = tag("call")(input)?;

    preceded(
        multispace0,
        context("numerical index or identifier", index_parsen),
    )(rest)
}

/// Parses an instruction for direct variable access.
///
/// Does not eat leading whitespace.
///
/// ```
/// use water::ast::{ScopeKind, VariableInstruction, VariableOperation, Opcode, Index};
/// use water::parser::variablen_anweisung_parsen;
///
/// assert_eq!(
///     variablen_anweisung_parsen("local.set $idx"),
///     Ok(("", VariableOperation {
///         scope: ScopeKind::Local,
///         instruction: VariableInstruction::Set,
///         index: Index::Identifier("idx".into()),
///     }))
/// );
/// ```
pub fn variablen_anweisung_parsen(
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
        preceded(multispace0, index_parsen)(rest)?;

    let operation = VariableOperation {
        scope,
        instruction: opcode,
        index,
    };

    Ok((rest, operation))
}

/// Parses the `unreachable` instruction
pub fn unerreichbar_parsen(input: &str) -> IResult<Unreachable> {
    let (rest, _) = tag("unreachable")(input)?;

    Ok((rest, Unreachable))
}
