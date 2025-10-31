// A cheaply-clonable String type
// Taken from github.com/vrmiguel/ceceio

const std = @import("std");

pub const INLINE_CAP: usize = 22;

/// A cheaply-clonable String type
pub const SmallString = union(enum) {
    inlined: struct {
        len: u8,
        buf: [INLINE_CAP]u8,
    },
    heap: []const u8,

    pub fn init(allocator: std.mem.Allocator, input: []const u8) !SmallString {
        if (input.len > INLINE_CAP) {
            const heap_str = try allocator.dupe(u8, input);
            return SmallString{ .heap = heap_str };
        } else {
            return inlined(input);
        }
    }

    fn inlined(bytes: []const u8) SmallString {
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

    pub fn deinit(self: SmallString, allocator: std.mem.Allocator) void {
        switch (self) {
            .heap => |s| allocator.free(s),
            .inlined => {},
        }
    }

    pub fn isInHeap(self: SmallString) bool {
        return switch (self) {
            .heap => true,
            .inlined => false,
        };
    }

    pub fn asStr(self: SmallString) []const u8 {
        return switch (self) {
            .inlined => |data| data.buf[0..data.len],
            .heap => |s| s,
        };
    }

    pub fn eql(self: SmallString, other: SmallString) bool {
        return std.mem.eql(u8, self.asStr(), other.asStr());
    }

    pub fn hash(self: SmallString, hasher: anytype) void {
        hasher.update(self.asStr());
    }

    pub fn format(
        self: SmallString,
        comptime fmt: []const u8,
        options: std.fmt.FormatOptions,
        writer: anytype,
    ) !void {
        _ = fmt;
        _ = options;
        try writer.print("s#\"{s}\"", .{self.asStr()});
    }
};

test "creates inlined small strings correctly" {
    const allocator = std.testing.allocator;

    const hey = try SmallString.init(allocator, "hey");
    defer hey.deinit(allocator);
    try std.testing.expectEqualStrings("hey", hey.asStr());
    try std.testing.expect(!hey.isInHeap());

    const length_22 = try SmallString.init(allocator, "abcdefghijkabcdefghijk");
    defer length_22.deinit(allocator);
    try std.testing.expectEqualStrings("abcdefghijkabcdefghijk", length_22.asStr());
    try std.testing.expect(!length_22.isInHeap());

    const length_23 = try SmallString.init(allocator, "abcdefghijkabcdefghijkz");
    defer length_23.deinit(allocator);
    try std.testing.expectEqualStrings("abcdefghijkabcdefghijkz", length_23.asStr());
    try std.testing.expect(length_23.isInHeap());
}
