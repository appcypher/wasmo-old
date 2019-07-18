Experimenting! Random notes on speculative ideas.

### COMMANDLINE

#### COMPILATION TYPE
- Generating an executable from a wasm file.

    ```bash
    wasmo server.wasm -o server.out
    ```

    This command should generate the following files:
    * server.out
    * libwasabi.dylib (if wasabi is specified)

    ```bash
    wasmo server.wasm -c libserver.a
    ```

    This command should generate a static library that can be linked against.
    * libserver.a

    In the future, there could be more sophisticated options for linking multiple wasm files into a single executable.
    https://github.com/WebAssembly/tool-conventions/blob/master/Linking.md

    ```bash
    wasmo server.wasm --compilation-mode=lazy
    wasmo server.wasm -cm=lazy
    ```

#### SELECTING HOST ENVIRONMENT
- Running a wasm file with the wasabi (wasm32-linux-llvm) abi.

    ```bash
    wasmo server.wasm --host=wasabi
    wasmo server.wasm -h=wasabi
    ```

#### PASSING ARGUMENTS TO WASM APPLICATION
- Passing arguments to a wasm application

    ```bash
    wasmo server.wasm -- 8080
    ```

#### PERMISSIONS
- Checking a wasm application permissions

    ```bash
    wasmo server.wasm --perms -h=wasabi
    wasmo server.wasm -p -h=wasabi
    ```

- Setting the permissions a wasm application can have

    ```bash
    wasmo server.wasm --allow-file="~/Desktop/sample.txt" -h=wasi
    ```

### INSTALLATION

- For Unix-like OSes

    ```bash
    curl https://lite.wasmlab.io -sSf | sh
    ```

- Windows
    - Download appropriate binary from [wasmlab.io/downloads/wasmo](https://www.wasmlab.io/downloads/wasmo)

### CUSTOM SECTIONS
- HOST INTERFACE SECTION
    - Target triple definition (Cranelift style)
    - Payload structure
        ```
        section_id               - varuint7  = 0

        payload_length           - varuint32

        name_length              - varuint32 = 14
        name_string              - uint8*    = "host-interface"

        target_triple_length     - varuint32
        target_architecture      - uint8
        target_vendor            - varuint32
        target_operating_system  - varuint32

        import_count             - varuint32
        import_index             - varuint32* (field index in import section)
        ```
    - The host-binding section can be placed before or after any other section
    - There can only be one host-binding section in wasm file

- JIT SECTION
    > Contains details of functions to be replaced or created as well as the strategy to use if available.
    - Payload structure

#### COMPILED PYTHON

- Type inference
- JIT


#### DETECT HOST BINDING

- Wasmo shouldn't assume host binding. It has to be specified

```sh
wasmo examples/print.wasm -h=emscripten
```
