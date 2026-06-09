from __future__ import annotations

import re
from collections.abc import Callable
from typing import TypeVar

from .ast import (
    Constant,
    Function,
    FunctionImport,
    Index,
    Instruction,
    Local,
    Module,
    NumericalType,
    NumericalValue,
    Parameter,
    ScopeKind,
    Type,
    Unreachable,
    VariableInstruction,
    VariableOperation,
)

T = TypeVar("T")

_IDENTIFIER_CHARS = set("!#$%&'*+-./:<=>?@\\^_`|~")


class ParseError(ValueError):
    pass


def _skip_ws(input_: str, pos: int) -> int:
    while pos < len(input_) and input_[pos].isspace():
        pos += 1
    return pos


def _tag(input_: str, pos: int, tag: str) -> int:
    if input_.startswith(tag, pos):
        return pos + len(tag)
    raise ParseError(f"expected {tag!r} at byte {pos}")


def _parse_parenthesized(input_: str, pos: int, inner: Callable[[str, int], tuple[int, T]]) -> tuple[int, T]:
    pos = _tag(input_, pos, "(")
    pos = _skip_ws(input_, pos)
    pos, value = inner(input_, pos)
    pos = _skip_ws(input_, pos)
    pos = _tag(input_, pos, ")")
    return pos, value


def _finish(input_: str, pos: int, value: T) -> tuple[str, T]:
    return input_[pos:], value


def parse_identifier(input_: str) -> tuple[str, str]:
    pos, identifier = _parse_identifier_at(input_, 0)
    return _finish(input_, pos, identifier)


def _parse_identifier_at(input_: str, pos: int) -> tuple[int, str]:
    pos = _tag(input_, pos, "$")
    start = pos
    while pos < len(input_) and (input_[pos].isalnum() or input_[pos] in _IDENTIFIER_CHARS):
        pos += 1
    if pos == start:
        raise ParseError(f"expected identifier at byte {start}")
    return pos, input_[start:pos]


def parse_string(input_: str) -> tuple[str, str]:
    pos, value = _parse_string_at(input_, 0)
    return _finish(input_, pos, value)


def _parse_string_at(input_: str, pos: int) -> tuple[int, str]:
    pos = _tag(input_, pos, '"')
    chars: list[str] = []
    while pos < len(input_):
        ch = input_[pos]
        if ch == '"':
            return pos + 1, "".join(chars)
        if ch == "\\":
            if pos + 1 >= len(input_) or input_[pos + 1] != '"':
                raise ParseError(f"unsupported string escape at byte {pos}")
            chars.append('"')
            pos += 2
            continue
        chars.append(ch)
        pos += 1
    raise ParseError("unterminated string literal")


def parse_numerical_type(input_: str) -> tuple[str, NumericalType]:
    pos, type_ = _parse_numerical_type_at(input_, 0)
    return _finish(input_, pos, type_)


def _parse_numerical_type_at(input_: str, pos: int) -> tuple[int, NumericalType]:
    for text, type_ in (
        ("i32", NumericalType.INT32),
        ("i64", NumericalType.INT64),
        ("f32", NumericalType.FLOAT32),
        ("f64", NumericalType.FLOAT64),
    ):
        if input_.startswith(text, pos):
            return pos + len(text), type_
    raise ParseError(f"expected numerical type at byte {pos}")


def parse_type(input_: str) -> tuple[str, Type]:
    pos, type_ = _parse_type_at(input_, 0)
    return _finish(input_, pos, type_)


def _parse_type_at(input_: str, pos: int) -> tuple[int, Type]:
    pos, numerical_type = _parse_numerical_type_at(input_, pos)
    return pos, Type(numerical_type)


def parse_index(input_: str) -> tuple[str, Index]:
    pos, index = _parse_index_at(input_, 0)
    return _finish(input_, pos, index)


def _parse_index_at(input_: str, pos: int) -> tuple[int, Index]:
    if pos < len(input_) and input_[pos] == "$":
        pos, identifier = _parse_identifier_at(input_, pos)
        return pos, Index(identifier)

    match = re.match(r"[+-]?\d+", input_[pos:])
    if not match:
        raise ParseError(f"expected numerical index or identifier at byte {pos}")
    return pos + match.end(), Index(int(match.group(0)))


def parse_const(input_: str) -> tuple[str, NumericalValue]:
    pos, value = _parse_const_at(input_, 0)
    return _finish(input_, pos, value)


def _parse_const_at(input_: str, pos: int) -> tuple[int, NumericalValue]:
    pos, numerical_type = _parse_numerical_type_at(input_, pos)
    pos = _tag(input_, pos, ".const")
    pos = _skip_ws(input_, pos)

    if numerical_type in (NumericalType.INT32, NumericalType.INT64):
        match = re.match(r"[+-]?\d+", input_[pos:])
        if not match:
            raise ParseError(f"expected integer literal at byte {pos}")
        return pos + match.end(), NumericalValue(numerical_type, int(match.group(0)))

    match = re.match(r"[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?", input_[pos:])
    if not match:
        raise ParseError(f"expected floating-point literal at byte {pos}")
    return pos + match.end(), NumericalValue(numerical_type, float(match.group(0)))


def parse_call(input_: str) -> tuple[str, Index]:
    pos = _tag(input_, 0, "call")
    pos = _skip_ws(input_, pos)
    pos, index = _parse_index_at(input_, pos)
    return _finish(input_, pos, index)


def parse_variable_instruction(input_: str) -> tuple[str, VariableOperation]:
    pos, operation = _parse_variable_instruction_at(input_, 0)
    return _finish(input_, pos, operation)


def _parse_variable_instruction_at(input_: str, pos: int) -> tuple[int, VariableOperation]:
    if input_.startswith("global", pos):
        scope = ScopeKind.GLOBAL
        pos += len("global")
    elif input_.startswith("local", pos):
        scope = ScopeKind.LOCAL
        pos += len("local")
    else:
        raise ParseError(f"expected variable scope at byte {pos}")

    options = ((".set", VariableInstruction.SET), (".get", VariableInstruction.GET))
    if scope is ScopeKind.LOCAL:
        options = options + ((".tee", VariableInstruction.TEE),)

    for suffix, instruction in options:
        if input_.startswith(suffix, pos):
            pos += len(suffix)
            break
    else:
        raise ParseError(f"expected variable instruction at byte {pos}")

    pos = _skip_ws(input_, pos)
    pos, index = _parse_index_at(input_, pos)
    return pos, VariableOperation(scope, instruction, index)


def parse_unreachable(input_: str) -> tuple[str, Unreachable]:
    pos = _tag(input_, 0, "unreachable")
    return _finish(input_, pos, Unreachable())


def parse_opcode(input_: str) -> tuple[str, object]:
    pos, opcode = _parse_opcode_at(input_, 0)
    return _finish(input_, pos, opcode)


def _parse_opcode_at(input_: str, pos: int) -> tuple[int, object]:
    for parser in (_parse_variable_instruction_at, _parse_const_opcode_at, _parse_unreachable_at, _parse_call_at):
        try:
            return parser(input_, pos)
        except ParseError:
            continue
    raise ParseError(f"expected opcode at byte {pos}")


def _parse_const_opcode_at(input_: str, pos: int) -> tuple[int, Constant]:
    pos, value = _parse_const_at(input_, pos)
    return pos, Constant(value)


def _parse_unreachable_at(input_: str, pos: int) -> tuple[int, Unreachable]:
    return _tag(input_, pos, "unreachable"), Unreachable()


def _parse_call_at(input_: str, pos: int) -> tuple[int, Index]:
    pos = _tag(input_, pos, "call")
    pos = _skip_ws(input_, pos)
    return _parse_index_at(input_, pos)


def parse_instruction(input_: str) -> tuple[str, Instruction]:
    pos, instruction = _parse_instruction_at(input_, 0)
    return _finish(input_, pos, instruction)


def _parse_instruction_at(input_: str, pos: int) -> tuple[int, Instruction]:
    if pos < len(input_) and input_[pos] == "(":
        return _parse_parenthesized(input_, pos, _parse_instruction_with_arguments_at)

    pos, opcode = _parse_opcode_at(input_, pos)
    return pos, Instruction(opcode)


def _parse_instruction_with_arguments_at(input_: str, pos: int) -> tuple[int, Instruction]:
    pos, opcode = _parse_opcode_at(input_, pos)
    arguments: list[Instruction] = []

    while True:
        next_pos = _skip_ws(input_, pos)
        if next_pos >= len(input_) or input_[next_pos] != "(":
            return pos, Instruction(opcode, tuple(arguments))
        pos, argument = _parse_parenthesized(input_, next_pos, _parse_instruction_with_arguments_at)
        arguments.append(argument)


def parse_export(input_: str) -> tuple[str, str]:
    pos, value = _parse_export_at(input_, 0)
    return _finish(input_, pos, value)


def _parse_export_at(input_: str, pos: int) -> tuple[int, str]:
    def inner(input_: str, pos: int) -> tuple[int, str]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "export")
        pos = _skip_ws(input_, pos)
        return _parse_string_at(input_, pos)

    pos = _skip_ws(input_, pos)
    return _parse_parenthesized(input_, pos, inner)


def parse_parameter(input_: str) -> tuple[str, Parameter]:
    pos, parameter = _parse_parameter_at(input_, 0)
    return _finish(input_, pos, parameter)


def _parse_parameter_at(input_: str, pos: int) -> tuple[int, Parameter]:
    def inner(input_: str, pos: int) -> tuple[int, Parameter]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "param")
        pos = _skip_ws(input_, pos)
        identifier = None
        if pos < len(input_) and input_[pos] == "$":
            pos, identifier = _parse_identifier_at(input_, pos)
            pos = _skip_ws(input_, pos)
        pos, type_ = _parse_type_at(input_, pos)
        return pos, Parameter(identifier, type_)

    pos = _skip_ws(input_, pos)
    return _parse_parenthesized(input_, pos, inner)


def parse_local(input_: str) -> tuple[str, Local]:
    pos, local = _parse_local_at(input_, 0)
    return _finish(input_, pos, local)


def _parse_local_at(input_: str, pos: int) -> tuple[int, Local]:
    def inner(input_: str, pos: int) -> tuple[int, Local]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "local")
        pos = _skip_ws(input_, pos)
        identifier = None
        if pos < len(input_) and input_[pos] == "$":
            pos, identifier = _parse_identifier_at(input_, pos)
            pos = _skip_ws(input_, pos)
        pos, type_ = _parse_type_at(input_, pos)
        return pos, Local(identifier, type_)

    pos = _skip_ws(input_, pos)
    return _parse_parenthesized(input_, pos, inner)


def parse_function(input_: str) -> tuple[str, Function]:
    pos, function = _parse_function_at(input_, 0)
    return _finish(input_, pos, function)


def _parse_function_at(input_: str, pos: int) -> tuple[int, Function]:
    def inner(input_: str, pos: int) -> tuple[int, Function]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "func")
        pos = _skip_ws(input_, pos)
        identifier = None
        if pos < len(input_) and input_[pos] == "$":
            pos, identifier = _parse_identifier_at(input_, pos)

        exports: list[str] = []
        parameters: list[Parameter] = []
        locals_: list[Local] = []

        while True:
            next_pos = _skip_ws(input_, pos)
            try:
                pos, export = _parse_export_at(input_, next_pos)
                exports.append(export)
                continue
            except ParseError:
                pass

            try:
                pos, parameter = _parse_parameter_at(input_, next_pos)
                parameters.append(parameter)
                continue
            except ParseError:
                pass

            try:
                pos, local = _parse_local_at(input_, next_pos)
                locals_.append(local)
                continue
            except ParseError:
                pass

            return pos, Function(identifier, tuple(exports), tuple(parameters), tuple(locals_))

    pos = _skip_ws(input_, pos)
    return _parse_parenthesized(input_, pos, inner)


def parse_function_import(input_: str) -> tuple[str, FunctionImport]:
    pos, import_ = _parse_function_import_at(input_, 0)
    return _finish(input_, pos, import_)


def _parse_function_import_at(input_: str, pos: int) -> tuple[int, FunctionImport]:
    def inner(input_: str, pos: int) -> tuple[int, FunctionImport]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "import")
        pos = _skip_ws(input_, pos)
        pos, namespace = _parse_string_at(input_, pos)
        pos = _skip_ws(input_, pos)
        pos, fn_name = _parse_string_at(input_, pos)
        pos = _skip_ws(input_, pos)
        pos, function = _parse_function_at(input_, pos)
        if function.exports or function.local_variables:
            raise ParseError("function imports cannot contain exports or locals")
        return pos, FunctionImport(namespace, fn_name, function)

    pos = _skip_ws(input_, pos)
    return _parse_parenthesized(input_, pos, inner)


def parse_module(input_: str) -> tuple[str, Module]:
    def inner(input_: str, pos: int) -> tuple[int, Module]:
        pos = _skip_ws(input_, pos)
        pos = _tag(input_, pos, "module")
        return pos, Module()

    pos = _skip_ws(input_, 0)
    pos, module = _parse_parenthesized(input_, pos, inner)
    return _finish(input_, pos, module)
