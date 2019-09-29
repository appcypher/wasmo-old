use crate::types::{ValType, TablePtr, MemoryPtr, GlobalPtr, FuncPtr, Imports, Exports};


pub struct ResizableLimits {
    minimum: u16,
    maximum: u16,
}

pub struct Signature {
    params: Vec<ValType>,
    returns: Vec<ValType>,
}

pub struct TableDesc {
    ptr: TablePtr, // Nullable
    limits: ResizableLimits,
}

pub struct MemoryDesc {
    ptr: MemoryPtr, // Nullable
    limits: ResizableLimits,
}

pub struct GlobalDesc {
    ptr: GlobalPtr, // Nullable
    mutable: bool,
    ty: ValType,
}

pub struct FuncDesc {
    ptr: FuncPtr, // Nullable
    sig: Signature,
}

pub struct Locals {
    tables: Vec<TableDesc>,
    memories: Vec<MemoryDesc>,
    globals: Vec<GlobalDesc>,
    functions: Vec<FuncDesc>,
}

pub struct ModuleDesc {
    exports: Exports,
    imports: Imports,
    locals: Locals,
}

pub enum Desc {
    TableDesc(TableDesc),
    MemoryDesc(MemoryDesc),
    GlobalDesc(GlobalDesc),
    FuncDesc(FuncDesc),
}
