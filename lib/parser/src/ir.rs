use hashbrown::HashMap;

///
pub fn get_signature_by_body_index(
    body_index: usize,
    sections: &HashMap<u8, Section>,
) -> Option<FuncSignature> {
    // Get the function
    let signature_index = get_function_by_index(body_index, sections)?;

    // Get the signature
    get_signature_by_index(signature_index as usize, sections)
}

///
pub fn get_signature_by_index(
    index: usize,
    sections: &HashMap<u8, Section>,
) -> Option<FuncSignature> {
    match sections.get(&0x01).unwrap() {
        Section::Type(types) => {
            // Get the signature
            match &types[index] {
                Type::Func(sig) => Some(sig.clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

///
pub fn get_function_by_index(index: usize, sections: &HashMap<u8, Section>) -> Option<u32> {
    match sections.get(&0x03).unwrap() {
        Section::Function(functions) => Some(functions[index]),
        _ => None,
    }
}

///
pub fn get_global_by_index(index: u32, sections: &HashMap<u8, Section>) -> Option<Global> {
    match sections.get(&0x06).unwrap() {
        Section::Global(globals) => Some(globals[index as usize].clone()),
        _ => None,
    }
}

///
#[derive(Debug, Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    Func(FuncSignature),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncSignature {
    pub params: Vec<ValueType>,
    pub returns: Vec<ValueType>,
}

impl From<i8> for Type {
    fn from(value: i8) -> Self {
        match value {
            -0x01 => Type::I32,
            -0x02 => Type::I64,
            -0x03 => Type::F32,
            -0x04 => Type::F64,
            -0x10 => Type::FuncRef,
            -0x20 => Type::Func(FuncSignature {
                params: vec![],
                returns: vec![],
            }),
            -0x40 => Type::Empty,
            _ => unreachable!(),
        }
    }
}

///
#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

impl From<i8> for ValueType {
    fn from(value: i8) -> Self {
        match value {
            -0x01 => ValueType::I32,
            -0x02 => ValueType::I64,
            -0x03 => ValueType::F32,
            -0x04 => ValueType::F64,
            _ => unreachable!(),
        }
    }
}

///
#[derive(Debug, Clone)]
pub struct Module {
    // TODO: Change this to Vecs of Vecs vec![ type: vec![], import: vec![], function: vec![], table: vec![], ..., customs: ... ]
    pub sections: HashMap<u8, Section>,
}

///
#[derive(Debug, Clone)]
pub enum Section {
    Type(Vec<Type>),
    Import(Vec<Import>),
    Function(Vec<u32>),
    Table(Vec<Table>),
    Memory(Vec<Memory>),
    Global(Vec<Global>),
    Export(Vec<Export>),
    Start(u32),
    Element(Vec<Element>),
    Code(Vec<Function>),
    Data(Vec<Data>),
    Custom,
}

///
#[derive(Debug, Clone)]
pub struct Import {
    pub module_name: String,
    pub field_name: String,
    pub desc: ImportDesc,
}

///
#[derive(Debug, Clone)]
pub enum ImportDesc {
    Function {
        type_index: u32,
    },
    Table(Table),
    Memory(Memory),
    Global {
        content_type: ValueType,
        mutability: bool,
    },
}

///
#[derive(Debug, Clone)]
pub struct Table {
    pub element_type: Type,
    pub minimum: u32,
    pub maximum: Option<u32>,
}

///
#[derive(Debug, Clone)]
pub struct Memory {
    pub minimum: u32,
    pub maximum: Option<u32>,
}

///
#[derive(Debug, Clone)]
pub struct Global {
    pub content_type: ValueType,
    pub mutability: bool,
    pub instructions: Vec<Operator>,
}

///
#[derive(Debug, Clone)]
pub struct Function {
    pub locals: Vec<Local>,
    pub instructions: Vec<Operator>,
}

///
#[derive(Debug, Clone)]
pub struct Element {
    pub table_index: u32,
    pub instructions: Vec<Operator>,
    pub func_indices: Vec<u32>,
}

///
#[derive(Debug, Clone)]
pub struct Data {
    pub memory_index: u32,
    pub instructions: Vec<Operator>,
    pub bytes: Vec<u8>,
}

///
#[derive(Debug, Clone)]
pub struct Local {
    pub count: u32,
    pub local_type: ValueType,
}

///
#[derive(Debug, Clone)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

///
#[derive(Debug, Clone)]
pub enum ExportDesc {
    Function(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}

///
#[derive(Debug, Clone)]
pub enum Operator {
    End,
    Nop,
    Unreachable,

    // VARIABLE ACCESS
    // (local_index: Imm) -> T
    LocalGet(u32),
    // (local_index: Imm, value: T)
    LocalSet(u32, usize),
    // (local_index: Imm, value: T) -> T
    LocalTee(u32, usize),

    // (global_index: Imm, value: T)
    GlobalGet(u32),
    // (global_index: Imm, value: T)
    GlobalSet(u32, usize),

    // MEMORY
    // (align: Imm, offset: Imm, base: I32) -> T
    I32Load(u32, u32, usize),
    I64Load(u32, u32, usize),
    F32Load(u32, u32, usize),
    F64Load(u32, u32, usize),

    I32Load8Signed(u32, u32, usize),
    I64Load8Signed(u32, u32, usize),

    I32Load8Unsigned(u32, u32, usize),
    I64Load8Unsigned(u32, u32, usize),

    I32Load16Signed(u32, u32, usize),
    I64Load16Signed(u32, u32, usize),

    I32Load16Unsigned(u32, u32, usize),
    I64Load16Unsigned(u32, u32, usize),

    I64Load32Signed(u32, u32, usize),
    I64Load32Unsigned(u32, u32, usize),

    // (align: Imm, offset: Imm, base: I32, value: T)
    I32Store(u32, u32, usize, usize),
    I64Store(u32, u32, usize, usize),
    F32Store(u32, u32, usize, usize),
    F64Store(u32, u32, usize, usize),

    I32Store8(u32, u32, usize, usize),
    I64Store8(u32, u32, usize, usize),

    I32Store16(u32, u32, usize, usize),
    I64Store16(u32, u32, usize, usize),

    I64Store32(u32, u32, usize, usize),

    // (delta: I32) -> I32
    MemoryGrow(usize),

    // () -> I32
    MemorySize,

    // CONSTANT
    // (value: Imm)
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // COMPARISONS
    // (value: T) -> I32
    I32Eqz(usize),
    I64Eqz(usize),

    // (lhs: T, rhs: T) -> I32
    I32Eq(usize, usize),
    I64Eq(usize, usize),
    F32Eq(usize, usize),
    F64Eq(usize, usize),

    I32Ne(usize, usize),
    I64Ne(usize, usize),
    F32Ne(usize, usize),
    F64Ne(usize, usize),

    I32LtSigned(usize, usize),
    I64LtSigned(usize, usize),

    I32LtUnsigned(usize, usize),
    I64LtUnsigned(usize, usize),

    I32GtSigned(usize, usize),
    I64GtSigned(usize, usize),

    I32GtUnsigned(usize, usize),
    I64GtUnsigned(usize, usize),

    I32LeSigned(usize, usize),
    I64LeSigned(usize, usize),

    I32LeUnsigned(usize, usize),
    I64LeUnsigned(usize, usize),

    I32GeSigned(usize, usize),
    I64GeSigned(usize, usize),

    I32GeUnsigned(usize, usize),
    I64GeUnsigned(usize, usize),

    F32Lt(usize, usize),
    F64Lt(usize, usize),

    F32Gt(usize, usize),
    F64Gt(usize, usize),

    F32Le(usize, usize),
    F64Le(usize, usize),

    F32Ge(usize, usize),
    F64Ge(usize, usize),

    // NUMERIC
    // (value: T) -> T
    I32Clz(usize),
    I64Clz(usize),

    I32Ctz(usize),
    I64Ctz(usize),

    I32Popcnt(usize),
    I64Popcnt(usize),

    // (lhs: T, rhs: T) -> T
    I32And(usize, usize),
    I64And(usize, usize),

    I32Or(usize, usize),
    I64Or(usize, usize),

    I32Xor(usize, usize),
    I64Xor(usize, usize),

    I32Shl(usize, usize),
    I64Shl(usize, usize),

    I32ShrSigned(usize, usize),
    I64ShrSigned(usize, usize),

    I32ShrUnsigned(usize, usize),
    I64ShrUnsigned(usize, usize),

    I32Rotl(usize, usize),
    I64Rotl(usize, usize),

    I32Rotr(usize, usize),
    I64Rotr(usize, usize),

    I32Add(usize, usize),
    I64Add(usize, usize),
    F32Add(usize, usize),
    F64Add(usize, usize),

    I32Sub(usize, usize),
    I64Sub(usize, usize),
    F32Sub(usize, usize),
    F64Sub(usize, usize),

    I32Mul(usize, usize),
    I64Mul(usize, usize),
    F32Mul(usize, usize),
    F64Mul(usize, usize),

    I32DivSigned(usize, usize),
    I32DivUnsigned(usize, usize),
    I64DivSigned(usize, usize),
    I64DivUnsigned(usize, usize),
    F32Div(usize, usize),
    F64Div(usize, usize),

    I32RemSigned(usize, usize),
    I32RemUnsigned(usize, usize),
    I64RemSigned(usize, usize),
    I64RemUnsigned(usize, usize),

    I32Min(usize, usize),
    I64Min(usize, usize),
    F32Min(usize, usize),
    F64Min(usize, usize),

    I32Max(usize, usize),
    I64Max(usize, usize),
    F32Max(usize, usize),
    F64Max(usize, usize),

    F32CopySign(usize, usize),
    F64CopySign(usize, usize),

    F32Abs(usize),
    F64Abs(usize),

    F32Neg(usize),
    F64Neg(usize),

    F32Ceil(usize),
    F64Ceil(usize),

    F32Floor(usize),
    F64Floor(usize),

    F32Trunc(usize),
    F64Trunc(usize),

    F32Nearest(usize),
    F64Nearest(usize),

    F32Sqrt(usize),
    F64Sqrt(usize),

    // CONVERSIONS
    // (value: T) -> U
    I32WrapI64(usize),

    I32TruncF32Signed(usize),
    I32TruncF32Unsigned(usize),

    I32TruncF64Signed(usize),
    I32TruncF64Unsigned(usize),

    I64ExtendI32Signed(usize),
    I64ExtendI32Unsigned(usize),

    I64TruncF32Signed(usize),
    I64TruncF32Unsigned(usize),

    I64TruncF64Signed(usize),
    I64TruncF64Unsigned(usize),

    F32ConvertI32Signed(usize),
    F32ConvertI32Unsigned(usize),

    F32ConvertI64Signed(usize),
    F32ConvertI64Unsigned(usize),

    F32DemoteF64(usize),

    F64ConvertI32Signed(usize),
    F64ConvertI32Unsigned(usize),

    F64ConvertI64Signed(usize),
    F64ConvertI64Unsigned(usize),

    F64PromoteF32(usize),

    // REINTERPRETATIONS
    // (value: T) -> U
    I32ReinterpretF32(usize),
    I64ReinterpretF64(usize),
    F32ReinterpretI32(usize),
    F64ReinterpretI64(usize),
}
