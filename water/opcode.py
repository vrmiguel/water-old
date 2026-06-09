from __future__ import annotations

from .ast import (
    ArithmeticInstruction,
    ArithmeticOperation,
    ComparisonInstruction,
    ComparisonOperation,
    Constant,
    NumericalType,
    NumericalValue,
    ScopeKind,
    Unreachable,
    VariableInstruction,
    VariableOperation,
)


def numerical_value_opcode(value: NumericalValue) -> int:
    return {
        NumericalType.INT32: 0x41,
        NumericalType.INT64: 0x42,
        NumericalType.FLOAT32: 0x43,
        NumericalType.FLOAT64: 0x44,
    }[value.type_]


def arithmetic_opcode(operation: ArithmeticOperation) -> int:
    mapping = {
        (NumericalType.INT32, ArithmeticInstruction.ADDITION): 0x6A,
        (NumericalType.INT32, ArithmeticInstruction.SUBTRACTION): 0x6B,
        (NumericalType.INT32, ArithmeticInstruction.MULTIPLICATION): 0x6C,
        (NumericalType.INT32, ArithmeticInstruction.SIGNED_DIVISION): 0x6D,
        (NumericalType.INT32, ArithmeticInstruction.UNSIGNED_DIVISION): 0x6E,
        (NumericalType.INT32, ArithmeticInstruction.SIGNED_REMAINDER): 0x6F,
        (NumericalType.INT32, ArithmeticInstruction.UNSIGNED_REMAINDER): 0x70,
        (NumericalType.INT64, ArithmeticInstruction.ADDITION): 0x7C,
        (NumericalType.INT64, ArithmeticInstruction.SUBTRACTION): 0x7D,
        (NumericalType.INT64, ArithmeticInstruction.MULTIPLICATION): 0x7E,
        (NumericalType.INT64, ArithmeticInstruction.SIGNED_DIVISION): 0x7F,
        (NumericalType.INT64, ArithmeticInstruction.UNSIGNED_DIVISION): 0x80,
        (NumericalType.INT64, ArithmeticInstruction.SIGNED_REMAINDER): 0x81,
        (NumericalType.INT64, ArithmeticInstruction.UNSIGNED_REMAINDER): 0x82,
        (NumericalType.FLOAT32, ArithmeticInstruction.ADDITION): 0x92,
        (NumericalType.FLOAT32, ArithmeticInstruction.SUBTRACTION): 0x93,
        (NumericalType.FLOAT32, ArithmeticInstruction.MULTIPLICATION): 0x94,
        (NumericalType.FLOAT32, ArithmeticInstruction.FLOAT_DIVISION): 0x95,
        (NumericalType.FLOAT64, ArithmeticInstruction.ADDITION): 0xA0,
        (NumericalType.FLOAT64, ArithmeticInstruction.SUBTRACTION): 0xA1,
        (NumericalType.FLOAT64, ArithmeticInstruction.MULTIPLICATION): 0xA2,
        (NumericalType.FLOAT64, ArithmeticInstruction.FLOAT_DIVISION): 0xA3,
    }

    try:
        return mapping[(operation.type_, operation.instr)]
    except KeyError as exc:
        raise ValueError(f"invalid arithmetic opcode: {operation}") from exc


def comparison_opcode(operation: ComparisonOperation) -> int:
    mapping = {
        (NumericalType.INT32, ComparisonInstruction.EQUAL): 0x45,
        (NumericalType.INT32, ComparisonInstruction.NOT_EQUAL): 0x47,
        (NumericalType.INT64, ComparisonInstruction.EQUAL): 0x51,
        (NumericalType.INT64, ComparisonInstruction.NOT_EQUAL): 0x52,
        (NumericalType.FLOAT32, ComparisonInstruction.EQUAL): 0x5B,
        (NumericalType.FLOAT32, ComparisonInstruction.NOT_EQUAL): 0x5C,
        (NumericalType.FLOAT64, ComparisonInstruction.EQUAL): 0x61,
        (NumericalType.FLOAT64, ComparisonInstruction.NOT_EQUAL): 0x62,
    }

    try:
        return mapping[(operation.type_, operation.instr)]
    except KeyError as exc:
        raise NotImplementedError(f"comparison opcode is not implemented: {operation}") from exc


def variable_opcode(operation: VariableOperation) -> int:
    mapping = {
        (ScopeKind.LOCAL, VariableInstruction.GET): 0x20,
        (ScopeKind.LOCAL, VariableInstruction.SET): 0x21,
        (ScopeKind.LOCAL, VariableInstruction.TEE): 0x22,
        (ScopeKind.GLOBAL, VariableInstruction.GET): 0x23,
        (ScopeKind.GLOBAL, VariableInstruction.SET): 0x24,
    }

    try:
        return mapping[(operation.scope, operation.instruction)]
    except KeyError as exc:
        raise ValueError(f"invalid variable opcode: {operation}") from exc


def to_opcode(element: object) -> int:
    if isinstance(element, Unreachable):
        return 0x00
    if isinstance(element, Constant):
        return numerical_value_opcode(element.value)
    if isinstance(element, VariableOperation):
        return variable_opcode(element)
    if isinstance(element, ArithmeticOperation):
        return arithmetic_opcode(element)
    if isinstance(element, ComparisonOperation):
        return comparison_opcode(element)
    raise TypeError(f"no opcode mapping for {type(element).__name__}")
