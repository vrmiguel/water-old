const std = @import("std");
const Allocator = std.mem.Allocator;

pub const INLINE_CAP = 22;

/// A cheaply-clonable String type
pub const SmallString = union(enum) {
    inlined: struct {
        len: u8,
        buf: [INLINE_CAP]u8,
    },
    heap: []const u8,

    pub fn init(allocator: Allocator, input: []const u8) !SmallString {
        if (input.len > INLINE_CAP) {
            const heap_str = try allocator.dupe(u8, input);
            return SmallString{ .heap = heap_str };
        } else {
            return initInlined(input);
        }
    }

    fn initInlined(bytes: []const u8) SmallString {
        std.debug.assert(bytes.len <= INLINE_CAP);
        var buf = [_]u8{0} ** INLINE_CAP;
        @memcpy(buf[0..bytes.len], bytes);
        return SmallString{
            .inlined = .{
                .len = @intCast(bytes.len),
                .buf = buf,
            },
        };
    }

    pub fn deinit(self: *SmallString, allocator: Allocator) void {
        switch (self.*) {
            .heap => |str| allocator.free(str),
            .inlined => {},
        }
    }

    pub fn isInHeap(self: SmallString) bool {
        return switch (self) {
            .heap => true,
            .inlined => false,
        };
    }

    pub fn asSlice(self: SmallString) []const u8 {
        return switch (self) {
            .inlined => |inl| inl.buf[0..inl.len],
            .heap => |str| str,
        };
    }

    pub fn eql(self: SmallString, other: SmallString) bool {
        const self_slice = self.asSlice();
        const other_slice = other.asSlice();
        return std.mem.eql(u8, self_slice, other_slice);
    }

    pub fn hash(self: SmallString) u64 {
        return std.hash.Wyhash.hash(0, self.asSlice());
    }

    pub fn format(
        self: SmallString,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;
        try writer.print("s#\"{s}\"", .{self.asSlice()});
    }
};

test "creates inlined small strings correctly" {
    const allocator = std.testing.allocator;

    const hey = try SmallString.init(allocator, "hey");
    defer hey.deinit(allocator);
    try std.testing.expectEqualStrings("hey", hey.asSlice());
    try std.testing.expect(!hey.isInHeap());

    const length_22 = try SmallString.init(allocator, "abcdefghijkabcdefghijk");
    defer length_22.deinit(allocator);
    try std.testing.expectEqualStrings("abcdefghijkabcdefghijk", length_22.asSlice());
    try std.testing.expect(!length_22.isInHeap());

    const length_23 = try SmallString.init(allocator, "abcdefghijkabcdefghijkz");
    defer length_23.deinit(allocator);
    try std.testing.expectEqualStrings("abcdefghijkabcdefghijkz", length_23.asSlice());
    try std.testing.expect(length_23.isInHeap());
}
