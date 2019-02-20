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


#### STRUCTURE
Context
    * new
    * 
    * type_*


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
