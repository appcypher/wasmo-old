Experimenting! Random notes on speculative ideas.

### COMMANDLINE

#### COMPILATION TYPE
- Generating an executable from a wasm file.

    ```bash
    wasmlite server.wasm -o server.out
    ```

    This command should generate the following files:
    * server.out
    * libwasabi.dylib (if wasabi is specified)

    ```bash
    wasmlite server.wasm -c libserver.a
    ```

    This command should generate a static library that can be linked against.
    * libserver.a

    In the future, there could be more sophisticated options for linking multiple wasm files into a single executable.
    https://github.com/WebAssembly/tool-conventions/blob/master/Linking.md

    ```bash
    wasmlite server.wasm --mode=lazy
    ```

#### SELECTING ABI
- Running a wasm file with the wasabi (llvm/musl) abi.

    ```bash
    wasmlite server.wasm --abi=wasabi
    ```

    The emscripten abi is the default (?).

#### PASSING ARGUMENTS TO WASM APPLICATION
- Passing arguments to a wasm application

    ```bash
    wasmlite server.wasm -- 8080
    ```

#### PERMISSIONS
- Setting the permissions a wasm application can have

    ```bash
    wasmlite server.wasm --perms=no-port,no-file
    ```

- Checking a wasm application permissions

    ```bash
    wasmlite server.wasm --perms
    ```

### INSTALLATION

- For Unix-like OSes

    ```bash
    curl https://lite.wasmlab.io -sSf | sh
    ```

- Windows
    - Download appropriate binary from [wasmlab.io/downloads/wasmlite](https://www.wasmlab.io/downloads/wasmlite)
