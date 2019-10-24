use crate::data::{FuncData, GlobalData, MemoryData, TableData, Data};
use hashbrown::HashMap;

#[derive(Debug)]
pub struct FuncType {
    params: Vec<ValueType>,
    returns: Vec<ValueType>,
}

///
#[derive(Debug)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}


#[derive(Debug)]
pub enum ExportKind {
    Memory(u32),
    Table(u32),
    Global(u32),
    Function(u32),
}

///
#[derive(Debug)]
pub enum Type {
    ValueType(ValueType),
    FuncRef,
}

/// Ownership - the data this structure holds is meant to be owned by the enclosing structure.
#[repr(C)]
#[derive(Debug)]
pub struct BoundPtr<T> {
    base_ptr: *mut T,
    size: usize,
}

impl<T> BoundPtr<T> {
    pub(crate) fn as_mut_ptr(&mut self) -> *mut T {
        self.base_ptr
    }
}

/// ###### Security
/// - protected by bounds checks
pub type TablePtr = BoundPtr<u32>;

/// ###### Security
/// - protected by guard page
pub type MemoryPtr = *mut u8;

///
pub type GlobalPtr = *mut u64;

///
pub type FuncPtr = *const ();

/// Imports and Exports
pub type Imports = HashMap<String, HashMap<String, Data>>;
pub type Exports = HashMap<String, ExportKind>;

impl FuncType {
    pub fn new(params: Vec<ValueType>, returns: Vec<ValueType>) -> Self {
        Self { params, returns }
    }
}
