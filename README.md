
# WASMLITE (WASM TO LLVM)
This project aims to provide the necessary tools for compiling wasm binary to LLVM IR which can further be compiled to machine-specific code.

This project is supposed to make it easy for languages compiling to wasm to take advantage of the LLVM infrastructure as well as get the benefit of useful host APIs like Emscripten.

It also allows stripping of expensive wasm runtime elements for times when full wasm specification is not desired.

Lastly, it is a platform for learning about WebAssmbly, LLVM and how they can play well together.

### POSSIBLE API (NOT FINAL)
#### COMPILATION PIPELINE
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

// Create wasm instance options.
let instance_options = Some(InstanceOptions {
    compiler_flags,
    host_apis: vec![HostAPI::Emscripten], // Problematic!
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

#### COMPILATION TYPES
##### AOT COMPILATIONS
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    strategy: CompilationStrategy::AheadOfTime,
    ..
});

// Create wasm instance options.
let instance_options = Some(InstanceOptions { compiler_flags, .. });

// instance holds an in-memory object code of the entire wasm program.
// Possibly generates a dylib for Emscripten APIs as well.
let (module, instantiate) = Runtime::instantiate(wasm_binary, imports, instance_options);

// Create executables.
let (imports_dylib, wasm_exe) = module.create_executables();
```

##### LAZY COMPILATION
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    strategy: CompilationStrategy::LazyCompilation,
    ..
});

// Create wasm instance options.
let instance_options = Some(InstanceOptions { compiler_flags, .. });

// Functions are not compiled until their first call.
let (module, instance) = Runtime::instantiate(wasm_binary, imports, instance_options);
```

##### REPL-TYPE LAZY COMPILATION
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    strategy: CompilationStrategy::REPL,
    ..
});

// Create wasm instance options.
let instance_options = Some(InstanceOptions { compiler_flags, .. });

// Lazily compiles the entire wasm instance.
let (module, instance) = Runtime::instantiate(wasm_binary, imports, instance_options);

// ???
let func = module.add_function(wasm_function_binary, instance);
let expression = module.add_expression(wasm_expression_binary, instance);
```

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
- wasm32 to LLVMIR to machine code
- fuzz tests
- unittests
- validation (utf8 [Unicode Standard 11.0, Section 3.9, Table 3-7. Well-Formed UTF-8 Byte Sequences] and semantics)

    - https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
    - https://webassembly.github.io/spec/core/binary/values.html#names

- error messages and making error position point of error instead of start_position
