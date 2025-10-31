const std = @import("std");
const parser = @import("parser.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout = std.io.getStdOut().writer();

    // Example: parse "i32.const 5"
    const result1 = parser.parseConst(allocator, "i32.const 5") catch |err| {
        try stdout.print("Error parsing 'i32.const 5': {}\n", .{err});
        return;
    };
    try stdout.print("Parsed: i32.const 5 = {any}\n", .{result1.value});

    // Example: parse "(i32.const 5)"
    // This would need more complex parsing for parenthesis-enclosed instructions

    // Example: parse module
    const import_wat = "(import \"console\" \"log\" (func $log (param i32) (param i32)))";
    const import_result = parser.parseFunctionImport(allocator, import_wat) catch |err| {
        try stdout.print("Error parsing import: {}\n", .{err});
        // This is expected as we have simplified parser
    };
    _ = import_result;

    try stdout.print("Water WebAssembly Text Format compiler (Zig version)\n", .{});
}
