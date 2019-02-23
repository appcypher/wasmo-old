use crate::{
    TablePtr,
    TableDesc,
    MemoryPtr,
    MemoryDesc,
    GlobalDesc,
    GlobalPtr,
    FuncDesc,
    FuncPtr,
};

///
pub struct Imports {
    tables: HostTables,
    memories: HostMemories,
    globals: HostGlobals,
    functions: HostFunctions
};

///
type HostTables = Vec<(String, String, TablePtr, TableDesc)>;

///
type HostMemories = Vec<(String, String, MemoryPtr, MemoryDesc)>;

///
type HostGlobals = Vec<(String, String, GlobalPtr, GlobalDesc)>;

///
type HostFunctions = Vec<(String, String, FuncPtr, FuncDesc)>;
