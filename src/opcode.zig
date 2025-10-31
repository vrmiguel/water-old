const std = @import("std");
const ast = @import("ast.zig");

pub fn toOpcode(opcode: ast.Opcode) u8 {
    return switch (opcode) {
        .unreachable => unreachableToOpcode(),
        .call => 0x10,
        .variable_instruction => |var_op| variableOperationToOpcode(var_op),
        .constant => |c| numericalValueToOpcode(c.value),
        .arithmetic => |op| arithmeticOperationToOpcode(op),
        .comparison => |op| comparisonOperationToOpcode(op),
    };
}

pub fn unreachableToOpcode() u8 {
    return 0x00;
}

pub fn numericalValueToOpcode(value: ast.NumericalValue) u8 {
    return switch (value) {
        .int32 => 0x41,
        .int64 => 0x42,
        .float32 => 0x43,
        .float64 => 0x44,
    };
}

pub fn arithmeticOperationToOpcode(op: ast.ArithmeticOperation) u8 {
    return switch (op.type) {
        .int32 => switch (op.instr) {
            .addition => 0x6a,
            .subtraction => 0x6b,
            .multiplication => 0x6c,
            .signed_division => 0x6d,
            .unsigned_division => 0x6e,
            .signed_remainder => 0x6f,
            .unsigned_remainder => 0x70,
            .float_division => unreachable,
        },
        .int64 => switch (op.instr) {
            .addition => 0x7c,
            .subtraction => 0x7d,
            .multiplication => 0x7e,
            .signed_division => 0x7f,
            .unsigned_division => 0x80,
            .signed_remainder => 0x81,
            .unsigned_remainder => 0x82,
            .float_division => unreachable,
        },
        .float32 => switch (op.instr) {
            .addition => 0x92,
            .subtraction => 0x93,
            .multiplication => 0x94,
            .float_division => 0x95,
            .unsigned_division, .signed_division => unreachable,
            .signed_remainder, .unsigned_remainder => unreachable,
        },
        .float64 => switch (op.instr) {
            .addition => 0xa0,
            .subtraction => 0xa1,
            .multiplication => 0xa2,
            .float_division => 0xa3,
            .unsigned_division, .signed_division => unreachable,
            .signed_remainder, .unsigned_remainder => unreachable,
        },
    };
}

pub fn comparisonOperationToOpcode(op: ast.ComparisonOperation) u8 {
    return switch (op.type) {
        .int32 => switch (op.instr) {
            .equal => 0x45,
            .not_equal => 0x47,
            .greater_than => @panic("TODO"),
            .less_than => @panic("TODO"),
            .greater_or_equal => @panic("TODO"),
            .less_or_equal => @panic("TODO"),
        },
        .int64 => switch (op.instr) {
            .equal => 0x51,
            .not_equal => 0x52,
            .greater_than => @panic("TODO"),
            .less_than => @panic("TODO"),
            .greater_or_equal => @panic("TODO"),
            .less_or_equal => @panic("TODO"),
        },
        .float32 => switch (op.instr) {
            .equal => 0x5b,
            .not_equal => 0x5c,
            .greater_than => @panic("TODO"),
            .less_than => @panic("TODO"),
            .greater_or_equal => @panic("TODO"),
            .less_or_equal => @panic("TODO"),
        },
        .float64 => switch (op.instr) {
            .equal => 0x61,
            .not_equal => 0x62,
            .greater_than => @panic("TODO"),
            .less_than => @panic("TODO"),
            .greater_or_equal => @panic("TODO"),
            .less_or_equal => @panic("TODO"),
        },
    };
}

pub fn variableOperationToOpcode(op: ast.VariableOperation) u8 {
    return switch (op.scope) {
        .local => switch (op.instruction) {
            .get => 0x20,
            .set => 0x21,
            .tee => 0x22,
        },
        .global => switch (op.instruction) {
            .get => 0x23,
            .set => 0x24,
            .tee => unreachable,
        },
    };
}
