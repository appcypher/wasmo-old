(module
    (func $main (export "main") (param i32 i64 i64) (result i32)
        (i32.wrap/i64 (get_local 1))
        (get_local 0)
        (i32.add)
    )
)
