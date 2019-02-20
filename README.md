
# WASMLITE
This project aims to provide the necessary tools for compiling wasm binary to LLVM IR which can further be compiled to machine-specific code.

This project is supposed to make it easy for languages compiling to wasm to take advantage of the LLVM infrastructure.

It also allows stripping of expensive wasm runtime elements for times when full wasm specification is not desired.

Lastly, it is a platform for learning about WebAssembly, LLVM and how they play well together.

### POSSIBLE API (NOT FINAL)
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    optimization_level: 3,
    exclude_passes: vec![
        LLVMPass::InstCombine,
    ],
    runtime_ignores: vec![
        RuntimeProperty::SignatureChecks,
        RuntimeProperty::MemoryBoundsChecks,
    ],
    strategy: CompilationStrategy::Normal,
});

// Create wasm options.
let options = Some(InstanceOptions {
    compiler_flags,
    abi: vec![ABI::Wasabi],
});

// JIT compile module in current process.
let (module, instance) = Runtime::instantiate(wasm_binary, imports, options);

// Get the exported main function from instance.
let main = instance.get_func("main");

// Store array of items in wasm memory 0
let wasm_array = instance.set_array(&arguments);

// Call the function.
main.call(5, wasm_array);
```

### NON_GOAL
- Have multiple backends

### CURRENT SUPPORT
- [ ] Parser
    - [x] preamble
    - [x] types
    - [x] imports
    - [x] local memories, tables, globals
    - [x] elems, data, globals
    - [ ] functions body
    - [x] exports (functions, tables, globals)
    - [ ] validation
    - [ ] exhaustive unit tests and fuzz tests

- [ ] Codegen
    - [ ] basic llvm codegen
    - [ ] basic llvm traps and checks
    - [ ] basic AOT and Regular compilation
    - [ ] basic memory and table impl
    - [ ] basic table/call_indirect impl
    - [ ] llvm codegen completion

- [ ] Runtime
- [ ] Compilation Strategies
- [ ] Compilation Flags
- [ ] An ABI
- [ ] Other Features

### TODO
- wasm32 to LLVMIR to machine code
- fuzz tests
- unittests
- validation (utf8 [Unicode Standard 11.0, Section 3.9, Table 3-7. Well-Formed UTF-8 Byte Sequences] and semantics)

    - https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
    - https://webassembly.github.io/spec/core/binary/values.html#names

- error messages and making error position point of error instead of start_position

### ATTRIBUTIONS
- [inkwell](https://github.com/TheDan64/inkwell) - inspired the LLVM codegen section
- [wasmer](https://github.com/wasmerio/wasmer) - takes some design cues from this similar project
- [wasmtime](https://github.com/CraneStation/wasmtime) - takes some design cues from this similar project as well
