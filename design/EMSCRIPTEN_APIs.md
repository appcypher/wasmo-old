## EMSCRIPTEN HOST APIs
This document describes how Emscripten host APIs are created

### GENERATE EMSCRIPTEN HOST DATA
```rust
enum HostData {
    // Addresses and descriptions
    HostTable { ... },
    HostFunction { ... },
    ..
}

type EmscriptenHostData {
    imports_desc: ImportsDescription,
    data: Dict[String, Dict[String, HostData]],
}

fun init(imports_desc: ImportsDescription) -> EmscriptenHostData {
    // Create memories based on imports description
    // ...

    // Create tables based on imports description
    // ...

    // Add known globals
    // ...

    // Add known functions
    // ...
}

fun generate_imports(host_data: EmscriptenHostData) -> Imports {
    // Create imports out of host_data.data.
}
```

```rust
let imports

if api == HostAPI.Emscripten {
    // Contains necessary emcripten APIs needed by a wasm instance
    let emscripten_host_data = EmscriptenHostData(module.imports_desc)
    imports = emscripten_host_data.generate_imports()
}
```


### EMSCRIPTEN HOST FUNCTIONS AND ABI
```rust
// mkdir
fun ___syscall39(which: CInt, varargs: CInt, var instance: Instance) -> CInt {
    debug!("${___syscall39.name} ${which}")
    let { pathname, mode } = get_varargs!(varargs, instance, { @pathname, mode })
    unsafe { mkdir(pathname_addr, mode as _) }
}
```

### EMSCRIPTEN ACCESSING GUEST FUNCTIONS
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

