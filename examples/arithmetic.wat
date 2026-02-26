(module
  ;; Demonstrates various arithmetic operations

  ;; Integer addition
  (func $add_i32 (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )

  ;; Integer subtraction
  (func $sub_i32 (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.sub
  )

  ;; Integer multiplication
  (func $mul_i32 (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.mul
  )

  ;; Signed integer division
  (func $div_s_i32 (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.div_s
  )

  ;; Float addition
  (func $add_f64 (param f64 f64) (result f64)
    local.get 0
    local.get 1
    f64.add
  )

  ;; Export functions
  (export "add_i32" (func $add_i32))
  (export "sub_i32" (func $sub_i32))
  (export "mul_i32" (func $mul_i32))
  (export "div_s_i32" (func $div_s_i32))
  (export "add_f64" (func $add_f64))
)
