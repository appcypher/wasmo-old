<div align="center">
    <a href="#" target="_blank">
        <img src="media/wasmo.png" alt="Wasabi Logo" width="140" height="140"></img>
    </a>
</div>


<h1 align="center">WASMO</h1>

### GOALS
- Useable as a standalone WebAssembly runtime.
- Can serve as language backends compiling to WebAssembly.
- Provide options for taking advantage of the LLVM backend with JIT and AOT support.
- Provide options for increasing performance by turning off safety checks.
- Must be embedabble within other projects written in other languages.

### BUILDING THE PROJECT
- Stay tuned!

### USAGE
- Stay tuned!

### API (Susceptible to Change!)
```rust
// AOT
let module: ModuleAOT = Module::create_aot(&wasm_code);

let instance: InstanceAOT = module.instantiate(&imports);

instance.execute(&args)?;

// JIT
let module: Module = Module::create(&wasm_code);

let instance: Instance = module.instantiate(&imports)

instance.execute(&args)?;
```

### ATTRIBUTIONS
- [Inkwell](https://github.com/TheDan64/inkwell) [Apache-2.0] - Inkwell provides a type-safe interface for [llvm-sys](https://bitbucket.org/tari/llvm-sys.rs), and this project's llvm library is mostly based on it.
