## PIPELINE
```
MODULE
   |
   | compile
   |
MODULE —— IMPORTS
        |
        | instantiation (memory alloc & relocation)
        |
    MODULE
```


### OVERVIEW
COMPILE:

    JIT Eager:
        * Compiles to code with unresolved external symbols
        * Populate InstanceData with local func addrs

    JIT Lazy:
        * Does nothing. Functions are compiled on first call

    AOT:
        * Compiles to code with dynamic calls to external symbols
        * Outputs to object file


INSTANTIATION:

    JIT Eager:
        * Takes in imports
        * Creates local memories, tables, globals
        * Populate InstanceData with local tables, mems and globals addrs
        * Populate InstanceData with imported func, tables, mems and globals addrs
        * Relocate refs to external symbols
        * Drop llvm module

    JIT Lazy:
        * Takes in imports
        * Creates local memories, tables, globals
        * Populate InstanceData with local tables, mems and globals addrs
        * Populate InstanceData with imported func, tables, mems and globals addrs
        * Apply external symbols address relocation (ORC's job)
        * Drop llvm module

RUNTIME:

    JIT Lazy:
        * Takes in function request. Check signature match
        * Tells llvm world which function to compile. Phase is skipped if already compiled
        * Gets addr to symbol
        * Apply external symbols address relocation (ORC's job)
        * Also on function request through `get_func`, check signature match

    JIT Eval:
        * Takes in wasm module
        * Validate and generates wasm IR
        * Generates llvm code and adds to llvm Module
        * Tells llvm world which function to compile. Phase is skipped if already compiled
        * Gets addr to symbol
        * Apply external symbols address relocation (ORC's job)
        * Also on function request through `get_func`, check signature match

    PGO:
        * ...


### MODULE
```rust
struct Module {
    data: InstanceData?, // Disposable
    desc: ModuleDesc,
    llvm_ir: llvm.Module?, // Diposable // DEP:aot_compile // DEP:runtime_lazy_compile
}
```

```rust
/// Filled at compilation and instantiation time
struct InstanceData { // DEP:imported_funcs // DEP:local_funcs // DEP:instance.get_func
    tables: *const TablePtr, // imports | locals
    memories: *mut MemoryPtr, // imports | locals
    globals: *mut GlobalPtr, // imports | locals // ptr to ptrs to global value
    functions: *mut FuncPtr, // imports | locals
}
```

```rust
struct ModuleDesc { // DEP:generate_imports // DEP:instance.get_func
    local: Locals,
    imports: Imports,
    exports: Exports,
}
```

```rust
struct Locals { // All sorted by declaration order
    tables: Vec<TableDesc>,
    memories: Vec<MemDesc>,
    globals: Vec<GlobalDesc>,
    functions: Vec<FuncDesc>,
}
```

```rust
struct TableDesc {
    minimum: u32,
    maximum: u32,
    addr: Ptr, // ptr to InstanceData
}
```

### IMPORTS
```rust
type Imports = Hashmap<String, Hashmap<String, Desc>>;  // DEP:generate_imports
```

```rust
enum Desc {
    TableDesc(TableDesc),
    MemDesc(MemDesc),
    GlobalDesc(GlobalDesc),
    FuncDesc(FuncDesc),
}
```

### EXPORTS
```rust
/// Filled at compilation and instantiation time
/// ##### Implementation
/// - TODO: Turn structure into a vector sorted by names and use bin search for lookup. Benchmark against current approaach.
type Exports = Hashmap<String, Desc>; // DEP:instance.get_func
```

### EXAMPLE
```rust
let module = Module::compile(&wasm_code, &options);

let instance = module.instantiate(imports);

let main: Func<(i32, i32), i32> = instance.get_func("main");

let result = main.call(1, 2);
```

-------------------------------------------------------------------

## THREADS
* ...

-------------------------------------------------------------------

## TODO
MEMORY MANAGEMENT
- A way to free the `reference` Module after instantiation unless it has been referenced elsewhere
- Freeing the Instance-owned module for non-lazy modes.
