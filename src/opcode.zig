const std = @import("std");
const ast = @import("ast.zig");

pub fn toOpcode(value: anytype) u8 {
    const T = @TypeOf(value);

    if (T == ast.Unreachable) {
        return 0x00;
    } else if (T == ast.NumericalValue) {
        return switch (value) {
            .int32 => 0x41,
            .int64 => 0x42,
            .float32 => 0x43,
            .float64 => 0x44,
        };
    } else if (T == ast.ArithmeticOperation) {
        return arithmeticOpToOpcode(value);
    } else if (T == ast.ComparisonOperation) {
        return comparisonOpToOpcode(value);
    } else if (T == ast.Opcode) {
        return opcodeToOpcode(value);
    } else if (T == ast.VariableOperation) {
        return variableOpToOpcode(value);
    } else {
        @compileError("Unsupported type for toOpcode");
    }
}

fn arithmeticOpToOpcode(op: ast.ArithmeticOperation) u8 {
    return switch (op.type_) {
        .int32 => switch (op.instr) {
            .addition => 0x6a,
            .subtraction => 0x6b,
            .multiplication => 0x6c,
            .signed_division => 0x6d,
            .unsigned_division => 0x6e,
            .signed_remainder => 0x6f,
            .unsigned_remainder => 0x70,
            .float_division => unreachable, // no float division for integers
        },
        .int64 => switch (op.instr) {
            .addition => 0x7c,
            .subtraction => 0x7d,
            .multiplication => 0x7e,
            .signed_division => 0x7f,
            .unsigned_division => 0x80,
            .signed_remainder => 0x81,
            .unsigned_remainder => 0x82,
            .float_division => unreachable, // no float division for integers
        },
        .float32 => switch (op.instr) {
            .addition => 0x92,
            .subtraction => 0x93,
            .multiplication => 0x94,
            .float_division => 0x95,
            .signed_division, .unsigned_division => unreachable, // no signed/unsigned division for floats
            .signed_remainder, .unsigned_remainder => unreachable, // no remainder for floats
        },
        .float64 => switch (op.instr) {
            .addition => 0xa0,
            .subtraction => 0xa1,
            .multiplication => 0xa2,
            .float_division => 0xa3,
            .signed_division, .unsigned_division => unreachable, // no signed/unsigned division for floats
            .signed_remainder, .unsigned_remainder => unreachable, // no remainder for floats
        },
    };
}

fn comparisonOpToOpcode(op: ast.ComparisonOperation) u8 {
    return switch (op.type_) {
        .int32 => switch (op.instr) {
            .equal => 0x45,
            .not_equal => 0x47,
            .greater_than => @panic("TODO: implement greater_than for i32"),
            .less_than => @panic("TODO: implement less_than for i32"),
            .greater_or_equal => @panic("TODO: implement greater_or_equal for i32"),
            .less_or_equal => @panic("TODO: implement less_or_equal for i32"),
        },
        .int64 => switch (op.instr) {
            .equal => 0x51,
            .not_equal => 0x52,
            .greater_than => @panic("TODO: implement greater_than for i64"),
            .less_than => @panic("TODO: implement less_than for i64"),
            .greater_or_equal => @panic("TODO: implement greater_or_equal for i64"),
            .less_or_equal => @panic("TODO: implement less_or_equal for i64"),
        },
        .float32 => switch (op.instr) {
            .equal => 0x5b,
            .not_equal => 0x5c,
            .greater_than => @panic("TODO: implement greater_than for f32"),
            .less_than => @panic("TODO: implement less_than for f32"),
            .greater_or_equal => @panic("TODO: implement greater_or_equal for f32"),
            .less_or_equal => @panic("TODO: implement less_or_equal for f32"),
        },
        .float64 => switch (op.instr) {
            .equal => 0x61,
            .not_equal => 0x62,
            .greater_than => @panic("TODO: implement greater_than for f64"),
            .less_than => @panic("TODO: implement less_than for f64"),
            .greater_or_equal => @panic("TODO: implement greater_or_equal for f64"),
            .less_or_equal => @panic("TODO: implement less_or_equal for f64"),
        },
    };
}

fn opcodeToOpcode(opcode: ast.Opcode) u8 {
    return switch (opcode) {
        .unreachable => toOpcode(ast.Unreachable{}),
        .call => 0x10,
        .variable_instruction => |var_op| toOpcode(var_op),
        .constant => |constant| toOpcode(constant.value),
        .arithmetic => |arith| toOpcode(arith),
        .comparison => |comp| toOpcode(comp),
    };
}

fn variableOpToOpcode(var_op: ast.VariableOperation) u8 {
    return switch (var_op.scope) {
        .local => switch (var_op.instruction) {
            .get => 0x20, // local.get
            .set => 0x21, // local.set
            .tee => 0x22, // local.tee
        },
        .global => switch (var_op.instruction) {
            .get => 0x23, // global.get
            .set => 0x24, // global.set
            .tee => unreachable, // global.tee is not supported
        },
    };
}
