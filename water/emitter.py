from __future__ import annotations

import struct
from io import BytesIO

from .ast import ArithmeticOperation, Constant, NumericalType, NumericalValue, Program, Unreachable
from .leb128 import encode_signed_leb128
from .opcode import arithmetic_opcode, to_opcode

MAGIC = b"\x00asm"
VERSION = b"1000"


class Emitter:
    def __init__(self, writer: BytesIO | None = None) -> None:
        self.writer = writer if writer is not None else BytesIO()

    def emit_byte(self, byte: int) -> int:
        self.writer.write(bytes([byte & 0xFF]))
        return 1

    def emit_bytes(self, data: bytes | bytearray) -> int:
        self.writer.write(bytes(data))
        return len(data)

    def emit_magic(self) -> int:
        return self.emit_bytes(MAGIC)

    def emit_version(self) -> int:
        return self.emit_bytes(VERSION)

    def emit_program(self, program: Program) -> int:
        del program
        return self.emit_magic() + self.emit_version()

    def emit_numerical_value(self, value: NumericalValue) -> int:
        if value.type_ in (NumericalType.INT32, NumericalType.INT64):
            return self.emit_bytes(encode_signed_leb128(int(value.value)))
        if value.type_ is NumericalType.FLOAT32:
            return self.emit_bytes(struct.pack("<f", float(value.value)))
        if value.type_ is NumericalType.FLOAT64:
            return self.emit_bytes(struct.pack("<d", float(value.value)))
        raise ValueError(f"unsupported numerical type: {value.type_}")

    def emit_constant(self, constant: Constant) -> int:
        return self.emit_byte(to_opcode(constant)) + self.emit_numerical_value(constant.value)

    def emit_arithmetic_operation(self, operation: ArithmeticOperation) -> int:
        return self.emit_byte(arithmetic_opcode(operation))

    def emit_unreachable(self, unreachable: Unreachable) -> int:
        return self.emit_byte(to_opcode(unreachable))

    def getvalue(self) -> bytes:
        return self.writer.getvalue()
