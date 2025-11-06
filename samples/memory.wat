;; Memory operations example
;; Demonstrates linear memory and load/store operations

(module
  ;; Define 1 page of memory (64KB)
  (memory 1)

  ;; Store a value in memory
  (func $store_value (param $offset i32) (param $value i32)
    local.get $offset
    local.get $value
    i32.store
  )

  ;; Load a value from memory
  (func $load_value (param $offset i32) (result i32)
    local.get $offset
    i32.load
  )

  ;; Store a byte in memory
  (func $store_byte (param $offset i32) (param $value i32)
    local.get $offset
    local.get $value
    i32.store8
  )

  ;; Load a byte from memory
  (func $load_byte (param $offset i32) (result i32)
    local.get $offset
    i32.load8_u
  )

  ;; Get current memory size in pages
  (func $get_memory_size (result i32)
    memory.size
  )

  ;; Grow memory by specified number of pages
  (func $grow_memory (param $pages i32) (result i32)
    local.get $pages
    memory.grow
  )

  ;; Export memory and functions
  (export "memory" (memory 0))
  (export "store_value" (func $store_value))
  (export "load_value" (func $load_value))
  (export "store_byte" (func $store_byte))
  (export "load_byte" (func $load_byte))
  (export "get_memory_size" (func $get_memory_size))
  (export "grow_memory" (func $grow_memory))
)
