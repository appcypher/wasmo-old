use crate::ir::Type;
use crate::kinds::SectionKind;

pub fn int_to_type(value: i8) -> Type {
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

pub fn int_to_section(value: u8) -> SectionKind {
    match value {
        0x1 => SectionKind::Type,
        0x2 => SectionKind::Import,
        0x3 => SectionKind::Function,
        0x4 => SectionKind::Table,
        0x5 => SectionKind::Memory,
        0x6 => SectionKind::Global,
        0x7 => SectionKind::Export,
        0x8 => SectionKind::Start,
        0x9 => SectionKind::Element,
        0xA => SectionKind::Code,
        0xB => SectionKind::Data,
        _ => unreachable!(),
    }
}
