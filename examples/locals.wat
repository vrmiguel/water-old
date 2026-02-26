(module
  ;; Demonstrates local variables

  ;; Function with local variables
  (func $compute (param $x i32) (param $y i32) (result i32)
    (local $temp i32)
    (local $result i32)

    ;; temp = x + y
    local.get $x
    local.get $y
    i32.add
    local.set $temp

    ;; result = temp * 2
    local.get $temp
    i32.const 2
    i32.mul
    local.set $result

    ;; return result
    local.get $result
  )

  ;; Using local.tee (sets and returns value)
  (func $square_and_add (param $n i32) (result i32)
    (local $squared i32)

    ;; squared = n * n
    local.get $n
    local.get $n
    i32.mul
    local.tee $squared  ;; Sets $squared and keeps value on stack

    ;; Add original n to the result
    local.get $n
    i32.add
  )

  (export "compute" (func $compute))
  (export "square_and_add" (func $square_and_add))
)
