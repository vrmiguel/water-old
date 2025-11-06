;; Function definitions example
;; Demonstrates various function definition patterns and local variables

(module
  ;; Simple function with no parameters or return value
  (func $noop)

  ;; Function with parameters
  (func $square (param $x i32) (result i32)
    local.get $x
    local.get $x
    i32.mul
  )

  ;; Function with local variables
  (func $sum_of_squares (param $a i32) (param $b i32) (result i32)
    (local $a_squared i32)
    (local $b_squared i32)

    ;; Calculate a squared
    local.get $a
    local.get $a
    i32.mul
    local.set $a_squared

    ;; Calculate b squared
    local.get $b
    local.get $b
    i32.mul
    local.set $b_squared

    ;; Return sum
    local.get $a_squared
    local.get $b_squared
    i32.add
  )

  ;; Function calling another function
  (func $factorial (param $n i32) (result i32)
    (local $result i32)
    (local $i i32)

    ;; Initialize result to 1
    i32.const 1
    local.set $result

    ;; Initialize counter to 1
    i32.const 1
    local.set $i

    ;; Loop would go here (simplified for now)
    local.get $result
  )

  ;; Export functions
  (export "square" (func $square))
  (export "sum_of_squares" (func $sum_of_squares))
)
