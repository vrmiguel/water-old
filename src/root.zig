// Water - WebAssembly Text Format (WAT) compiler library
//
// This is the main library entrypoint that exports all public modules.

pub const ast = @import("ast.zig");
pub const emitter = @import("emitter.zig");
pub const leb128 = @import("leb128.zig");
pub const opcode = @import("opcode.zig");
pub const parser = @import("parser.zig");
pub const small_string = @import("small_string.zig");

// Re-export commonly used types for convenience
pub const Emitter = emitter.Emitter;
pub const SmallString = small_string.SmallString;

// AST types
pub const Program = ast.Program;
pub const Module = ast.Module;
pub const Function = ast.Function;
pub const Instruction = ast.Instruction;
pub const Opcode = ast.Opcode;
pub const NumericalType = ast.NumericalType;
pub const NumericalValue = ast.NumericalValue;

test {
    // This will run all tests in imported modules
    @import("std").testing.refAllDecls(@This());
}
