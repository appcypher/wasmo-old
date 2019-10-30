<div align="center">
    <a href="#" target="_blank">
        <img src="media/wasmo.png" alt="Wasabi Logo" width="140" height="140"></img>
    </a>
</div>


<h2 align="center">WASMO</h2>

--------------

### <sup><sup>🚀</sup></sup> GOALS
- Useable as a standalone WebAssembly runtime.
- Can serve as a backend for languages compiling to WebAssembly.
- Provide options for taking advantage of the LLVM backend with JIT and AOT support.
- Provide options for increasing performance by turning off safety checks.
- Must be embedabble within projects written in other languages.

--------------

### <sup><sup>🛠</sup></sup> BUILDING THE PROJECT
#### REQUIREMENTS
  - Rust and Cargo

    Rust and Cargo can be installed by following the instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

  - LLVM 8.0.1

    You can download an LLVM installer for Windows or pre-compiled binaries for your Unix platform [here](https://github.com/llvm/llvm-project/releases/tag/llvmorg-8.0.1)


#### STEPS
  - Clone the repository.

    ```
    git clone https://github.com/appcypher/wasmo.git
    ```

  - Change directory

    ```
    cd wasmo
    ```

  - Create an `LLVM_SYS_80_PREFIX` environment variable and set its value to your installed LLVM path. The syntax for this depends on the shell you are using

    <details>
      <summary>Read more</summary>

      - POSIX (BASH, ZSH, ...)

        ```
        export LLVM_SYS_80_PREFIX="/path/to/llvm"
        ```

      - FISH

        ```
        setenv LLVM_SYS_80_PREFIX "/path/to/llvm"
        ```

      - CMD [WINDOWS]

        ```
        set LLVM_SYS_80_PREFIX="/path/to/llvm"
        ```

      - POWERSHELL [WINDOWS]

        ```
        setx LLVM_SYS_80_PREFIX "/path/to/llvm"
        ```

      </details>

  - Build the project

    ```
    cargo build
    ```

  - Run wasmo executable

    ```
    target/debug/wasmo --help
    ```

--------------

### <sup><sup>▶️</sup></sup> USAGE
- Run a WebAssembly file _<sup><sup>WIP<sup></sup>_

  ```
  target/debug/wasmo sample.wasm
  ```

- Print help messages

  ```
  target/debug/wasmo --help
  ```

--------------

### <sup><sup>↔️</sup></sup> API _<sup><sup>WIP<sup></sup>_
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

--------------

### <sup><sup>👍</sup></sup> ATTRIBUTIONS
- [Inkwell](https://github.com/TheDan64/inkwell) [Apache-2.0] - Inkwell provides a type-safe interface for [llvm-sys](https://bitbucket.org/tari/llvm-sys.rs), and this project's llvm library is mostly based on it.
