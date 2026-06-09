from water.ast import (
    ArithmeticInstruction,
    ArithmeticOperation,
    Constant,
    NumericalType,
    NumericalValue,
    Program,
    Unreachable,
)
from water.emitter import Emitter
from water.leb128 import encode_signed_leb128, encode_unsigned_leb128


def test_emit_program_header():
    emitter = Emitter()
    assert emitter.emit_program(Program()) == 8
    assert emitter.getvalue() == b"\x00asm1000"


def test_encode_signed_leb128():
    to_encode = [
        -(2**63),
        0,
        36,
        128,
        156,
        256,
        512,
        50603,
        -85092,
        -9999999,
        -20312391039,
        2**63 - 1,
    ]
    expected = [
        bytes([128, 128, 128, 128, 128, 128, 128, 128, 128, 127]),
        bytes([0]),
        bytes([36]),
        bytes([128, 1]),
        bytes([156, 1]),
        bytes([128, 2]),
        bytes([128, 4]),
        bytes([171, 139, 3]),
        bytes([156, 231, 122]),
        bytes([129, 211, 157, 123]),
        bytes([129, 133, 166, 170, 180, 127]),
        bytes([255, 255, 255, 255, 255, 255, 255, 255, 255, 0]),
    ]

    assert [encode_signed_leb128(value) for value in to_encode] == expected


def test_encode_unsigned_leb128():
    to_encode = [0, 15, 97, 128, 225, 256, 512, 900, 9203, 242962, 2**64 - 1]
    expected = [
        bytes([0]),
        bytes([15]),
        bytes([97]),
        bytes([128, 1]),
        bytes([225, 1]),
        bytes([128, 2]),
        bytes([128, 4]),
        bytes([132, 7]),
        bytes([243, 71]),
        bytes([146, 234, 14]),
        bytes([255, 255, 255, 255, 255, 255, 255, 255, 255, 1]),
    ]

    assert [encode_unsigned_leb128(value) for value in to_encode] == expected


def test_emit_i32_const():
    emitter = Emitter()
    constant = Constant(NumericalValue(NumericalType.INT32, 128))

    assert emitter.emit_constant(constant) == 3
    assert emitter.getvalue() == bytes([0x41, 128, 1])


def test_emit_i64_const():
    emitter = Emitter()
    constant = Constant(NumericalValue(NumericalType.INT64, 505))

    assert emitter.emit_constant(constant) == 3
    assert emitter.getvalue() == bytes([0x42, 249, 3])


def test_emit_f32_const():
    emitter = Emitter()
    constant = Constant(NumericalValue(NumericalType.FLOAT32, 5.0))

    assert emitter.emit_constant(constant) == 5
    assert emitter.getvalue() == bytes([0x43, 0x00, 0x00, 0xA0, 0x40])


def test_emit_f64_const():
    emitter = Emitter()
    constant = Constant(NumericalValue(NumericalType.FLOAT64, 25.50))

    assert emitter.emit_constant(constant) == 9
    assert emitter.getvalue() == bytes([0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x39, 0x40])


def test_emit_unreachable_opcode():
    emitter = Emitter()

    assert emitter.emit_unreachable(Unreachable()) == 1
    assert emitter.getvalue() == bytes([0x00])


def test_emit_arithmetic_operation():
    emitter = Emitter()
    operation = ArithmeticOperation(NumericalType.INT32, ArithmeticInstruction.ADDITION)

    assert emitter.emit_arithmetic_operation(operation) == 1
    assert emitter.getvalue() == bytes([0x6A])
