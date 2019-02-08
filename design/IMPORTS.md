## IMPORTS
Imports basically hold raw memory addresses and their descriptions

```rust
enum HostData {
    // Addresses and descriptions
    HostTable { ... },
    HostFunction { ... },
    ..
}

type Imports {
    data: Dict[String, Dict[String, ImportData]]
}

enum ImportData {
    Table { addr, desc },
}

```

### HOW IMPORTS WORK
- instantiate needs imports
- emscripten host data uses module's imports description to generate some imports
- some emscripten host data (functions) reference instance guest data
- imports generated from emscripten host data
- instantiate use generated imports
