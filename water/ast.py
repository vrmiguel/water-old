from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from typing import Union


class NumericalType(Enum):
    INT32 = "i32"
    INT64 = "i64"
    FLOAT32 = "f32"
    FLOAT64 = "f64"


@dataclass(frozen=True)
class Type:
    numerical: NumericalType


@dataclass(frozen=True)
class NumericalValue:
    type_: NumericalType
    value: int | float


@dataclass(frozen=True)
class Parameter:
    identifier: str | None
    type_: Type


@dataclass(frozen=True)
class Local:
    identifier: str | None
    type_: Type


@dataclass(frozen=True)
class Function:
    identifier: str | None
    exports: tuple[str, ...] = ()
    parameters: tuple[Parameter, ...] = ()
    local_variables: tuple[Local, ...] = ()


@dataclass(frozen=True)
class FunctionImport:
    namespace: str
    fn_name: str
    signature: Function


@dataclass(frozen=True)
class Module:
    pass


@dataclass(frozen=True)
class Program:
    modules: tuple[Module, ...] = ()


@dataclass(frozen=True)
class Index:
    value: str | int

    @property
    def is_identifier(self) -> bool:
        return isinstance(self.value, str)

    @property
    def is_numerical(self) -> bool:
        return isinstance(self.value, int)


class ScopeKind(Enum):
    GLOBAL = "global"
    LOCAL = "local"


class VariableInstruction(Enum):
    GET = "get"
    SET = "set"
    TEE = "tee"


@dataclass(frozen=True)
class VariableOperation:
    scope: ScopeKind
    instruction: VariableInstruction
    index: Index


@dataclass(frozen=True)
class Constant:
    value: NumericalValue


class ArithmeticInstruction(Enum):
    ADDITION = "add"
    SUBTRACTION = "sub"
    MULTIPLICATION = "mul"
    FLOAT_DIVISION = "div"
    SIGNED_DIVISION = "div_s"
    UNSIGNED_DIVISION = "div_u"
    SIGNED_REMAINDER = "rem_s"
    UNSIGNED_REMAINDER = "rem_u"


@dataclass(frozen=True)
class ArithmeticOperation:
    type_: NumericalType
    instr: ArithmeticInstruction


class ComparisonInstruction(Enum):
    EQUAL = "eq"
    NOT_EQUAL = "ne"
    GREATER_THAN = "gt"
    LESS_THAN = "lt"
    GREATER_OR_EQUAL = "ge"
    LESS_OR_EQUAL = "le"


@dataclass(frozen=True)
class ComparisonOperation:
    type_: NumericalType
    instr: ComparisonInstruction


@dataclass(frozen=True)
class Unreachable:
    pass


Call = Index
Opcode = Union[
    Call,
    VariableOperation,
    Constant,
    ArithmeticOperation,
    ComparisonOperation,
    Unreachable,
]


@dataclass(frozen=True)
class Instruction:
    opcode: Opcode
    arguments: tuple["Instruction", ...] = field(default_factory=tuple)
