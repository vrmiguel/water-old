const std = @import("std");
const ast = @import("ast.zig");
const opcode = @import("opcode.zig");
const leb128 = @import("leb128.zig");

const MAGIC: []const u8 = "\x00asm";
const VERSION: []const u8 = "1000";

pub fn Emitter(comptime WriterType: type) type {
    return struct {
        const Self = @This();

        /// Where this Emitter will write to
        writer: WriterType,

        pub fn init(writer: WriterType) Self {
            return Self{ .writer = writer };
        }

        /// Emit a single byte to the writer
        pub fn emitByte(self: *Self, byte: u8) !usize {
            try self.writer.writeByte(byte);
            return 1;
        }

        /// Emit a sequence of bytes to the writer
        pub fn emitBytes(self: *Self, bytes: []const u8) !void {
            try self.writer.writeAll(bytes);
        }

        /// Emits the WASM magic constant
        fn emitMagic(self: *Self) !void {
            try self.emitBytes(MAGIC);
        }

        /// Emits the WASM version tag
        fn emitVersion(self: *Self) !void {
            try self.emitBytes(VERSION);
        }

        /// Emit the given program to WASM
        pub fn emitProgram(self: *Self, program: ast.Program) !void {
            _ = program;
            try self.emitMagic();
            try self.emitVersion();
        }

        // Emittable implementations

        /// Emit an Unreachable instruction
        pub fn emitUnreachable(self: *Self, unreachable_inst: ast.Unreachable) !usize {
            _ = unreachable_inst;
            const op = opcode.toOpcode(ast.Unreachable{});
            return try self.emitByte(op);
        }

        /// Emit a NumericalValue
        pub fn emitNumericalValue(self: *Self, value: ast.NumericalValue) !usize {
            return switch (value) {
                .int32 => |int32| try leb128.emitSignedLeb128(self, leb128.SignedLeb128.init(@as(i64, int32))),
                .int64 => |int64| try leb128.emitSignedLeb128(self, leb128.SignedLeb128.init(int64)),
                .float32 => |f32_val| {
                    const bytes = f32ToBytes(f32_val);
                    try self.emitBytes(&bytes);
                    return 4;
                },
                .float64 => |f64_val| {
                    const bytes = f64ToBytes(f64_val);
                    try self.emitBytes(&bytes);
                    return 8;
                },
            };
        }

        /// Emit a Constant
        pub fn emitConstant(self: *Self, constant: ast.Constant) !usize {
            const op = opcode.toOpcode(constant.value);

            // Emit the `const` opcode for the given value
            _ = try self.emitByte(op);

            // .. and then the actual literal
            return try self.emitNumericalValue(constant.value);
        }

        /// Emit an ArithmeticOperation
        pub fn emitArithmeticOperation(self: *Self, arith_op: ast.ArithmeticOperation) !usize {
            const op = opcode.toOpcode(arith_op);
            return try self.emitByte(op);
        }
    };
}

/// According to the WebAssembly spec, floating-point values are encoded
/// by their IEEE 754-2019 (Section 3.4) bit pattern in little endian byte order.
fn f32ToBytes(n: f32) [4]u8 {
    return @bitCast(std.mem.nativeToLittle(u32, @bitCast(n)));
}

fn f64ToBytes(n: f64) [8]u8 {
    return @bitCast(std.mem.nativeToLittle(u64, @bitCast(n)));
}

test "assert correct magic" {
    try std.testing.expectEqualSlices(u8, &[_]u8{ 0x00, 0x61, 0x73, 0x6d }, MAGIC);
}

test "emits unreachable opcode" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();

    var emitter = Emitter(@TypeOf(buffer.writer())).init(buffer.writer());
    _ = try emitter.emitUnreachable(ast.Unreachable{});

    try std.testing.expectEqualSlices(u8, &[_]u8{0x00}, buffer.items);
}

test "emits i32 const correctly" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();

    var emitter = Emitter(@TypeOf(buffer.writer())).init(buffer.writer());

    const constant = ast.Constant{
        .value = ast.NumericalValue{ .int32 = 128 },
    };

    _ = try emitter.emitConstant(constant);

    try std.testing.expectEqualSlices(
        u8,
        &[_]u8{
            // `i32.const`'s opcode
            0x41,
            // and the LEB128 for 128
            128,
            1,
        },
        buffer.items,
    );
}

test "emits i64 const correctly" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();

    var emitter = Emitter(@TypeOf(buffer.writer())).init(buffer.writer());

    const constant = ast.Constant{
        .value = ast.NumericalValue{ .int64 = 505 },
    };

    _ = try emitter.emitConstant(constant);

    try std.testing.expectEqualSlices(
        u8,
        &[_]u8{
            // `i64.const`'s opcode
            0x42,
            // and the LEB128 for 505
            249,
            3,
        },
        buffer.items,
    );
}

test "emits f32 const correctly" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();

    var emitter = Emitter(@TypeOf(buffer.writer())).init(buffer.writer());

    const constant = ast.Constant{
        .value = ast.NumericalValue{ .float32 = 5.0 },
    };

    _ = try emitter.emitConstant(constant);

    try std.testing.expectEqualSlices(
        u8,
        &[_]u8{
            // `f32.const`'s opcode
            0x43,
            // and then the LE bit pattern for 5.0
            0x00, 0x00, 0xa0, 0x40,
        },
        buffer.items,
    );
}

test "emits f64 const correctly" {
    var buffer = std.ArrayList(u8).init(std.testing.allocator);
    defer buffer.deinit();

    var emitter = Emitter(@TypeOf(buffer.writer())).init(buffer.writer());

    const constant = ast.Constant{
        .value = ast.NumericalValue{ .float64 = 25.50 },
    };

    _ = try emitter.emitConstant(constant);

    try std.testing.expectEqualSlices(
        u8,
        &[_]u8{
            // `f64.const`'s opcode
            0x44,
            // and then the LE bit pattern for 25.50
            0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x39, 0x40,
        },
        buffer.items,
    );
}
