#### TODO
- [ ] Generate module information
- [ ] Generate simple functions and operations for test
- [ ] Simple AOT compilation example
- [ ] Simple Regular compilation example
- [ ] Simple Lazy compilation example with ORC
- [ ] Simple REPL compilation example with ORC
- [ ] Proper planning of module/ir memory management
- [ ] Trap and checks integration (checks types)
- [ ] Proper planning of wasm memory model (page reservation and guard page)
- [ ] Turn string conversion panics and asserts to CompilerError

#### OWNERSHIP
- Context
    - can be shared by
        - Module
        - Builder
        - BasicBlock

- Module
    - can be consumed by
        - ExecutionEngine (?)

- Builder
    - not owned by anyone

- BasicBlock
    - can be shared by
        - Builder

- Execution Engine
    - not owned by anyone

- Types
    - are consumed when used in other data structures.

- Values
    - are consumed when used in other data structures.

#### Reason I'm not using Inkwell
[Inkwell](https://github.com/TheDan64/inkwell) is a great project and this project is largely based on it, but I decided wasmo needs its own wrapper because wasmo is based on LLVM and it will be nice to have very little abstraction blackbox as possible.
