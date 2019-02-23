use libc::c_void;

use crate::{
    Module,
    Imports,
    TablePtr,
    MemoryPtr,
    GlobalPtr,
};

struct Data {
    /// - Security - tables size validated at parse-time
    tables: *const TablePtr,
    /// - Security - memories size validated at parse-time
    memories: *mut MemoryPtr,
    ///
    globals: *mut GlobalPtr,
}

///
#[repr(C)]
pub struct Instance {
    data: Data,
    module: Module,
}

impl Instance {
    /// ##### Compilation modes
    /// - Normal - combines local and imported tables, memories and globals and creates them once
    ///      - resolves links to functions, globals, memories and tables
    /// - AOT - AOT doesn't reach here
    /// - Lazy - ...
    /// - REPL - ...
    ///
    /// ##### Security
    /// - partial RELRO must be enabled for lazy compilations
    pub fn instantiate(module: &Module, import: &Imports) -> Instance { unimplemented!() }

    ///
    pub fn get_func(&self, func_name: &str) -> Func { unimplemented!() }

    ///
    fn get_func_addr(&self, index: u32) -> *const c_void { unimplemented!() }

    ///
    fn get_func_index(&self, func_name: &str) -> u32 { unimplemented!() }
}

impl Drop for Instance {
    /// Unmap module-local tables, memories and globals from process address space
    fn drop() { unimplemented!() }
}
