(module
  (memory 1)
  (table 10 funcref)

  (elem (i32.const 0) $f)

  (data (i32.const 0) "a")
  (data (i32.const 3) "b")
  (data (i32.const 100) "cde")
  (data (i32.const 5) "x")
  (data (i32.const 3) "c")

  (func $f)
  (func $main)

  (start $main)
)
