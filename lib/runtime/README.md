
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
    INSTANCE
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

