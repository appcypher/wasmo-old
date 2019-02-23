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
pub struct Exports {
    tables: GuestTables,
    memories: GuestMemories,
    globals: GuestGlobals,
    functions: GuestFunctions
};

///
type GuestTables = Vec<(String, TablePtr, TableDesc)>;

///
type GuestMemories = Vec<(String, MemoryPtr, MemoryDesc)>;

///
type GuestGlobals = Vec<(String, GlobalPtr, GlobalDesc)>;

///
type GuestFunctions = Vec<(String, FuncPtr, FuncDesc)>;

