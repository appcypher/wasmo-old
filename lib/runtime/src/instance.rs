use crate::Module;

pub(crate) struct Ofssets {}

///
#[repr(C)]
pub struct Instance {
    offsets: Offsets,
    module: &Module,
}

impl Instance {}
