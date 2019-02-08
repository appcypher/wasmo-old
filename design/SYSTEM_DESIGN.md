

<div align="center">
<img src="https://github.com/appcypher/wasmlite/blob/design/design/media/MEMORY_MODEL.png" alt="Design" width="473" height="886"></img>
</div>

### DESIGN DIAGRAM
```
MODULE
   |
   | compile
   |
MODULE —— IMPORTS
        |
        | instantiate + resolution
        |
    INSTANCE
```

### MODULE TYPE

```rust
// Shareable
// tramps for fixing links to import functions at instantiation time or at
// runtime
type Module {
  llvm_ir: LLVMModule?,
  // refs to compiled functions in memory
  // Not relevant for AOT
  compiled_function_refs?,
  descriptions: ModuleDescriptions,
}

derive!(Clone)
type ModuleDecriptions {
    tables_desc,
    mems_desc,
    globals_desc,
    function_desc,
    imports_desc,
    exports_desc,
}
```

### INSTANCE TYPE

```rust
// Cloneable
// Each instance is meant to be run on its own thread
// `drop_llvm_module` function for dropping the llvm module when no longer needed
derive!(Clone)
type Instance {
    module, // A copy of module
    offsets: Offsets,
}

type Offsets {
    memories,
    tables,
    globals,
}
```

### OTHER COMPILATION STRATEGIES
#### AOT
- Generates module,
- During compilation, creates an in-memory object file represnting the wasm program with unresolved links to imports.
- Can generate dylib for EmscriptenHostData.
- Instantiation phase is unecessary here.

#### LAZY JIT
- Generates module; nothing is compiled.
- During instantiation, does validation only.
- At runtime. The instance calls a `module.compile_llvm_func("func_name")` that compiles the code, fix the tramps and return the function address.

#### REPL LAZY JIT
- Generates module; nothing is compiled.
- During instantiation, does validation only.
- At runtime. The instance calls a `module.compile_wasm_func(binary)` that takes a wasm_binary, generates the llvm_ir, optionally validate it, compiles the code, fix the tramps and return the function address.
