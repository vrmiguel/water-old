;; Arithmetic operations example
;; Demonstrates basic integer arithmetic in WebAssembly

(module
  ;; Add two i32 numbers
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )

  ;; Subtract two i32 numbers
  (func $sub (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.sub
  )

  ;; Multiply two i32 numbers
  (func $mul (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.mul
  )

  ;; Divide two i32 numbers (signed)
  (func $div (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.div_s
  )

  ;; Export functions for external use
  (export "add" (func $add))
  (export "sub" (func $sub))
  (export "mul" (func $mul))
  (export "div" (func $div))
)
