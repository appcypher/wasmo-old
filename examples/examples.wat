;; PREAMBLE 

;; preamble
(module)

;; TYPE SECTION

;; type section
(module
  (type (func (param i32) (result i32)))
  (type (func (param i32 i64)))
)

;; IMPORT SECTION

;; import section
(module
  (type (func (param i32 i64) (result i32)))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (import "env" "memory" (memory 0 1))
  (import "env" "global_1" (global i32))
  (import "env" "global_2" (global (mut i64)))
)

;; FUNCTION CODE SECTION

;; emepty function body
(module
  (type (func (param i32 i64) (result i32)))
  (type (func))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (func (;0;) (type 1)) ;; A function with empty code section.
)

;; locals
(module
  (type (func (param i32 i64) (result i32)))
  (type (func))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (func (;0;) (type 1)
    (local (;0;) i32)
    (local (;1;) i64 i64)
  )
)

;; nop
(module
  (type (func (param i32 i64) (result i32)))
  (type (func))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (func (;0;) (type 1)
    (local (;0;) i32)
    (nop)
  )
)

;; code section
