## HOST DATA AND IMPORTS
This document describes how Emscripten host APIs are created

### GENERATE EMSCRIPTEN IMPORTS
##### IMPORTS
```kotlin
type Imports = Dict[String, Dict[String, HostData]]
```

##### HOST DATA
```kotlin
enum HostData {
    HostTable { addr, desc },
    HostFunction { addr, desc },
    ..
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
fun ___syscall39(which: CInt, varargs: CInt, var instance: Instance) -> CInt {
    debug!("${___syscall39.name} ${which}")

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

// get_memory_addr_0
inline!()
fun get_memory_addr_0(var instance: Instance) -> *U8 {
    instance.get_memory_offset(0)
}

// write_to_buf
fun write_to_buf(str: *c_char, buf: U32, max: U32, var instance: Instance) -> CInt {
    unsafe {
        let buf_addr = get_memory_addr(buf, instance) as _;

        for i in 0..max {
            *buf_addr.offset(i) = *str.offset(i)
        }

        buf
    }
}
```

### HOW IMPORTS WORK
- instantiate needs imports
- emscripten host data uses module's imports description to generate some imports
- some emscripten host data (functions) reference instance guest data
- imports generated from emscripten host data
- instantiate use generated imports
