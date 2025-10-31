const std = @import("std");
const ast = @import("ast.zig");
const opcode = @import("opcode.zig");
const leb128 = @import("leb128.zig");

const MAGIC = "\x00asm";
const VERSION = "1000";

pub const Emitter = struct {
    writer: std.ArrayList(u8).Writer,

    pub fn init(writer: std.ArrayList(u8).Writer) Emitter {
        return .{ .writer = writer };
    }

    /// Emit a single byte to the writer
    pub fn emitByte(self: *Emitter, byte: u8) !void {
        try self.writer.writeByte(byte);
    }

    /// Emit a sequence of bytes to the writer
    pub fn emitBytes(self: *Emitter, bytes: []const u8) !void {
        try self.writer.writeAll(bytes);
    }

    /// Emits the WASM magic constant
    fn emitMagic(self: *Emitter) !void {
        try self.emitBytes(MAGIC);
    }

    /// Emits the WASM version tag
    fn emitVersion(self: *Emitter) !void {
        try self.emitBytes(VERSION);
    }

    /// Emit the given program to WASM
    pub fn emitProgram(self: *Emitter, program: ast.Program) !void {
        _ = program;
        try self.emitMagic();
        try self.emitVersion();
    }

    // Emittable implementations

    pub fn emitUnreachable(self: *Emitter, unreachable_instr: ast.Unreachable) !void {
        _ = unreachable_instr;
        const op = opcode.unreachableToOpcode();
        try self.emitByte(op);
    }

    pub fn emitConstant(self: *Emitter, constant: ast.Constant) !void {
        const op = opcode.numericalValueToOpcode(constant.value);
        try self.emitByte(op);
        try self.emitNumericalValue(constant.value);
    }

    pub fn emitNumericalValue(self: *Emitter, value: ast.NumericalValue) !void {
        switch (value) {
            .int32 => |i| {
                const encoder = leb128.SignedLeb128.init(@as(i64, i));
                _ = try encoder.encode(self.writer);
            },
            .int64 => |i| {
                const encoder = leb128.SignedLeb128.init(i);
                _ = try encoder.encode(self.writer);
            },
            .float32 => |f| {
                const bytes = std.mem.toBytes(f);
                try self.emitBytes(&bytes);
            },
            .float64 => |f| {
                const bytes = std.mem.toBytes(f);
                try self.emitBytes(&bytes);
            },
        }
    }

    pub fn emitArithmeticOperation(self: *Emitter, op: ast.ArithmeticOperation) !void {
        const op_code = opcode.arithmeticOperationToOpcode(op);
        try self.emitByte(op_code);
    }
};

test "assert correct magic" {
    const magic_bytes = [_]u8{ 0x00, 0x61, 0x73, 0x6d };
    try std.testing.expectEqualSlices(u8, &magic_bytes, MAGIC);
}

test "emits i32 const correctly" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    var emitter = Emitter.init(buf.writer());
    const constant = ast.Constant{
        .value = .{ .int32 = 128 },
    };

    try emitter.emitConstant(constant);

    const expected = [_]u8{
        0x41, // `i32.const`'s opcode
        128,
        1, // LEB128 for 128
    };
    try std.testing.expectEqualSlices(u8, &expected, buf.items);
}

test "emits i64 const correctly" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    var emitter = Emitter.init(buf.writer());
    const constant = ast.Constant{
        .value = .{ .int64 = 505 },
    };

    try emitter.emitConstant(constant);

    const expected = [_]u8{
        0x42, // `i64.const`'s opcode
        249,
        3, // LEB128 for 505
    };
    try std.testing.expectEqualSlices(u8, &expected, buf.items);
}

test "emits f32 const correctly" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    var emitter = Emitter.init(buf.writer());
    const constant = ast.Constant{
        .value = .{ .float32 = 5.0 },
    };

    try emitter.emitConstant(constant);

    const expected = [_]u8{
        0x43, // `f32.const`'s opcode
        0x00,
        0x00,
        0xa0,
        0x40, // LE bit pattern for 5.0
    };
    try std.testing.expectEqualSlices(u8, &expected, buf.items);
}

test "emits f64 const correctly" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    var emitter = Emitter.init(buf.writer());
    const constant = ast.Constant{
        .value = .{ .float64 = 25.50 },
    };

    try emitter.emitConstant(constant);

    const expected = [_]u8{
        0x44, // `f64.const`'s opcode
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x80,
        0x39,
        0x40, // LE bit pattern for 25.50
    };
    try std.testing.expectEqualSlices(u8, &expected, buf.items);
}

test "emits unreachable opcode" {
    var buf = std.ArrayList(u8).init(std.testing.allocator);
    defer buf.deinit();

    var emitter = Emitter.init(buf.writer());
    try emitter.emitUnreachable(.{});

    const expected = [_]u8{0x00};
    try std.testing.expectEqualSlices(u8, &expected, buf.items);
}
