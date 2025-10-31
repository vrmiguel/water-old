// The abstract syntax tree for WAT (WebAssembly Text Format)

const std = @import("std");
const SmallString = @import("small_string.zig").SmallString;

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
    /// Signed integer of 32 bits
    int32,
    /// Signed integer of 64 bits
    int64,
    /// Floating-number of 32 bits
    float32,
    /// Floating-number of 64 bits
    float64,
};

/// The same as NumericalType but actually carries a value
/// that it represents
pub const NumericalValue = union(enum) {
    /// Signed integer of 32 bits
    int32: i32,
    /// Signed integer of 64 bits
    int64: i64,
    /// Floating-number of 32 bits
    float32: f32,
    /// Floating-number of 64 bits
    float64: f64,
};

/// A function parameter.
pub const Parameter = struct {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its index.
    identifier: ?SmallString,
    /// The type of this parameter
    type_: Type,
};

/// A local variable within a function.
pub const Local = struct {
    /// The identifier of this parameter. May not be present, in
    /// which case the local must be accessed through its index.
    identifier: ?SmallString,
    /// The type of this parameter
    type_: Type,
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

/// Represents an instruction along the possible "inlined"
/// arguments it may have.
pub const Instruction = struct {
    /// The actual operation this instruction represents
    opcode: Opcode,
    /// The list of "inlined" arguments to this instruction, if any.
    // TODO: transform this into a "generic" Value
    // TODO: investigate use of SmallVec here
    arguments: std.ArrayList(Instruction),
};

/// Represents an `import` statement for functions.
///
/// Consists of the namespace from which we're importing from,
/// the name of the imported and the WAT function signature the
/// imported function will be attached to.
///
/// E.g.:
///
/// ```
///               function name
///                    ↓↓↓
/// (import "console" "log" (func $log (param i32 i32)))
///          ↑↑↑↑↑↑↑         ↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
///         namespace           WAT function signature
/// ```
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
    ///
    /// E.g. `i32.const 5`, `f64.const 2.5`
    constant: Constant,
    /// An arithmetic operation
    arithmetic: ArithmeticOperation,
    comparison: ComparisonOperation,
    /// Denotes a point in code that should not be reachable.
    /// `unreachable` is an unconditional trap: in the case
    /// where an unreachable is reached and executed, the
    /// instruction traps.
    ///
    /// Note: unreachable accepts any arity.
    ///
    /// ```
    /// (i32.const 6)
    /// (unreachable (i32.const 5) (i32.const 5))
    /// ```
    unreachable: Unreachable,
};

pub const VariableOperation = struct {
    /// Whether this instruction is in `local.` or `global.`
    scope: ScopeKind,
    /// Defines if we're getting/setting/teeing the variable
    instruction: VariableInstruction,
    /// Accesses the variable either through its definition
    /// index or by its identifier
    index: Index,
};

/// Pushes a numerical constant to the stack.
///
/// E.g. `i32.const 5`, `f64.const 2.5`
pub const Constant = struct {
    /// Represents both the type of the constant
    /// and the constant itself
    value: NumericalValue,
};

/// An arithmetic operation
pub const ArithmeticOperation = struct {
    /// The related type of this operation (i32, i64, f32 or f64)
    type_: NumericalType,
    /// The arithmetic instruction of this operation (such as
    /// addition, subtraction, etc)
    instr: ArithmeticInstruction,
};

/// A comparison operation
pub const ComparisonOperation = struct {
    /// The related type of this operation (i32, i64, f32 or f64)
    type_: NumericalType,
    /// The arithmetic instruction of this operation (such as
    /// equal, not equal, greater than, etc)
    instr: ComparisonInstruction,
};

/// An index for an instruction, may be an identifier or a
/// numerical index.
///
/// Examples:
///
/// * `call $function` (function is an identifier in an indexing position)
/// * `local.get 0` (0 is a numerical index)
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
    ///
    /// E.g. `get $number`
    get,
    /// Set the value of a variable.
    ///
    /// E.g. `(local.set $var (i32.const 10)) ;; set $var to 10`
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

/// Zero-sized type to denote the `unreachable` instruction,
/// which denotes a point in code that should not be reachable.
/// `unreachable` is an unconditional trap: in the case
/// where an unreachable is reached and executed, the
/// instruction traps.
pub const Unreachable = struct {};
