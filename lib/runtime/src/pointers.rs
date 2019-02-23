use std::marker::PhantomData;

/// Ownership - the data this structure holdsis meant to be owned by the enclosing structure.
struct BoundPtr<T> {
    data: *mut T,
    size: usize,
}

impl BoundPtr<T> {
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
pub type FuncPtr = *const usize;
