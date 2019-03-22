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
[Inkwell](https://github.com/TheDan64/inkwell) is a great project and the wrapper section of this project is largely based on it, but I decided wasmlite needs its own wrapper because wasmlite is in part meant to be a way of learning about LLVM and wasmlite's needs may differ from Inkwell's perspective. For example, Inkwell doesn't have a wrapper for the ORC C API yet. This is perhaps due to the API being currently outdated, but I'd like to start working on it as soon as possible.

Wasmlite may still use Inkwell in the future if deemed appropriate.

BTW, if you are just starting out with LLVM and you know some Rust, I encourage you to read Inkwell's source, there are interesting comments and notes left in it.

-------------------------------------------------------------------


## PROCESS ADDRESS SPACE (LINUX x86-64 EXAMPLE)

```
+++++++++++++++++++++++++++
|         STACK
+++++++++++++++++++++++++++
|    SHARED LIBRARIES
+++++++++++++++++++++++++++
|          HEAP
|
|
|  ++++ WASM INSTANCE +++
|  ++++++++++++++++++++++
|  |                    |
|  |    LINEAR MEMORY   | RW
|  |                    |
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |      GLOBALS       | RW
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |       TABLE        | R
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |        CODE        | E
|  ++++++++++++++++++++++
|
+++++++++++++++++++++++++++
|      BSS SEGMENT
+++++++++++++++++++++++++++
|    (RO)DATA SEGMENT
+++++++++++++++++++++++++++
|      TEXT SEGMENT
+++++++++++++++++++++++++++
```

-------------------------------------------------------------------

## CALLING CONVENTION


-------------------------------------------------------------------

## TRAPS AND EXCEPTIONS
Things that can generate traps
- Code-generated traps for illegal arithmetic operations, OOB accesses, etc.
- OS-generated traps for illegal memory access
- Host-generated traps

A list of things that can trap at runtime:
- ...


-------------------------------------------------------------------

## THREADS AND ATOMICS



-------------------------------------------------------------------

## SIMD


-------------------------------------------------------------------


## DEBUGINFO


