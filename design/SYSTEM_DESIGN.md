

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
type Module {
  llvm_ir: LLVMModule?,
  compiled_functions?,
  descriptions: ModuleDescription,
}

type ModuleDecription {
    tables_desc,
    mems_desc,
    globals_desc,
    function_desc,
    imports_desc,
    exports_desc,
}
```
