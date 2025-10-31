const std = @import("std");
const SmallString = @import("small_string.zig").SmallString;
const Allocator = std.mem.Allocator;

pub const Program = struct {
    modules: std.ArrayList(Module),
};

/// Represents a WebAssembly Text Format module
pub const Module = struct {
    // TODO
};

pub const Type = union(enum) {
    numerical: NumericalType,
};

/// The four built-in WebAssembly numerical types.
pub const NumericalType = enum {
    int32,
    int64,
    float32,
    float64,
};

/// The same as NumericalType but actually carries a value that it represents
pub const NumericalValue = union(enum) {
    int32: i32,
    int64: i64,
    float32: f32,
    float64: f64,
};

/// A function parameter.
pub const Parameter = struct {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its index.
    identifier: ?SmallString,
    /// The type of this parameter
    type: Type,
};

/// A local variable within a function.
pub const Local = struct {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its index.
    identifier: ?SmallString,
    /// The type of this parameter
    type: Type,
};

/// Represents a function definition.
pub const Function = struct {
    /// The identifier for this function, if any.
    identifier: ?SmallString,
    /// The identifiers this function will be exported to, if any.
    exports: std.ArrayList(SmallString),
    /// The parameters of this function.
    /// Ordered according to the order the parameters were defined.
    parameters: std.ArrayList(Parameter),
    /// The local variables of this function.
    /// Ordered according to the order the locals were defined.
    local_variables: std.ArrayList(Local),
};

/// Represents an instruction along the possible "inlined" arguments it may have.
pub const Instruction = struct {
    /// The actual operation this instruction represents
    opcode: Opcode,
    /// The list of "inlined" arguments to this instruction, if any.
    arguments: std.ArrayList(Instruction),
};

/// Represents an `import` statement for functions.
pub const FunctionImport = struct {
    namespace: SmallString,
    fn_name: SmallString,
    signature: Function,
};

/// A single instruction that can be located inside a function body
pub const Opcode = union(enum) {
    /// Calls a function
    call: Index,
    /// Fetch or set a local or global variable
    variable_instruction: VariableOperation,
    /// Pushes a numerical constant to the stack.
    constant: Constant,
    /// An arithmetic operation
    arithmetic: ArithmeticOperation,
    /// A comparison operation
    comparison: ComparisonOperation,
    /// Denotes a point in code that should not be reachable.
    unreachable: Unreachable,
};

pub const VariableOperation = struct {
    /// Whether this instruction is in `local.` or `global.`
    scope: ScopeKind,
    /// Defines if we're getting/setting/teeing the variable
    instruction: VariableInstruction,
    /// Accesses the variable either through its definition index or by its identifier
    index: Index,
};

/// Pushes a numerical constant to the stack.
pub const Constant = struct {
    /// Represents both the type of the constant and the constant itself
    value: NumericalValue,
};

/// An arithmetic operation
pub const ArithmeticOperation = struct {
    /// The related type of this operation (i32, i64, f32 or f64)
    type: NumericalType,
    /// The arithmetic instruction of this operation
    instr: ArithmeticInstruction,
};

/// A comparison operation
pub const ComparisonOperation = struct {
    /// The related type of this operation (i32, i64, f32 or f64)
    type: NumericalType,
    /// The arithmetic instruction of this operation
    instr: ComparisonInstruction,
};

/// An index for an instruction, may be an identifier or a numerical index.
pub const Index = union(enum) {
    identifier: SmallString,
    numerical: i64,
};

/// Whether a given instruction is in `local.` or `global.`
pub const ScopeKind = enum {
    global,
    local,
};

/// Represents an instruction for direct variable access.
pub const VariableInstruction = enum {
    /// Get the value of an identifier by its index or identifier.
    get,
    /// Set the value of a variable.
    set,
    /// Like `local.set` but also returns its argument.
    /// Does not exist for `global`.
    tee,
};

pub const ArithmeticInstruction = enum {
    /// i32.add, i64.add, f32.add, or f64.add
    addition,
    /// i32.sub, i64.sub, f32.sub, or f64.sub
    subtraction,
    /// i32.mul, i64.mul, f32.mul, or f64.mul
    multiplication,
    /// f32.div, or f64.div
    float_division,
    /// i32.div_s, i64.div_s
    signed_division,
    /// i32.div_u, i64.div_u
    unsigned_division,
    /// i32.rem_s or i64.rem_s
    signed_remainder,
    /// i32.rem_u or i64.rem_u
    unsigned_remainder,
};

pub const ComparisonInstruction = enum {
    equal,
    not_equal,
    greater_than,
    less_than,
    greater_or_equal,
    less_or_equal,
};

/// Zero-sized type to denote the `unreachable` instruction
pub const Unreachable = struct {};
