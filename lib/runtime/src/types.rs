use hashbrown::HashMap;
use crate::descriptions::{Desc, TableDesc, MemoryDesc, GlobalDesc, FuncDesc};

///
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

/// 
pub enum Type {
    ValType(ValType),
    FuncRef,
}

/// Ownership - the data this structure holds is meant to be owned by the enclosing structure.
pub struct BoundPtr<T> {
    data: *mut T,
    size: usize,
}

impl<T> BoundPtr<T> {
    pub(crate) fn as_ptr(&mut self) -> *mut T {
        self.data
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
pub type Imports = HashMap<String, HashMap<String, Desc>>;
pub type Exports = HashMap<String, Desc>;


