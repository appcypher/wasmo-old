
# WASM TO LLVM
This project aims to provide the necessary tools for compiling wasm binary to LLVM IR which can further be compiled to machine-specific code.

This project is supposed to make it easy for languages compiling to wasm to take advantage of the LLVM infrastructure as well as get the benefit of useful host APIs like Emscripten.

It also allows stripping of expensive wasm runtime elements for times when full wasm specification is not desired.

Lastly, this project is also a platform for learning more about wasm and LLVM.

### PROPOSED API
#### COMPLETE EXAMPLE
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    optimization_level: 3,
    exclude_passes: vec![
        LLVMPass::InstCombine,
    ],
    runtime_ignores: vec![
        RuntimeProperty::SignatureChecks,
        RuntimeProperty::TableChecks,
    ],
});

// Create wasm instance options.
let instance_options = Some(InstanceOptions {
    compiler_flags,
    host_apis: vec![HostAPI::Emscripten],
});

// JIT compile module in current process.
let (module, instance) = Runtime::instantiate(wasm_binary, imports, instance_options);

// Get the exported main function from instance.
let main = instance.get_func("main");

// Store array of items in wasm memory 0
let wasm_array = instance.set_array(&arguments);

// Call the function.
main.call(5, wasm_array);
```

#### COMPILATION TYPE
```rust
// instance holds an in-memory machine code of the entire wasm program.
let (module, instance) = Runtime::aot_compile(wasm_binary, imports, instance_options);

// Create executables.
let (imports_dylib, wasm_exe) = instance.create_executables();
```

### NOT CURRENTLY SUPPORTED
- wasm64

### NON_GOAL
- Have multiple backends

### STRATEGY
- Single-pass parsing, validation and codegen from wasm binary to LLVM IR

### CURRENT SUPPORT
- preamble
- types
- imports
- functions

### TODO
- wasm32 to LLVMIR
- fuzz tests
- unittests
- validation (utf8 [Unicode Standard 11.0, Section 3.9, Table 3-7. Well-Formed UTF-8 Byte Sequences] and semantics)

    - https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
    - https://webassembly.github.io/spec/core/binary/values.html#names

- error messages and making error position point of error instead of start_position

### POSSIBLE FUTURE ADDITIONS
- Lazy compilation
- Interpreter
- AOT compilation

### PROCESS ADDRESS SPACE (LINUX x86-64 EXAMPLE)
```
+++++++++++++++++++++++++++
|         STACK
+++++++++++++++++++++++++++
|    SHARED LIBRARIES
+++++++++++++++++++++++++++
|          HEAP
|
|
|  ++++++ INSTANCE ++++++
|  ++++++++++++++++++++++
|  |                    |
|  |    LINEAR MEMORY   | RW
|  |                    |
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |      GLOBALS       | R
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |       TABLES       | R
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |        CODE        | E
|  ++++++++++++++++++++++
|
+++++++++++++++++++++++++++
|      BSS SEGMENT
+++++++++++++++++++++++++++
|    (RO)DATA SEGMENT
+++++++++++++++++++++++++++
|      TEXT SEGMENT
+++++++++++++++++++++++++++
```

# ENCODING
## LEB128
#### UNSIGNED
```
MSB ------------------ LSB
00001001 10000111 01100101  In raw binary
 0100110  0001110  1100101  Padded to a multiple of 7 bits
00100110 10001110 11100101  Add high 1 bits on all but last (most significant) group to form bytes

Starting with the first byte, get rid of the msb bit
11100101 -> 11001010 << 1
11001010 -> 01100101 >> 1

Widen to appropriate type
01100101 -> 00000000 01100101 as u32

For the next byte, get rid of the msb bit
10001110 -> 00011100 << 1
00011100 -> 00001110 >> 1

Widen to appropriate type
00001110 -> 00000000 00001110 as u32

Shift by 7 bits to the left
00000000 00001110 -> 00000111 00000000 << 7

Or the value with the previous result
00000111 00000000 | 00000000 01100101

And so on. Basically you shift by a multiple of 7

if byte's msb is unset, you can break the loop
```
#### SIGNED
```
MSB ------------------ LSB
00000110 01111000 10011011  In raw two's complement binary
 1011001  1110001  0011011  Sign extended to a multiple of 7 bits
01011001 11110001 10011011  Add high 1 bits on all but last (most significant) group to form bytes

Starting with the first byte, get rid of the msb bit
10011011 -> 00110110 << 1
00110110 -> 00011011 >> 1

Widen to appropriate type
01100101 -> 00000000 01100101 as u32

For the next byte, get rid of the msb bit
10001110 -> 00011100 << 1
00011100 -> 00001110 >> 1

Widen to appropriate type
00001110 -> 00000000 00001110 as u32

Shift by 7 bits to the left
00000000 00001110 -> 00000111 00000000 << 7

Or the value with the previous result
00000111 00000000 | 00000000 01100101

And so on. Basically you shift by a multiple of 7

Decoding a signed LEB128 encoding has an extra twist, we need to extend the sign bit

If the value is signed, then the msb is set

if result & 0x0100_0000 == 0x0100_0000 {
    result |= !(0x1 << encoding_size)
}

if byte's msb is unset, you can break the loop
```

### WELL-FORMED UTF-8 BYTE SEQUENCES
Based on Unicode Standard 11.0, Section 3.9, Table 3-7.

| Code Points        | First Byte   | Second Byte    | Third Byte    | Fourth Byte   |
|:-------------------|:-------------|:---------------|:--------------|:--------------|
| U+0000..U+007F     | 00..7F       |                |               |               |
| U+0080..U+07FF     | C2..DF       | 80..BF         |               |               |
| U+0800..U+0FFF     | E0           | A0..BF         | 80..BF        |               |
| U+1000..U+CFFF     | E1..EC       | 80..BF         | 80..BF        |               |
| U+D000..U+D7FF     | ED           | 80..9F         | 80..BF        |               |
| U+E000..U+FFFF     | EE..EF       | 80..BF         | 80..BF        |               |
| U+10000..U+3FFFF   | F0           | 90..BF         | 80..BF        | 80..BF        |
| U+40000..U+FFFFF   | F1..F3       | 80..BF         | 80..BF        | 80..BF        |
| U+100000..U+10FFFF | F4           | 80..8F         | 80..BF        | 80..BF        |

---------------------------------------------------

let mut parser = Parser::new(&code, &module); // ModuleEnvironment
