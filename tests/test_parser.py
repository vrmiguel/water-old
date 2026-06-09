import pytest

from water.ast import (
    Constant,
    Function,
    FunctionImport,
    Index,
    Instruction,
    Local,
    NumericalType,
    NumericalValue,
    Parameter,
    ScopeKind,
    Type,
    Unreachable,
    VariableInstruction,
    VariableOperation,
)
from water.parser import (
    ParseError,
    parse_call,
    parse_const,
    parse_export,
    parse_function,
    parse_function_import,
    parse_identifier,
    parse_index,
    parse_instruction,
    parse_local,
    parse_module,
    parse_parameter,
    parse_variable_instruction,
)


def test_parse_identifier():
    assert parse_identifier("$idx") == ("", "idx")
    assert parse_identifier("$asd_aa? a") == (" a", "asd_aa?")


def test_parse_index():
    assert parse_index("$var") == ("", Index("var"))
    assert parse_index("5") == ("", Index(5))


def test_parse_const():
    assert parse_const("i64.const -5") == ("", NumericalValue(NumericalType.INT64, -5))
    assert parse_const("f64.const 5.5") == ("", NumericalValue(NumericalType.FLOAT64, 5.5))
    assert parse_const("f32.const 2E-3") == ("", NumericalValue(NumericalType.FLOAT32, 0.002))


def test_parse_call():
    assert parse_call("call 5") == ("", Index(5))
    assert parse_instruction("call 5") == ("", Instruction(Index(5)))
    assert parse_instruction("(call 5 (i32.const 5))")[0] == ""
    with pytest.raises(ParseError):
        parse_instruction("(call 5")
    assert parse_call("call $func") == ("", Index("func"))


def test_parse_variable_instruction():
    assert parse_variable_instruction("local.set $idx") == (
        "",
        VariableOperation(ScopeKind.LOCAL, VariableInstruction.SET, Index("idx")),
    )

    with pytest.raises(ParseError):
        parse_variable_instruction("global.tee $idx")


def test_parse_instruction_with_argument():
    rest, instruction = parse_instruction("(local.set $idx (i32.const 5))")

    assert rest == ""
    assert instruction == Instruction(
        VariableOperation(ScopeKind.LOCAL, VariableInstruction.SET, Index("idx")),
        (
            Instruction(
                Constant(NumericalValue(NumericalType.INT32, 5)),
                (),
            ),
        ),
    )


def test_parse_unreachable():
    assert parse_instruction("unreachable") == ("", Instruction(Unreachable()))
    assert parse_instruction("(unreachable (i32.const 5) (i32.const 6))")[0] == ""


def test_parse_export():
    assert parse_export('(export "add")') == ("", "add")
    assert parse_export('(  export  "doSomethingUseful")') == ("", "doSomethingUseful")
    assert parse_export('(export"")') == ("", "")

    invalid = [
        "(export)",
        '(export ")',
        '(export "valid"',
        'export "valid")',
        'export "valid"',
        '(expor "valid"))',
        '(exporT "valid"))',
        '(export "valid"")',
    ]
    for text in invalid:
        with pytest.raises(ParseError):
            parse_export(text)


def test_parse_parameter():
    assert parse_parameter("(param i32)") == (
        "",
        Parameter(None, Type(NumericalType.INT32)),
    )
    assert parse_parameter("( param $number f64)") == (
        "",
        Parameter("number", Type(NumericalType.FLOAT64)),
    )


def test_parse_local():
    assert parse_local("(local f32)") == (
        "",
        Local(None, Type(NumericalType.FLOAT32)),
    )
    assert parse_local("( local $number i64)") == (
        "",
        Local("number", Type(NumericalType.INT64)),
    )


def test_parse_function():
    parameters = (
        Parameter("number", Type(NumericalType.FLOAT64)),
        Parameter(None, Type(NumericalType.INT64)),
    )
    local_variables = (
        Local("l1", Type(NumericalType.INT32)),
        Local(None, Type(NumericalType.FLOAT32)),
    )
    function = Function("add", (), parameters, local_variables)

    assert parse_function(
        "(func $add (param $number f64) (param i64) (local $l1 i32) (local f32))"
    ) == ("", function)


def test_parse_function_import():
    import_wat = '(import "console" "log" (func $log (param f32) (param f32)))'
    parsed_import = FunctionImport(
        namespace="console",
        fn_name="log",
        signature=Function(
            identifier="log",
            parameters=(
                Parameter(None, Type(NumericalType.FLOAT32)),
                Parameter(None, Type(NumericalType.FLOAT32)),
            ),
        ),
    )

    assert parse_function_import(import_wat) == ("", parsed_import)


def test_parse_module():
    assert parse_module("(module)")[0] == ""
    assert parse_module("\n  (module)")[0] == ""

    for text in (" (   module", "module)", "(mod)"):
        with pytest.raises(ParseError):
            parse_module(text)
