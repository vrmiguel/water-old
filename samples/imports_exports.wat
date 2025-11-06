;; Imports and exports example
;; Demonstrates importing functions from the host environment
;; and exporting functions and other module items

(module
  ;; Import a logging function from the JavaScript console
  (import "console" "log" (func $log (param i32)))

  ;; Import another function with multiple parameters
  (import "env" "external_func" (func $external (param i32) (param i32) (result i32)))

  ;; Import memory from the host
  (import "js" "mem" (memory 1))

  ;; Import a global variable
  (import "env" "global_val" (global $imported_global i32))

  ;; Define a global variable for export
  (global $counter (mut i32) (i32.const 0))

  ;; Function that uses imported function
  (func $log_value (param $x i32)
    local.get $x
    call $log
  )

  ;; Function that increments counter and returns new value
  (func $increment (result i32)
    global.get $counter
    i32.const 1
    i32.add
    global.set $counter
    global.get $counter
  )

  ;; Function that uses external function
  (func $use_external (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    call $external
  )

  ;; Function that accesses imported global
  (func $get_imported_global (result i32)
    global.get $imported_global
  )

  ;; Export functions
  (export "log_value" (func $log_value))
  (export "increment" (func $increment))
  (export "use_external" (func $use_external))
  (export "get_imported_global" (func $get_imported_global))

  ;; Export the mutable global
  (export "counter" (global $counter))
)
