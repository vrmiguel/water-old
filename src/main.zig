const std = @import("std");
const water = @import("root.zig");
const parser = water.parser;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout = std.io.getStdOut().writer();

    try stdout.print("Water - WebAssembly Text Format Compiler (Zig version)\n", .{});
    try stdout.print("======================================================\n\n", .{});

    // Example usage (these would fail with the stub parser):
    try stdout.print("Note: This is a Zig port of the water compiler.\n", .{});
    try stdout.print("The parser is currently a stub and would need to be fully implemented.\n", .{});
    try stdout.print("The core emitter and AST structures have been ported from Rust.\n\n", .{});

    // Demonstrate the emitter works by creating a simple constant
    const constant = water.ast.Constant{
        .value = water.ast.NumericalValue{ .int32 = 42 },
    };

    var buffer = std.ArrayList(u8).init(allocator);
    defer buffer.deinit();

    var emitter = water.Emitter(@TypeOf(buffer.writer())).init(buffer.writer());
    _ = try emitter.emitConstant(constant);

    try stdout.print("Successfully emitted i32.const 42 as bytes: ", .{});
    for (buffer.items) |byte| {
        try stdout.print("0x{X:0>2} ", .{byte});
    }
    try stdout.print("\n", .{});
}
