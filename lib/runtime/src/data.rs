//! This module contains compile-time and runtime information of a WebAssembly module.
use crate::types::{
    ExportKind, Exports, FuncPtr, FuncType, GlobalPtr, Imports, MemoryPtr, TablePtr, ValueType,
};

use std::ptr::null;

#[derive(Debug)]
pub struct ResizableLimits {
    minimum: u16,
    maximum: u16,
}

///
#[derive(Debug)]
pub struct MemoryData {
    ptr: MemoryPtr, // Nullable
    runtime_length: usize,
    limits: ResizableLimits,
}

#[derive(Debug)]
pub struct TableData {
    ptr: TablePtr, // Nullable
    runtime_length: usize,
    limits: ResizableLimits,
}

#[derive(Debug)]
pub struct GlobalData {
    ptr: GlobalPtr, // Nullable
    mutable: bool,
    ty: ValueType,
}

#[derive(Debug)]
pub struct FuncData {
    pub ptr: FuncPtr, // Nullable
    pub type_index: u32,
}

#[derive(Debug)]
pub struct Locals {
    pub types: Vec<FuncType>,
    pub memories: Vec<MemoryData>,
    pub tables: Vec<TableData>,
    pub globals: Vec<GlobalData>,
    pub functions: Vec<FuncData>,
}

#[derive(Debug)]
pub enum Data {
    Memory(MemoryData),
    Table(TableData),
    Global(GlobalData),
    Func(FuncData),
}

#[derive(Debug)]
pub struct ModuleData {
    pub exports: Exports,
    pub imports: Imports,
    pub locals: Locals,
}

impl Locals {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            memories: Vec::new(),
            tables: Vec::new(),
            globals: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl ModuleData {
    pub fn new() -> Self {
        use hashbrown::HashMap;

        Self {
            exports: HashMap::new(),
            imports: HashMap::new(),
            locals: Locals::new(),
        }
    }

    pub fn add_type(&mut self, func_type: FuncType) {
        self.locals.types.push(func_type);
    }

    pub fn add_function(&mut self, func: FuncData) {
        self.locals.functions.push(func);
    }

    pub fn add_export(&mut self, field: String, export_kind: ExportKind) {
        self.exports.insert(field, export_kind);
    }
}

impl FuncData {
    pub fn new(ptr: FuncPtr, type_index: u32) -> Self {
        Self { ptr, type_index }
    }
}
