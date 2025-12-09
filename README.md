# water (WebAssembly TExt foRmat compiler)

`water` aims to be a tiny and performant WebAssembly Text Format compiler.

## Example

Here's a simple example of a WebAssembly Text Format function that adds two numbers:

```wat
(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  (export "add" (func $add))
)
```

This module defines an `add` function that takes two 32-bit integers and returns their sum.

