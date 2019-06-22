;; valid export sections
(module
  (import "env" "table" (table 0 1 anyfunc))
  (import "env" "mem" (memory 1 2))
  (import "env" "glob" (global (mut f64)))
  (func (;0;) (export "main"))
  (func (;1;) (param f32 f64) (result f32) (f32.const 0xff.ff))
  (export "func_1" (func 1))
  (export "table" (table 0))
  (export "mem" (memory 0))
  (export "glob" (global 0))
)
