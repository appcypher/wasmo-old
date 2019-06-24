<div align="center">
    <a href="#" target="_blank">
        <img src="media/wasmo.png" alt="Wasabi Logo" width="140" height="140"></img>
    </a>
</div>


<h1 align="center">WASMO</h1>


This project aims to provide the necessary tools for compiling wasm binary to LLVM IR which can further be compiled to machine-specific code.

This project is supposed to make it easy for languages compiling to wasm to take advantage of the LLVM infrastructure.

It also allows stripping of expensive wasm runtime elements for times when complete specification compliance is not desired.

Lastly, it is a platform for learning about WebAssembly, LLVM and how they play well together, therefore it is designed to be approachable. Documentation is very important.

### BUILDING THE PROJECT

#### BSD (macOS, ...) and Linux
- Clone the repository.

    ```bash
    git clone https://github.com/appcypher/wasmo.git
    ```

- Wasmo is a [Rust](https://www.rust-lang.org) project so it depends on `rustc` and `cargo`. You can install them with the following:

    ```bash
    curl https://sh.rustup.rs -sSf | sh
    ```

- You need to have `LLVM 6.0.x` installed[¹](#llvm).
    - Check the [known issues](#KNOWN_ISSUE) section for more information.

- Build the project.

    ```bash
    cd wasmo
    ```

    ```bash
    bash lite.sh install "path/to/llvm"
    ```

    This command does the following:
    - builds the `wasmo` project.
    - installs necessary commands like `wasmo`, and `lite`.

- After a successful install you should be able to use the `lite.sh` script via the `lite` command.

    ```bash
    lite --help
    ```

- You can also use `wasmo` command now.

    ```bash
    wasmo --help
    ```

#### Windows
- Clone the repository.

    ```bash
    git clone https://github.com/appcypher/wasmo.git
    ```

- Wasmo is a [Rust](https://www.rust-lang.org) project so it depends on `rustc` and `cargo`. You can install Rust here with the following:
    - Navigate to the `stable` section.
    - Download the `.msi` binary under `x86_64-pc-windows-gnu` if you are mostly working from mingw-based apps (Cygwin, MSYS2) or Window Subsytem for Linux.
    - Download the `.msi` binary under `x86_64-pc-windows-msvc` if you are mostly working from Visual Studio or used to Microsoft tools.
    - This is assuming that the average user will have a 64-bit Intel/AMD CPU, otherwise feel free to install the appropriate binary for your architecture.
    - Set path to installed binaries.

- You need to have `LLVM 6.0.x` installed[¹](#llvm).
    - Check the [known issues](#KNOWN_ISSUE) section for more information.

- Build the project.

    ```bash
    cd wasmo
    ```

    - `lite.bat` is still in works, but for now
    - open the command prompt or terminal and run the following command:

    ```bash
    LLVM_SYS_60_PREFIX=path/to/llvm cargo build --feature "verbose"
    ```

- You can run `wasmo` binary now.

    ```bash
    ./target/debug/wasmo --help
    ```


### KNOWN ISSUES<a name="known_issues"></a>
- You don't have LLVM installed.¹<a name="llvm"></a>
    - Well, you have to build LLVM from scratch. :/ I know.
    - This [guide](https://github.com/appcypher/llvm-adventure) can help with buidling llvm
    - This projects uses version `6.0.x` of LLVM so make sure you are downloading version `6.0.x`
    - The build will take some time. By some time, I mean a long time.
    - You can try building the project again.

### USAGE (Highly Experimental!)
- Run wasm file in examples directory
    ```bash
    wasmo examples/wasm/.wasm
    ```

### API (Susceptible to Change!)
```rust
// Create compiler flags.
let compiler_flags = Some(CompilerOptions {
    optimization_level: 3,
    exclude_passes: vec![
        LLVMPass::InstCombine,
    ],
    runtime_ignores: vec![
        RuntimeProperty::SignatureChecks,
        RuntimeProperty::MemoryBoundsChecks,
    ],
    strategy: CompilationStrategy::Normal,
});

// Create wasm options.
let options = Some(InstanceOptions {
    compiler_flags,
    abi: vec![ABI::Wasabi],
});

// JIT compile module in current process.
let module = compile(&wasm_binary, &options);

// Instantiate module.
let instance = module.instantiate(&imports);

// Get the exported main function from instance.
let main = instance.get_func("main");

// Store array of items in wasm memory 0
let wasm_array = instance.set_array(&arguments);

// Call the function.
main.call(5, wasm_array);
```

### CURRENT SUPPORT
- [ ] Codegen
    - [ ] basic llvm codegen
    - [ ] basic llvm traps and checks
    - [ ] basic AOT and Regular compilation
    - [ ] basic memory and table impl
    - [ ] basic table/call_indirect impl
    - [ ] llvm codegen completion
- [ ] Runtime
- [ ] Compilation Strategies
- [ ] Compilation Flags
- [ ] An ABI
- [ ] Other Features

### TODO
- wasm32 to LLVMIR to machine code
- fuzz tests
- unittests
- validation (utf8 [Unicode Standard 11.0, Section 3.9, Table 3-7. Well-Formed UTF-8 Byte Sequences] and semantics)

    - https://www.unicode.org/versions/Unicode11.0.0/ch03.pdf
    - https://webassembly.github.io/spec/core/binary/values.html#names

- error messages and making error position point of error instead of start_position

###

### ATTRIBUTIONS
These are the nice projects `wasmo` references for its design
- [inkwell](https://github.com/TheDan64/inkwell) [Apache-2.0] - the LLVM wrapper section is inspired by this awesome project that gives [llvm-sys](https://bitbucket.org/tari/llvm-sys.rs) a type-safe interface.
- [wasmer](https://github.com/wasmerio/wasmer) [MIT] - wasmo takes some architectural cues taken from wasmer, a WebAssembly runtime with cross-platform portability in mind.
- [wasmtime](https://github.com/CraneStation/wasmtime) [Apache-2.0] - wasmo also borrows some ideas from this wasmtime a Cranelift-based Webassembly runtime.

If you are interested in this project, you should probably check out [WAVM](https://github.com/WAVM/WAVM) as well.
In fact, you need to see [these other webassembly runtimes](https://github.com/appcypher/awesome-wasm-runtimes), they are all interesting projects.
