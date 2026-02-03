(module
  ;; A simple function that adds two i32 numbers
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )

  ;; Export the function so it can be called from JavaScript
  (export "add" (func $add))
)
