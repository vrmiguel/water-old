;; Nested instructions example
;; Demonstrates complex nested instruction patterns

(module
  ;; Simple nested arithmetic
  (func $nested_calc (param $x i32) (result i32)
    (i32.add
      (i32.mul
        (local.get $x)
        (i32.const 5)
      )
      (i32.const 10)
    )
  )

  ;; Nested with local.set
  (func $complex_nested (param $a i32) (param $b i32) (result i32)
    (local $temp i32)
    (local.set $temp
      (i32.add
        (local.get $a)
        (local.get $b)
      )
    )
    (i32.mul
      (local.get $temp)
      (i32.const 2)
    )
  )

  ;; Nested if expressions
  (func $nested_if (param $x i32) (result i32)
    (if (result i32)
      (i32.gt_s (local.get $x) (i32.const 10))
      (then
        (if (result i32)
          (i32.lt_s (local.get $x) (i32.const 20))
          (then
            (i32.const 1)
          )
          (else
            (i32.const 2)
          )
        )
      )
      (else
        (i32.const 0)
      )
    )
  )

  ;; Complex expression with multiple operations
  (func $expression (param $a i32) (param $b i32) (param $c i32) (result i32)
    (i32.add
      (i32.mul
        (i32.sub
          (local.get $a)
          (local.get $b)
        )
        (i32.const 3)
      )
      (i32.div_s
        (local.get $c)
        (i32.const 2)
      )
    )
  )

  ;; Export functions
  (export "nested_calc" (func $nested_calc))
  (export "complex_nested" (func $complex_nested))
  (export "nested_if" (func $nested_if))
  (export "expression" (func $expression))
)
