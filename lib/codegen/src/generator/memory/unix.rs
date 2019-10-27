use wasmo_llvm::{Builder, Context};
use wasmparser::ResizableLimits;

pub struct MemoryGenerator();

impl MemoryGenerator {
    /// Generates IR for calling mmap and co needed for creating
    /// memory.
    pub fn generate_memory_setup_code(
        limits: &ResizableLimits,
        context: &Context,
        builder: &Builder,
    ) -> () {
        // TODO
    }

    // DATA
    pub fn generate_memory_initialization_code(values: &[u8]) -> () {}

    // MEMORY.GROW
    pub fn generate_memory_size_code() -> () {}

    // MEMORY.SIZE
    pub fn generate_memory_grow_code() -> () {}

    /// This is only generated for AOT mode, for JIT mode an in-process function
    /// is shared and used accross instances.
    pub fn generate_memory_grow_function() -> () {}

    /// This is only generated for AOT mode, for JIT mode an in-process function
    /// is shared and used accross instances.
    pub fn generate_memory_size_function() -> () {}
}

pub struct TableGenerator();

impl TableGenerator {
    // TABLE ENTRY
    pub fn generate_table_setup_code() -> () {}

    // ELEM
    pub fn generate_table_initialization_code() -> () {}
}

pub struct GlobalGenerator();

impl GlobalGenerator {}
