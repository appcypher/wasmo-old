pub struct TableDesc {
    minimum: u16,
    maximum: u16,
}

pub struct MemoryDesc {
    minimum: u16,
    maximum: u16,
}

pub struct GlobalDesc {
    type: Type,
}

pub struct FuncDesc {
    params: Vec<Type>,
    returns: Vec<Type>,
}
