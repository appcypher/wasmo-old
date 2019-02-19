///
#[derive(Debug, Clone)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    Anyfunc,
    AnyRef,
    Func {
        params: Vec<Type>,
        returns: Vec<Type>,
    },
    Empty,
}

///
#[derive(Debug, Clone)]
pub struct Import {
    module_name: String,
    field_name: String,
    desc: ImportDesc,
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
    element_type: Type,
    minimum: u32,
    maximum: Option<u32>,
}

///
#[derive(Debug, Clone)]
pub struct Memory {
    minimum: u32,
    maximum: Option<u32>,
}

///
#[derive(Debug, Clone)]
pub struct Global {
    content_type: Type,
    mutability: bool,
    instructions: Vec<Operator>,
}

///
#[derive(Debug, Clone)]
pub struct Function {
    locals: Vec<Local>,
    instructions: Vec<Operator>,
}

///
#[derive(Debug, Clone)]
pub struct Module {
    sections: Vec<Section>,
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
    Element(),
    Code(Vec<Function>),
    Data(),
    Custom,
}

///
#[derive(Debug, Clone)]
pub struct Local {
    count: u32,
    local_type: Type,
}

///
#[derive(Debug, Clone)]
pub enum Operator {
    // TODO
    I32Add { op0: i32, op1: i32 },
    Nop,
}

///
#[derive(Debug, Clone)]
pub struct Export {
    name: String,
    desc: ExportDesc,
}

///
#[derive(Debug, Clone)]
pub enum ExportDesc {
    Function(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}
