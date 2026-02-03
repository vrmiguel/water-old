(module
  ;; Demonstrates comparison operations

  ;; Check if two numbers are equal
  (func $equals (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.eq
  )

  ;; Check if first is less than second (signed)
  (func $less_than (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.lt_s
  )

  ;; Check if first is greater than second (signed)
  (func $greater_than (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.gt_s
  )

  ;; Check if first is less than or equal to second (signed)
  (func $less_or_equal (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.le_s
  )

  ;; Float comparison
  (func $f64_greater (param f64 f64) (result i32)
    local.get 0
    local.get 1
    f64.gt
  )

  ;; Export functions
  (export "equals" (func $equals))
  (export "less_than" (func $less_than))
  (export "greater_than" (func $greater_than))
  (export "less_or_equal" (func $less_or_equal))
  (export "f64_greater" (func $f64_greater))
)
