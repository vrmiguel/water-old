const std = @import("std");
const ast = @import("ast.zig");
const SmallString = @import("small_string.zig").SmallString;
const Allocator = std.mem.Allocator;

pub const ParseError = error{
    UnexpectedToken,
    UnexpectedEndOfInput,
    InvalidIdentifier,
    InvalidType,
    InvalidNumber,
    InvalidString,
    MissingClosingParenthesis,
    MissingOpeningParenthesis,
    OutOfMemory,
};

pub const ParseResult = struct {
    consumed: usize,
};

/// Skips whitespace and returns the number of bytes consumed
fn skipWhitespace(input: []const u8) usize {
    var i: usize = 0;
    while (i < input.len) : (i += 1) {
        switch (input[i]) {
            ' ', '\t', '\n', '\r' => continue,
            else => break,
        }
    }
    return i;
}

/// Parses a string literal enclosed in quotes
pub fn parseString(allocator: Allocator, input: []const u8) !struct { value: []const u8, consumed: usize } {
    _ = allocator;
    if (input.len == 0 or input[0] != '"') {
        return ParseError.InvalidString;
    }

    var i: usize = 1;
    while (i < input.len) : (i += 1) {
        if (input[i] == '"') {
            const str = input[1..i];
            return .{ .value = str, .consumed = i + 1 };
        }
        if (input[i] == '\\' and i + 1 < input.len) {
            i += 1;
        }
    }

    return ParseError.UnexpectedEndOfInput;
}

/// Parses an identifier starting with $
pub fn parseIdentifier(allocator: Allocator, input: []const u8) !struct { value: SmallString, consumed: usize } {
    if (input.len == 0 or input[0] != '$') {
        return ParseError.InvalidIdentifier;
    }

    var i: usize = 1;
    while (i < input.len) : (i += 1) {
        const ch = input[i];
        if (!isAcceptableIdentifierChar(ch)) break;
    }

    if (i == 1) return ParseError.InvalidIdentifier;

    const ident = try SmallString.init(allocator, input[1..i]);
    return .{ .value = ident, .consumed = i };
}

fn isAcceptableIdentifierChar(ch: u8) bool {
    return std.ascii.isAlphanumeric(ch) or switch (ch) {
        '!', '#', '$', '%', '&', '´', '*', '+', '-', '.', '/', ':', '<', '=', '>', '?', '@', '\\', '^', '_', '`', '|', '~' => true,
        else => false,
    };
}

/// Parses a numerical type (i32, i64, f32, f64)
pub fn parseNumericalType(input: []const u8) !struct { value: ast.NumericalType, consumed: usize } {
    if (input.len < 3) return ParseError.InvalidType;

    if (std.mem.startsWith(u8, input, "i32")) {
        return .{ .value = .int32, .consumed = 3 };
    } else if (std.mem.startsWith(u8, input, "i64")) {
        return .{ .value = .int64, .consumed = 3 };
    } else if (std.mem.startsWith(u8, input, "f32")) {
        return .{ .value = .float32, .consumed = 3 };
    } else if (std.mem.startsWith(u8, input, "f64")) {
        return .{ .value = .float64, .consumed = 3 };
    }

    return ParseError.InvalidType;
}

/// Parses a type
pub fn parseType(input: []const u8) !struct { value: ast.Type, consumed: usize } {
    const result = try parseNumericalType(input);
    return .{ .value = .{ .numerical = result.value }, .consumed = result.consumed };
}

/// Parses an index (either identifier or numerical)
pub fn parseIndex(allocator: Allocator, input: []const u8) !struct { value: ast.Index, consumed: usize } {
    if (input.len == 0) return ParseError.UnexpectedEndOfInput;

    if (input[0] == '$') {
        const ident_result = try parseIdentifier(allocator, input);
        return .{ .value = .{ .identifier = ident_result.value }, .consumed = ident_result.consumed };
    }

    // Try to parse as number
    var i: usize = 0;
    const is_negative = input[0] == '-';
    if (is_negative) i += 1;

    var found_digit = false;
    while (i < input.len and std.ascii.isDigit(input[i])) : (i += 1) {
        found_digit = true;
    }

    if (!found_digit) return ParseError.InvalidNumber;

    const num = std.fmt.parseInt(i64, input[0..i], 10) catch return ParseError.InvalidNumber;
    return .{ .value = .{ .numerical = num }, .consumed = i };
}

/// Parses content enclosed in parentheses
pub fn parseParenthesisEnclosed(allocator: Allocator, input: []const u8, comptime parser: anytype) !struct { value: @TypeOf(parser).ReturnType.value, consumed: usize } {
    _ = allocator;
    _ = parser;
    if (input.len == 0 or input[0] != '(') {
        return ParseError.MissingOpeningParenthesis;
    }

    // This is a simplified placeholder
    // In a real implementation, we'd call the parser and look for closing paren
    return ParseError.UnexpectedToken;
}

/// Parses a module
pub fn parseModule(allocator: Allocator, input: []const u8) !struct { value: ast.Module, consumed: usize } {
    _ = allocator;
    var pos: usize = skipWhitespace(input);

    if (pos >= input.len or input[pos] != '(') {
        return ParseError.MissingOpeningParenthesis;
    }
    pos += 1;

    pos += skipWhitespace(input[pos..]);

    if (!std.mem.startsWith(u8, input[pos..], "module")) {
        return ParseError.UnexpectedToken;
    }
    pos += 6;

    pos += skipWhitespace(input[pos..]);

    if (pos >= input.len or input[pos] != ')') {
        return ParseError.MissingClosingParenthesis;
    }
    pos += 1;

    return .{ .value = .{}, .consumed = pos };
}

/// Parses a constant instruction like "i32.const 5"
pub fn parseConst(allocator: Allocator, input: []const u8) !struct { value: ast.NumericalValue, consumed: usize } {
    _ = allocator;
    const type_result = try parseNumericalType(input);
    var pos = type_result.consumed;

    if (!std.mem.startsWith(u8, input[pos..], ".const")) {
        return ParseError.UnexpectedToken;
    }
    pos += 6;

    pos += skipWhitespace(input[pos..]);

    return switch (type_result.value) {
        .int32 => blk: {
            const num = std.fmt.parseInt(i32, input[pos..], 10) catch return ParseError.InvalidNumber;
            // Find how many chars were consumed
            var end = pos;
            if (input[pos] == '-') end += 1;
            while (end < input.len and std.ascii.isDigit(input[end])) : (end += 1) {}
            break :blk .{ .value = .{ .int32 = num }, .consumed = end };
        },
        .int64 => blk: {
            const num = std.fmt.parseInt(i64, input[pos..], 10) catch return ParseError.InvalidNumber;
            var end = pos;
            if (input[pos] == '-') end += 1;
            while (end < input.len and std.ascii.isDigit(input[end])) : (end += 1) {}
            break :blk .{ .value = .{ .int64 = num }, .consumed = end };
        },
        .float32 => blk: {
            const num = std.fmt.parseFloat(f64, input[pos..]) catch return ParseError.InvalidNumber;
            var end = pos;
            if (input[pos] == '-') end += 1;
            while (end < input.len and (std.ascii.isDigit(input[end]) or input[end] == '.' or input[end] == 'e' or input[end] == 'E' or input[end] == '-' or input[end] == '+')) : (end += 1) {}
            break :blk .{ .value = .{ .float32 = @floatCast(num) }, .consumed = end };
        },
        .float64 => blk: {
            const num = std.fmt.parseFloat(f64, input[pos..]) catch return ParseError.InvalidNumber;
            var end = pos;
            if (input[pos] == '-') end += 1;
            while (end < input.len and (std.ascii.isDigit(input[end]) or input[end] == '.' or input[end] == 'e' or input[end] == 'E' or input[end] == '-' or input[end] == '+')) : (end += 1) {}
            break :blk .{ .value = .{ .float64 = num }, .consumed = end };
        },
    };
}

/// Placeholder for instruction parsing
pub fn parseInstruction(allocator: Allocator, input: []const u8) !struct { value: ast.Instruction, consumed: usize } {
    _ = allocator;
    _ = input;
    // This would be a complex parser combining all instruction types
    return ParseError.UnexpectedToken;
}

/// Placeholder for function import parsing
pub fn parseFunctionImport(allocator: Allocator, input: []const u8) !struct { value: ast.FunctionImport, consumed: usize } {
    _ = allocator;
    _ = input;
    return ParseError.UnexpectedToken;
}
