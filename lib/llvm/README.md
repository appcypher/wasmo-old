<h2 align="center">LLVM</h2>

--------------

### DESCRIPTION

Implementation of a type safe wrapper around llvm-sys.

--------------

### MAP

--------------

### OWNERSHIP DETAILS
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

--------------

### INKWELL

[Inkwell](https://github.com/TheDan64/inkwell) is a great project and this project is largely based on it. However, since `wasmo` is an LLVM-based project, it is important to have a greater degree of control of the API that wraps it. Inkwell doesn't yet have an ORCJIT API which is crucial to `wasmo`.
