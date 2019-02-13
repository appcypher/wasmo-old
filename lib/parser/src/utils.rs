#[derive(Debug)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    Anyfunc,
    Func,
    Empty,
}

pub fn to_type(value: i8) -> Type {
    match value {
        -0x01 => Type::I32,
        -0x02 => Type::I64,
        -0x03 => Type::F32,
        -0x04 => Type::F64,
        -0x10 => Type::Anyfunc,
        -0x20 => Type::Func,
        -0x40 => Type::Empty,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum Section {
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

pub fn to_section(value: u8) -> Section {
    match value {
        0x1 => Section::Type,
        0x2 => Section::Import,
        0x3 => Section::Function,
        0x4 => Section::Table,
        0x5 => Section::Memory,
        0x6 => Section::Global,
        0x7 => Section::Export,
        0x8 => Section::Start,
        0x9 => Section::Element,
        0xA => Section::Code,
        0xB => Section::Data,
        _ => unreachable!(),
    }
}

