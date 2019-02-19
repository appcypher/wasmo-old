
### PIPELINE
```
MODULE
   |
   | compile
   |
MODULE —— IMPORTS
        |
        | instantiate + relocation
        |
    INSTANCE
```

### MODULE TYPE

```rust
// Shareable & Cloneable
// - Module can be shared by multiple instances.
// - Allows relocation at instantiation time or at runtime
derive!(Clone)
type Module {
  llvm_ir: LLVM?, // Disposable (ORC?)
  compiled_function_addrs?, // Addresses
  descs: ModuleDescriptions, // Cloneable
}
```

```rust
derive!(Clone)
type ModuleDecriptions {
    tables,
    mems,
    globals,
    functions,
    imports,
    exports,
}
```

### INSTANCE TYPE

```rust
// Cloneable
// - Each instance is meant to be run on its own thread
derive!(Clone)
type Instance {
    module, // A reference to the module
    offsets: Offsets,
}
```

```rust
type Offsets {
    memories,
    tables,
    globals,
}
```

### COMPILATION MODES
#### Regular
- Compile-time:
    * Generates module
    * Compiles wasm binary to LLVM IR
    * Compiles LLVM IR to machine code

- Instantiation-time:
    * Validates module
    * Relocates imported symbols

```rust
let options = Options { mode: Mode.Lazy }

let module = compile(wasm_binary, options)

let instance = instantiate(module)

instance.call("main") // `main` is already compiled
```

#### AOT
- Compile-time:
    * Generates module
    * Compiles wasm binary to LLVM IR
    * Compiles LLVM IR to machine code
    * Generates Object file for module
    * Optionally generates dylib file for ABIs

```rust
let options = Options { mode: Mode.AOT }

let module = compile(wasm_binary, options)

let wasm_exe = module.write_exe()

let libwasabi_dylib = module.write_dylib()
```

#### Lazy
- Compile-time:
    * Generates module
    * Compiles wasm binary to LLVM IR
    * Parts LLVM IR compiled to machine code
    * Functions not compiled
    * Clones module

- Instantiation-time:
    * Validates module
    * Relocates imported symbols

- Runtime:
    * Compiles function on first call

```rust
let options = Options { mode: Mode.Lazy }

let module = compile(wasm_binary, options)

let instance = instantiate(module)

instance.call("main") // `main` is lazy-compiled
```

#### REPL Lazy
- Compile-time:
    * Generates module
    * Compiles wasm binary to LLVM IR
    * Parts LLVM IR compiled to machine code
    * Functions not compiled
    * Clones module

- Instantiation-time:
    * Validates module
    * Relocates imported symbols

- Runtime:
    * Compiles function on first call
    * Takes new wasm binaries as input and adds to module
    * Validates newly-added binary and module
    * Compiles newly-added binary to LLVM IR

```rust
let options = Options { mode: Mode.REPL }

let module = compile(wasm_binary, options)

let instance = instantiate(module)

instance.call("main") // `main` is lazy-compiled

instance.add_function(wasm_binary_2) // Validates newly-added function

instance.call("func") // `func` is lazy-compiled
```

---------

#### PGO
- Uses any of Normal, Lazy JIT and REPL Lazy JIT but the compiler lingers around and keeps certain profiling info for optimizing code further and hot-swapping code.
