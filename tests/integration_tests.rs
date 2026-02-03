use water::ast::*;
use water::emitter::{Emittable, Emitter};
use water::parser::*;

#[test]
fn test_end_to_end_simple_add() {
    // Parse a simple add function
    let wat = r#"(module
        (func $add (param $a i32) (param $b i32) (result i32)
            local.get $a
            local.get $b
            i32.add
        )
        (export "add" (func $add))
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse module");

    let program = Program {
        modules: vec![parsed_module],
    };

    // Emit to binary
    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_program(program).expect("Failed to emit program");

    // Verify we have some output
    assert!(!output.is_empty(), "Output should not be empty");

    // Check magic number
    assert_eq!(&output[0..4], b"\0asm", "Should start with WASM magic");

    // Check version
    assert_eq!(&output[4..8], &[0x01, 0x00, 0x00, 0x00], "Should have version 1");
}

#[test]
fn test_end_to_end_arithmetic() {
    let wat = r#"(module
        (func $compute (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.mul
        )
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse");
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_program(program).expect("Failed to emit");

    assert!(output.len() > 8, "Should have more than just header");
}

#[test]
fn test_end_to_end_with_locals() {
    let wat = r#"(module
        (func $test (param i32) (result i32)
            (local $temp i32)
            local.get 0
            i32.const 10
            i32.add
            local.set $temp
            local.get $temp
        )
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse");
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    let result = emitter.emit_program(program);

    assert!(result.is_ok(), "Should successfully emit with locals");
}

#[test]
fn test_end_to_end_multiple_functions() {
    let wat = r#"(module
        (func $add (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.add
        )
        (func $sub (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.sub
        )
        (export "add" (func $add))
        (export "sub" (func $sub))
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse");
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_program(program).expect("Failed to emit");

    assert!(output.len() > 8);
}

#[test]
fn test_end_to_end_comparison_operations() {
    let wat = r#"(module
        (func $is_equal (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.eq
        )
        (func $is_less (param i32 i32) (result i32)
            local.get 0
            local.get 1
            i32.lt_s
        )
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse");
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    let result = emitter.emit_program(program);

    assert!(result.is_ok(), "Should successfully emit comparison operations");
}

#[test]
fn test_end_to_end_f64_operations() {
    let wat = r#"(module
        (func $add_floats (param f64 f64) (result f64)
            local.get 0
            local.get 1
            f64.add
        )
    )"#;

    let (_, parsed_module) = module(wat).expect("Failed to parse");
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    let result = emitter.emit_program(program);

    assert!(result.is_ok());
}

#[test]
fn test_constant_emission_i32() {
    let constant = Constant {
        value: NumericalValue::Int32(42),
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(constant).expect("Failed to emit");

    assert!(!output.is_empty());
    // First byte should be i32.const opcode (0x41)
    assert_eq!(output[0], 0x41);
}

#[test]
fn test_constant_emission_i64() {
    let constant = Constant {
        value: NumericalValue::Int64(12345),
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(constant).expect("Failed to emit");

    assert!(!output.is_empty());
    // First byte should be i64.const opcode (0x42)
    assert_eq!(output[0], 0x42);
}

#[test]
fn test_constant_emission_f32() {
    let constant = Constant {
        value: NumericalValue::Float32(3.14),
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(constant).expect("Failed to emit");

    assert_eq!(output.len(), 5); // 1 byte opcode + 4 bytes for f32
    assert_eq!(output[0], 0x43); // f32.const opcode
}

#[test]
fn test_constant_emission_f64() {
    let constant = Constant {
        value: NumericalValue::Float64(2.718281828),
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(constant).expect("Failed to emit");

    assert_eq!(output.len(), 9); // 1 byte opcode + 8 bytes for f64
    assert_eq!(output[0], 0x44); // f64.const opcode
}

#[test]
fn test_arithmetic_operation_emission() {
    let op = ArithmeticOperation {
        type_: NumericalType::Int32,
        instr: ArithmeticInstruction::Addition,
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(op).expect("Failed to emit");

    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 0x6a); // i32.add opcode
}

#[test]
fn test_comparison_operation_emission() {
    let op = ComparisonOperation {
        type_: NumericalType::Int32,
        instr: ComparisonInstruction::Equal,
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    emitter.emit_element(op).expect("Failed to emit");

    assert_eq!(output.len(), 1);
    assert_eq!(output[0], 0x45); // i32.eq opcode
}

#[test]
fn test_parse_and_emit_examples_simple_add() {
    let wat = std::fs::read_to_string("examples/simple_add.wat");
    if wat.is_err() {
        // Skip test if example file doesn't exist
        return;
    }
    let wat = wat.unwrap();

    let parse_result = module(&wat);
    assert!(parse_result.is_ok(), "Should parse simple_add.wat");

    let (_, parsed_module) = parse_result.unwrap();
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    let result = emitter.emit_program(program);

    assert!(result.is_ok(), "Should emit simple_add.wat");
    assert!(output.len() > 8, "Should generate substantial output");
}

#[test]
fn test_parse_and_emit_examples_arithmetic() {
    let wat = std::fs::read_to_string("examples/arithmetic.wat");
    if wat.is_err() {
        return;
    }
    let wat = wat.unwrap();

    let parse_result = module(&wat);
    assert!(parse_result.is_ok(), "Should parse arithmetic.wat");

    let (_, parsed_module) = parse_result.unwrap();
    let program = Program {
        modules: vec![parsed_module],
    };

    let mut output = Vec::new();
    let mut emitter = Emitter::new(&mut output);
    let result = emitter.emit_program(program);

    assert!(result.is_ok(), "Should emit arithmetic.wat");
}
