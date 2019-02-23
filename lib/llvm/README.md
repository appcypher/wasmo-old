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


