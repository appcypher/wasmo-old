#![allow(clippy::cognitive_complexity)]

use wasmparser::{Parser, ParserState, WasmDecoder};
use wasmo_utils::{verbose, debug};

#[derive(Debug)]
pub struct ParserError {
    message: &'static str,
    offset: usize,
}

pub type ParserResult<T> = Result<T, ParserError>;

pub fn generate_module(wasm_binary: &[u8]) -> ParserResult<()> {
    let mut parser = Parser::new(wasm_binary);

    // Imports
    // Tables & Elements (Initializer)
    // Memories & Data (Initializer)
    // Globals (Mutability, Initializer)
    // Start
    // Functions & Code
    // Exports

    loop {
        let state = parser.read();
        let mut function_index = 0;

        match state {
            // MISC.
            ParserState::BeginWasm { .. } => {
                verbose!("wasm parsing started!");
            },
            ParserState::EndWasm => {
                verbose!("wasm parsing concluded!");
                break
            },
            // ERRORS
            ParserState::Error(error) => {
                return Err(ParserError { message: error.message, offset: error.offset })
            },
            // TYPE
            ParserState::TypeSectionEntry(ty) => {
                debug!("type entry => {:?}", ty);
            },
            // IMPORT
            ParserState::ImportSectionEntry { module, field, ty } => {
                debug!("import entry type => {:?}, {:?}, {:?}", module, field, ty);
            },
            // EXPORT
            ParserState::ExportSectionEntry { field, kind, index } => {
                debug!("export entry type => {:?}, {:?}, {:?}", field, kind, index);
            },
            // MEMORY | TABLE
            ParserState::MemorySectionEntry(ty) => {
                debug!("memory entry type => {:?}", ty);
            },
            ParserState::TableSectionEntry(ty) => {
                debug!("table entry type => {:?}", ty);
            },
            // GLOBAL
            ParserState::BeginGlobalSectionEntry(ty) => {
                verbose!("global section started!");
            },
            ParserState::EndGlobalSectionEntry => {
                verbose!("global section started!");
            },
            // INIT EXPRESSION
            ParserState::BeginInitExpressionBody => {
                verbose!("init expression started!");
            },
            ParserState::EndInitExpressionBody => {
                verbose!("init expression concluded!");
            },
            ParserState::InitExpressionOperator(operator) => {
                debug!("init expression operator => {:?}", operator);
            },
            // ELEMENT
            ParserState::BeginPassiveElementSectionEntry(ty) => {
                verbose!("element section (passive) started! => {:?}", ty);
            },
            ParserState::BeginActiveElementSectionEntry(table_index) => {
                verbose!("element section (active) started! => {:?}", table_index);
            },
            ParserState::EndElementSectionEntry => {
                verbose!("element section concluded!");
            },
            ParserState::ElementSectionEntryBody(func_indices) => {
                debug!("element function indices => {:?}", func_indices);
            },
            // DATA
            ParserState::BeginPassiveDataSectionEntry => {
                verbose!("data section (passive) started!");
            },
            ParserState::BeginActiveDataSectionEntry(mem_index) => {
                verbose!("data section (active) started! => {:?}", mem_index);
            },
            ParserState::EndDataSectionEntryBody => {
                verbose!("data section concluded!");
            },
            ParserState::DataSectionEntryBodyChunk(bytes) => {
                debug!("data bytes => {:?}", bytes);
            },
            // FUNCTION
            ParserState::FunctionSectionEntry(type_index) => {
                debug!("function entry type => {:?}", type_index);
            },
            // FUNCTION BODY | CODE
            ParserState::BeginFunctionBody { .. } => {
                verbose!("function body started! function index => {}", function_index);
            },
            ParserState::EndFunctionBody => {
                verbose!("function body concluded! function index => {}", function_index);
                // Increment function index
                function_index += 1;
            },
            ParserState::FunctionBodyLocals { locals } => {
                debug!("function body locals => {:?}", locals);
            },
            ParserState::CodeOperator(operator) => {
                debug!("code operator => {:?}", operator);
            },
            // START
            ParserState::StartSectionEntry(func_index) => {
                debug!("start function index => {:?}", func_index);
            },
            _ => (),
        }
    }

    Ok(())
}

