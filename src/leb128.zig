// Emitters for the Little Endian Base 128 variable length
// integer encoding, which is how WebAssembly stores integer
// literals.
//
// The code in this file is heavily based in the leb128 crate by gimli-rs.

const std = @import("std");
const Emitter = @import("emitter.zig").Emitter;

const CONTINUATION_BIT: u64 = 1 << 7;

/// LEB128 encoder for signed integers
pub const SignedLeb128 = struct {
    value: i64,

    pub fn init(value: i64) SignedLeb128 {
        return .{ .value = value };
    }
};

/// LEB128 encoder for unsigned integers
pub const UnsignedLeb128 = struct {
    value: u64,

    pub fn init(value: u64) UnsignedLeb128 {
        return .{ .value = value };
    }
};

/// Emit a signed LEB128 encoded integer
pub fn emitSignedLeb128(emitter: anytype, element: SignedLeb128) !usize {
    var bytes_written: usize = 0;
    var value = element.value;
    var is_done = false;

    while (!is_done) {
        // Backup the current value
        const bkp = value;

        value >>= 6;

        is_done = (value == 0 or value == -1);
        const byte: u8 = if (is_done)
            @as(u8, @intCast(bkp & ~@as(i64, @intCast(CONTINUATION_BIT))))
        else blk: {
            // Remove the sign bit
            value >>= 1;

            // More bytes to come, so set the continuation bit.
            break :blk @as(u8, @intCast(bkp | @as(i64, @intCast(CONTINUATION_BIT))));
        };

        try emitter.emitByte(byte);
        bytes_written += 1;
    }

    return bytes_written;
}

/// Emit an unsigned LEB128 encoded integer
pub fn emitUnsignedLeb128(emitter: anytype, element: UnsignedLeb128) !usize {
    var bytes_written: usize = 0;
    var value = element.value;

    if (value == 0) {
        try emitter.emitByte(0);
        return 1;
    }

    while (value != 0) {
        var byte = lowBits(value);
        value >>= 7;
        if (value != 0) {
            // More bytes to come, so set the continuation bit.
            byte |= @as(u8, @intCast(CONTINUATION_BIT));
        }

        bytes_written += 1;
        try emitter.emitByte(byte);
    }

    return bytes_written;
}

fn lowBits(value: u64) u8 {
    // This mask has all the lower 8 bits set
    const MASK: u64 = 0xFF;
    const lower_eight_bits = value & MASK;
    return @as(u8, @intCast(lower_eight_bits & ~CONTINUATION_BIT));
}

test "encodes signed leb 128" {
    const to_encode = [_]i64{
        std.math.minInt(i64),
        0,
        36,
        128,
        156,
        256,
        512,
        50603,
        -85092,
        -9999999,
        -20312391039,
        std.math.maxInt(i64),
    };

    const expected_encoding = [_][]const u8{
        &[_]u8{ 128, 128, 128, 128, 128, 128, 128, 128, 128, 127 },
        &[_]u8{0},
        &[_]u8{36},
        &[_]u8{ 128, 1 },
        &[_]u8{ 156, 1 },
        &[_]u8{ 128, 2 },
        &[_]u8{ 128, 4 },
        &[_]u8{ 171, 139, 3 },
        &[_]u8{ 156, 231, 122 },
        &[_]u8{ 129, 211, 157, 123 },
        &[_]u8{ 129, 133, 166, 170, 180, 127 },
        &[_]u8{ 255, 255, 255, 255, 255, 255, 255, 255, 255, 0 },
    };

    for (to_encode, expected_encoding) |value_to_encode, expected| {
        const encoder = SignedLeb128.init(value_to_encode);
        var buffer = std.ArrayList(u8).init(std.testing.allocator);
        defer buffer.deinit();

        var emitter = Emitter(std.ArrayList(u8).Writer).init(buffer.writer());
        _ = try emitSignedLeb128(&emitter, encoder);

        try std.testing.expectEqualSlices(u8, expected, buffer.items);
    }
}

test "encodes unsigned leb 128" {
    const to_encode = [_]u64{
        0,
        15,
        97,
        128,
        225,
        256,
        512,
        900,
        9203,
        242962,
        std.math.maxInt(u64),
    };

    const expected_encoding = [_][]const u8{
        &[_]u8{0},
        &[_]u8{15},
        &[_]u8{97},
        &[_]u8{ 128, 1 },
        &[_]u8{ 225, 1 },
        &[_]u8{ 128, 2 },
        &[_]u8{ 128, 4 },
        &[_]u8{ 132, 7 },
        &[_]u8{ 243, 71 },
        &[_]u8{ 146, 234, 14 },
        &[_]u8{ 255, 255, 255, 255, 255, 255, 255, 255, 255, 1 },
    };

    for (to_encode, expected_encoding) |value_to_encode, expected| {
        const encoder = UnsignedLeb128.init(value_to_encode);
        var buffer = std.ArrayList(u8).init(std.testing.allocator);
        defer buffer.deinit();

        var emitter = Emitter(std.ArrayList(u8).Writer).init(buffer.writer());
        _ = try emitUnsignedLeb128(&emitter, encoder);

        try std.testing.expectEqualSlices(u8, expected, buffer.items);
    }
}
