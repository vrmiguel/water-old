;; Tables example
;; Demonstrates function tables and indirect calls

(module
  ;; Define a function table with 3 slots
  (table 3 funcref)

  ;; Define some functions with the same signature
  (func $add (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )

  (func $sub (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )

  (func $mul (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.mul
  )

  ;; Initialize the table with function references
  (elem (i32.const 0) $add $sub $mul)

  ;; Function that calls through the table (indirect call)
  (func $call_operation (param $op_index i32) (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    local.get $op_index
    call_indirect (param i32 i32) (result i32)
  )

  ;; Type definition for function signature
  (type $binary_op (func (param i32 i32) (result i32)))

  ;; Export the caller function
  (export "call_operation" (func $call_operation))
  (export "table" (table 0))
)
