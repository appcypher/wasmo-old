#### TODO
- [ ] Generate module information
- [ ] Generate simple functions and operations for test
- [ ] Simple AOT compilation example
- [ ] Simple JIT compilation example with ORC
- [ ] Simple LazyJIT compilation example with ORC
- [ ] Proper planning of module/ir memory management
- [ ] Trap and checks integration (checks types)
- [ ] Proper planning of wasm memory model (page reservation and guard page)


#### PROCESS ADDRESS SPACE (LINUX x86-64 EXAMPLE)

```
+++++++++++++++++++++++++++
|         STACK
+++++++++++++++++++++++++++
|    SHARED LIBRARIES
+++++++++++++++++++++++++++
|          HEAP
|
|
|  ++++++ INSTANCE ++++++
|  ++++++++++++++++++++++
|  |                    |
|  |    LINEAR MEMORY   | RW
|  |                    |
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |      GLOBALS       | R
|  ++++++++++++++++++++++
|            :
|  ++++++++++++++++++++++
|  |       TABLES       | R
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
