
;; valid_module_with_preamble
(module)

;; valid_module_with_tysection
(module
  (type (func (param i32) (result i32)))
  (type (func (param i32 i64)))
)

;; valid_module_with_import_section
(module
  (type (func (param i32 i64) (result i32)))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (import "env" "memory" (memory 0 1))
  (import "env" "global_1" (global i32))
  (import "env" "global_2" (global (mut i64)))
)

;; valid_module_with_empty_function_body
(module
  (type (func (param i32 i64) (result i32)))
  (type (func))
  (import "env" "func" (func (type 0)))
  (import "env" "table" (table 0 1 anyfunc))
  (func (;0;) (type 1)) ;; A function with empty code section.
)

;; valid_module_with_table_section_and_maximum
(module
  (type (func (param i32 i64) (result i32)))
  (import "env" "func" (func (type 0)))
  (table 1 2 anyfunc) ;; table with maximum
)

;; valid_module_with_table_section_no_maximum
(module
  (type (func (param i32 i64) (result i32)))
  (import "env" "func" (func (type 0)))
  (table 1 anyfunc) ;; table without maximum
)

;; valid_module_with_memory_section_and_maximum
(module
  (type (func (param i32 i64) (result i32)))
  (table 1 anyfunc)
  (memory 4 70) ;; memory with maximum
)

;; valid_module_with_memory_section_no_maximum
(module
  (type (func (param i32 i64) (result i32)))
  (table 1 anyfunc)
  (memory 65535) ;; memory with no maximum
)

;; valid_module_with_memory_section_and_maximum
(module
  (type (func (param i32 i64) (result i32)))
  (table 1 anyfunc)
  (memory 4 70) ;; memory with maximum
)

;; valid_module_with_locals_in_function_body
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

;; valid_module_with_nop_in_function_body
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
