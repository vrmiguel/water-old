use water::ast::*;
use water::parser::*;

#[test]
fn test_parse_simple_i32_type() {
    let input = "i32";
    let result = numerical_type(input);
    assert!(result.is_ok());
    let (_, parsed_type) = result.unwrap();
    assert_eq!(parsed_type, NumericalType::Int32);
}

#[test]
fn test_parse_all_numerical_types() {
    let types = vec![
        ("i32", NumericalType::Int32),
        ("i64", NumericalType::Int64),
        ("f32", NumericalType::Float32),
        ("f64", NumericalType::Float64),
    ];

    for (input, expected) in types {
        let result = numerical_type(input);
        assert!(result.is_ok(), "Failed to parse {}", input);
        let (_, parsed_type) = result.unwrap();
        assert_eq!(parsed_type, expected);
    }
}

#[test]
fn test_parse_i32_const() {
    let input = "i32.const 42";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_i64_const() {
    let input = "i64.const 9223372036854775807";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_f32_const() {
    let input = "f32.const 3.14";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_f64_const() {
    let input = "f64.const -2.5";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_arithmetic_operations() {
    let operations = vec![
        "i32.add",
        "i32.sub",
        "i32.mul",
        "i32.div_s",
        "i32.div_u",
        "i64.add",
        "f32.add",
        "f32.div",
        "f64.mul",
    ];

    for op in operations {
        let result = instruction(op);
        assert!(result.is_ok(), "Failed to parse {}", op);
    }
}

#[test]
fn test_parse_comparison_operations() {
    let operations = vec![
        "i32.eq",
        "i32.ne",
        "i32.lt_s",
        "i32.gt_s",
        "i32.le_s",
        "i32.ge_s",
        "i64.eq",
        "f32.lt",
        "f64.gt",
    ];

    for op in operations {
        let result = instruction(op);
        assert!(result.is_ok(), "Failed to parse {}", op);
    }
}

#[test]
fn test_parse_local_get_with_identifier() {
    let input = "local.get $x";
    let result = instruction(input);
    assert!(result.is_ok());
    let (_, instr) = result.unwrap();
    if let Opcode::VariableInstruction(var_op) = instr.opcode {
        assert_eq!(var_op.scope, ScopeKind::Local);
        assert_eq!(var_op.instruction, VariableInstruction::Get);
        match var_op.index {
            Index::Identifier(ref id) => assert_eq!(id.as_str(), "x"),
            _ => panic!("Expected identifier index"),
        }
    } else {
        panic!("Expected VariableInstruction");
    }
}

#[test]
fn test_parse_local_get_with_index() {
    let input = "local.get 0";
    let result = instruction(input);
    assert!(result.is_ok());
    let (_, instr) = result.unwrap();
    if let Opcode::VariableInstruction(var_op) = instr.opcode {
        assert_eq!(var_op.scope, ScopeKind::Local);
        match var_op.index {
            Index::Numerical(idx) => assert_eq!(idx, 0),
            _ => panic!("Expected numerical index"),
        }
    } else {
        panic!("Expected VariableInstruction");
    }
}

#[test]
fn test_parse_local_set() {
    let input = "local.set $result";
    let result = instruction(input);
    assert!(result.is_ok());
    let (_, instr) = result.unwrap();
    if let Opcode::VariableInstruction(var_op) = instr.opcode {
        assert_eq!(var_op.instruction, VariableInstruction::Set);
    } else {
        panic!("Expected VariableInstruction");
    }
}

#[test]
fn test_parse_local_tee() {
    let input = "local.tee $temp";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_global_get() {
    let input = "global.get $counter";
    let result = instruction(input);
    assert!(result.is_ok());
    let (_, instr) = result.unwrap();
    if let Opcode::VariableInstruction(var_op) = instr.opcode {
        assert_eq!(var_op.scope, ScopeKind::Global);
    } else {
        panic!("Expected VariableInstruction");
    }
}

#[test]
fn test_parse_unreachable() {
    let input = "unreachable";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_call_with_identifier() {
    let input = "call $add";
    let result = instruction(input);
    assert!(result.is_ok());
    let (_, instr) = result.unwrap();
    if let Opcode::Call(Index::Identifier(ref name)) = instr.opcode {
        assert_eq!(name.as_str(), "add");
    } else {
        panic!("Expected Call with identifier");
    }
}

#[test]
fn test_parse_call_with_index() {
    let input = "call 0";
    let result = instruction(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_simple_function() {
    let input = r#"(func $add (param $a i32) (param $b i32) (result i32)
        local.get $a
        local.get $b
        i32.add
    )"#;
    let result = function(input);
    assert!(result.is_ok(), "Failed to parse function: {:?}", result);
}

#[test]
fn test_parse_function_with_export() {
    let input = r#"(func $test (export "test") (result i32)
        i32.const 42
    )"#;
    let result = function(input);
    assert!(result.is_ok());
    let (_, func) = result.unwrap();
    assert_eq!(func.exports.len(), 1);
    assert_eq!(func.exports[0].as_str(), "test");
}

#[test]
fn test_parse_function_with_locals() {
    let input = r#"(func $compute (param i32) (result i32)
        (local $temp i32)
        (local $result i32)
        local.get 0
    )"#;
    let result = function(input);
    assert!(result.is_ok());
    let (_, func) = result.unwrap();
    assert_eq!(func.local_variables.len(), 2);
}

#[test]
fn test_parse_function_no_params_no_result() {
    let input = r#"(func $noop
        unreachable
    )"#;
    let result = function(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_import() {
    let input = r#"(import "env" "log" (func $log (param i32)))"#;
    let result = import(input);
    assert!(result.is_ok());
    let (_, import_def) = result.unwrap();
    assert_eq!(import_def.namespace.as_str(), "env");
    assert_eq!(import_def.fn_name.as_str(), "log");
}

#[test]
fn test_parse_simple_module() {
    let input = r#"(module
        (func $add (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add
        )
        (export "add" (func $add))
    )"#;
    let result = module(input);
    assert!(result.is_ok(), "Failed to parse module: {:?}", result);
}

#[test]
fn test_parse_empty_module() {
    let input = "(module)";
    let result = module(input);
    assert!(result.is_ok());
}

#[test]
fn test_parse_complex_expression() {
    let input = r#"(module
        (func $complex (param $x i32) (result i32)
            (local $y i32)
            i32.const 10
            local.set $y
            local.get $x
            local.get $y
            i32.mul
            i32.const 5
            i32.add
        )
    )"#;
    let result = module(input);
    assert!(result.is_ok());
}
