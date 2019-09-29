(module
    (type (func (param i32 i32) (result i32)))

    (import "__wasi_fd_write" (func (type 0))) ;; wasi function for printing to stdout

    (memory (export "memory") 10)

    (data (i32.const 0) "Hello world!")

    (func (export "_start")
        (i32.load 0) ;; arg - fd - stdout
        (i32.load 0) ;; arg - buffer
        (call 0) ;; call __wasi_fd_write function
        (drop) ;; drop value on stack
    )
)
