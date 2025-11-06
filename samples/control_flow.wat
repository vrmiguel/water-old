;; Control flow example
;; Demonstrates if/else, loops, and blocks

(module
  ;; Simple if/else - returns absolute value
  (func $abs (param $x i32) (result i32)
    local.get $x
    i32.const 0
    i32.lt_s
    if (result i32)
      i32.const 0
      local.get $x
      i32.sub
    else
      local.get $x
    end
  )

  ;; Maximum of two numbers using if/else
  (func $max (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.gt_s
    if (result i32)
      local.get $a
    else
      local.get $b
    end
  )

  ;; Loop - count down from n to 0
  (func $countdown (param $n i32) (result i32)
    (local $i i32)
    local.get $n
    local.set $i

    block $exit
      loop $continue
        local.get $i
        i32.const 0
        i32.le_s
        br_if $exit

        local.get $i
        i32.const 1
        i32.sub
        local.set $i

        br $continue
      end
    end

    local.get $i
  )

  ;; Block with early exit
  (func $divide_safe (param $a i32) (param $b i32) (result i32)
    block $check
      local.get $b
      i32.const 0
      i32.eq
      br_if $check

      local.get $a
      local.get $b
      i32.div_s
      return
    end

    ;; Return 0 if divide by zero
    i32.const 0
  )

  ;; Export functions
  (export "abs" (func $abs))
  (export "max" (func $max))
  (export "countdown" (func $countdown))
  (export "divide_safe" (func $divide_safe))
)
