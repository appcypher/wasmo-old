Mostly reimplementing inkwell just to gain understanding of LLVM.

### OWNERSHIP
- Context
    - can be shared by
        - Module
        - Builder

- Module
    - can be consumed by
        - ExecutionEngine

- Builder
    - not owned by anyone

- Execution Engine
    - not owned by anyone

- Types
    - are always consumed when used in other data structures. Notably
        -

### TODO
- Add ownership semantics as doc on each type
