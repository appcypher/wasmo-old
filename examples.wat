;; type section
(module
  (type (func (param i32) (result i32)))
  (type (func (param i32 i64)))
)

;; import section
(module
  (type (func (param i32 i64) (result i32)))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (import "env" "memory" (memory 0 1))
  (import "env" "global_1" (global i32))
  (import "env" "global_2" (global (mut i64)))
)

;; function section

;; code section
