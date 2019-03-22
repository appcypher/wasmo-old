
## TODO
MEMORY MANAGEMENT
- A way to free the `reference` Module after instantiation unless it has been referenced elsewhere
- Freeing the Instance-owned module for non-lazy modes.

-------------------------------------------------------------------


_Pseudocode in Astro for clarity_

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

## SYSTEM
##### MODULE
```rust
// Shareable & Cloneable
// - Module can be shared by multiple instances.
// - Allows relocation at instantiation time or at runtime
// - llvm.Module sorts members by types. e.g. imports are sorted by types.
// - things are not ordered according to the wasm module intead they are ordered based on how the runtime frequently access them
type Module {
  llvm_ir: llvm.Module?, // Disposable (ORC?)
  exports: Exports,
  desc: ModuleDesc, // Cloneable
}
```

##### EXPORTS
```kotlin
type Exports = (GuestTables, GuestFunctions, ...)

type GuestTables = [(String, Ptr, TableDesc)]
```

##### MODULE DESCRIPTION
```rust
type ModuleDesc {
    tables, // Don't contain imports
    memories, // Don't contain imports
    globals, // Don't contain imports
    functions, // Don't contain imports
    imports,
    exports,
}
```

```kotlin
type ImportDesc = (TableDescs, FunctionDescs, ...)

type TableDescs = [(String, String, TableDesc)]
```


##### INSTANCE
```rust
// Cloneable
// - Each instance is meant to be run on its own thread
derive!(Clone)
repr!(C)
type Instance {
    data: Data,
    descs: InstanceDescriptions,
    module: llvm.Module,
}
```

##### DATA
```rust
type Data { // mmap'd / VirtualAlloc'd
    memories: **UInt8,
    tables: *Seq[UInt32],
    globals: *UInt64,
}
```

##### INSTANCE DESCRIPTIONS
```rust
type InstanceDescriptions {
    tables,
    memories,
    globals,
}
```

### PIPELINE
- Compile-time:
    * Compilation of llvm.Module to machine code
    * Adding import tramps

- Instantiation-time:
    * Import tramps are resolved

- Runtime:
    * Indirect call signature check



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

#### PGO
- The compiler lingers around and keeps profiling info about the code that can be used to optimize and hotswap code later.



-------------------------------------------------------------------



## IMPORTS AND ABI
##### IMPORTS
```kotlin
type Imports = (HostTables, HostFunctions, ...)

enum HostTables {
    tables: [(String, String, Ptr, TableDesc)]
}
```

##### HOST ABI
```kotlin
enum HostABI {
    Emscripten,
    Wasabi,
}
```

##### HOST IMPORTS GENERATION
```kotlin
fun create_imports(abi: HostABI, import_descs: ImportDescriptions) -> Imports {
    // Create memories based on imports description
    // ...

    // Create tables based on imports description
    // ...

    // Add known globals
    // ...

    // Add known functions
    // ...
}
```

##### USAGE
```swift
let module = compile(wasm_binary, options)

let emscripten_imports = create_imports(HostABI.Emscripten, module.descs.imports)

let instance = instantiate(module, emscripten_imports) // Addresses are cloned
```


### HOST FUNCTIONS AND ABI
```rust
// mkdir
fun syscall(which: CInt, varargs: CInt, var instance: Instance) -> CInt {
    let { pathname, mode } = get_varargs!(varargs, instance, { @pathname, mode })

    unsafe {
        mkdir(pathname_addr, mode as _)
    }
}
```

### ACCESSING GUEST EXPORTS
Performance can be improved with host binding implementation.

```rust
// copy_cstr_into_instance
fun copy_cstr_into_instance(str: *CChar, var instance: Instance) -> U32 {
    unsafe {
        let str_len = str.length()

        // Accessing guest functions.
        // ---> Abstract
        let malloc = instance.get_func_addr("malloc") as (CInt, CInt) -> U32
        let space_offset = malloc(str_len as _, instance)
        // <--- Abstract

        var space_addr = get_memory_addr(space_offset, instance) as _;
        var space_addr_slice = get_addr_slice(space_addr, str_len)

        for (loc, byte) in zip(space_addr_slice, str[:]) {
            *loc = byte
        }

        space_offset
    }
}
```

```kotlin
// get_memory_addr_0
fun get_memory_addr_0(var instance: Instance) -> *U8 {
    instance.get_memory_offset(0)
}
```

```rust
// write_to_buf
fun write_to_buf(str: *CChar, buf: U32, max: U32, var instance: Instance) -> CInt {
    unsafe {
        let buf_addr = get_memory_addr(buf, instance) as _;

        for i in 0..max {
            *buf_addr.offset(i) = *str.offset(i)
        }

        buf
    }
}
```

### HOW ABI IMPORTS WORK
- instantiate needs imports
- host uses module's imports description to generate some imports
- host functions reference instance guest data
- abi imports are generated
- instantiate use generated imports



-------------------------------------------------------------------


## CACHING
* Module caching (serde and llvm.Module serialization to bitcode)
* Object file and dylib creation

-------------------------------------------------------------------

## THREADS
* ...

-------------------------------------------------------------------

<h1>REDESIGN<h1>


### PROCESS
COMPILE:

    ORC Normal:
        * Compiles to code with unresolved external symbols
        * Populate InstanceData with local func addrs

    ORC Lazy:
        * Does nothing. Functions are called on first call

    AOT/No Orc:
        * Compiles to code with dynamic calls to external symbols
        * Outputs to object file


INSTANTIATION:

    ORC Normal:
        * Takes in imports
        * Creates local memories, tables, globals
        * Populate InstanceData with local tables, mems and globals addrs
        * Populate InstanceData with imported func, tables, mems and globals addrs
        * Relocate refs to external symbols
        * Drop llvm module

    ORC Lazy:
        * Takes in imports
        * Creates local memories, tables, globals
        * Populate InstanceData with local tables, mems and globals addrs
        * Populate InstanceData with imported func, tables, mems and globals addrs
        * Apply external symbols address relocation (ORC's job)
        * Drop llvm module

RUNTIME:

    ORC Normal:
        * On function request through `get_func`, check signature match

    ORC Lazy:
        * Takes in wasm IR, generate llvm code and adds to llvm Module (Lazy REPL)
        * Tell llvm world which function to compile. Phase is skipped if already compiled
        * Gets addr to symbol
        * Apply external symbols address relocation (ORC's job)
        * Also on function request through `get_func`, check signature match


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
