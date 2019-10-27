
use crate::types::{FuncPtr, GlobalPtr, MemoryPtr, TablePtr};

/// InstanceContext holds pointers for accessing all the memories, tables, functions,
/// and globals specified for an instance. It contains both local and imported elements
/// and it also contain pointers to intrinsice functions like `grow_memory`.
///
/// InstanceContext is purposefully designed to not be type safe because we
/// need to optimize access to the values it holds.
///
/// Conceptually, InstanceContext has the following type. Although, we cannot express dynamic
/// arrays in Rust.
///
/// ```rust
/// struct InstanceContext {
///     memories_offset: usize,
///     tables_offset: usize,
///     globals_offset: usize,
///     functions_offset: usize,
///     intrinsic_function_offset: usize,
///     memories: dyn [*mut u8; memory_count],
///     tables: dyn [*mut u32; table_count],
///     globals: dyn [*mut u64; global_count],
///     functions: dyn [*const (); function_count],
///     intrinsic_functions: dyn [*const (); intrinsic_function_count],
/// }
/// ```
///
/// With the structure above, loading a value from InstanceContext is matter of a single lea instruction.
/// It effectively improves cache hit and reduces pointer indirections. Offsets are statically known,
/// so the lea operands can are mostly immediate values.
///
/// **Code**
/// ```c
/// (context + intrinsic_function_offset + some_func_index)()
/// ```
///
/// **InstanceContext with pointers**
/// ```asm
/// mov rcx, dword ptr [rdi + 5*8]
/// lea rax, dword ptr [rdi + rcx + 0*8]
/// call [rax]
/// ```
///
/// **InstanceContext with buffer**
/// ```asm
/// lea rax, dword ptr [rdi + 5*8 + 1*8 + 1*8 + 20*8 + 0*8]
/// call [rax]
/// ```
///
/// #### POTENTIAL OPTIMIZATIONS
/// InstanceContext is used in virtually every function, might as well just store
/// it in a register that doesn't get cloberred between calls.
#[repr(C)]
#[derive(Debug)]
pub struct InstanceContext {
    buffer: *mut usize,
}


/// Because of its nature of type unsafety, InstanceContext only exposes type safe
/// interface to the public. You cannot meddle with the buffer pointer directly.
impl InstanceContext {
    ///
    pub fn new() -> Self {
        unimplemented!()
    }
}
