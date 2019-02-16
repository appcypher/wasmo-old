///
#[derive(Debug)]
pub enum Type {
    I32,
    I64,
    F32,
    F64,
    Anyfunc,
    AnyRef,
    Func,
    Empty,
}

///
pub struct FuncSig {
    params: Vec<Type>,
    returns: Vec<Type>,
}

///
pub struct Import {
    module_name: String,
    field_name: String,
    desc: ImportDesc,
}

///
pub enum ImportDesc {
    Function {
        type_ref: u32,
    },
    Table {
        elem_type: Type,
        initial: u32,
        maximum: Option<u32>,
    },
    Memory {
        initial: u32,
        maximum: Option<u32>,
    },
    Global {
        content_type: Type,
        mutability: bool,
    },
}

pub struct Module {
    sections: Vec<Section>
}

///
pub enum Section {
    Type(Vec<FuncSig>),
    Import(Vec<Import>),
    Function(Vec<u32>),
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code { locals: Vec<Local>, instructions: Vec<Operator> },
    Data,
}

///
pub struct Local {
    count: u32,
    local_type: Type,
}

///
pub enum Operator {
    // TODO
    I32Add { op0: i32, op1: i32 },
}
