///
#[derive(Debug, Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    FuncRef,
    Func {
        params: Vec<Type>,
        returns: Vec<Type>,
    },
    Empty,
}

impl From<i8> for Type {
    fn from(value: i8) -> Self {
        match value {
            -0x01 => Type::I32,
            -0x02 => Type::I64,
            -0x03 => Type::F32,
            -0x04 => Type::F64,
            -0x10 => Type::FuncRef,
            -0x20 => Type::Func {
                params: vec![],
                returns: vec![],
            },
            -0x40 => Type::Empty,
            _ => unreachable!(),
        }
    }
}

///
#[derive(Debug, Clone)]
pub struct Module {
    pub sections: Vec<Section>,
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
        content_type: Type,
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
    pub content_type: Type,
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
    pub local_type: Type,
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
    // TODO
    I32Add { op0: i32, op1: i32 },
    End,
    Nop,
    Unreachable,
}
