#![allow(clippy::cognitive_complexity)]

use crate::{
    convert::Runtime, convert::LLVM, options::CodegenOptions,
    generator::{FunctionGenerator},
    error::{Offset, ParserError,ParserResult}
};
use std::ptr::null;
use wasmo_llvm::target::{Target, TargetData};
use wasmo_llvm::types::{
    fn_type, BasicType, FloatType, FunctionType, IntType, PointerType, StructType,
};
use wasmo_llvm::values::{BasicValue, FloatValue, FunctionValue, InstructionValue, IntValue};
use wasmo_llvm::{AddressSpace, BasicBlock, Builder, Context, InitializationConfig, Module};
use wasmo_llvm::{CodeModel, OptimizationLevel, RelocationModel};
use wasmo_runtime::data::{FuncData, ModuleData};
use wasmo_utils::{debug, verbose};
use wasmparser::{FuncType, Operator, Parser, ParserState, WasmDecoder};

// TODO: MODULARIZATION
// at section entry fork parserstate, binary remains shared,
// generate_type(), generate_memory(), generate_function(), etc. Live in separate dirs.
pub struct ModuleGenerator<'a> {
    parser: Parser<'a>,
    instance_context_type: PointerType,
    runtime_data: ModuleData,
    llvm_context: Context,
    llvm_builder: Builder,
    llvm_func_types: Vec<FunctionType>,
    function_index: u32,
    options: CodegenOptions,
}

impl<'a> ModuleGenerator<'a> {
    ///
    pub fn new(wasm_binary: &'a [u8], options: &CodegenOptions) -> Self {
        let llvm_context = Context::create();
        let llvm_target_data = ModuleGenerator::create_target_data();
        let llvm_builder = llvm_context.create_builder();
        let instance_context_type =
            ModuleGenerator::create_instance_context_type(&llvm_context, &llvm_target_data);

        Self {
            llvm_context,
            parser: Parser::new(&wasm_binary),
            instance_context_type,
            runtime_data: ModuleData::new(),
            llvm_builder,
            llvm_func_types: Vec::new(),
            function_index: 0,
            options: *options,
        }
    }

    ///
    fn create_target_data() -> TargetData {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Unsuccessful initilalization of native target");
        let target_triple = Target::normalize_target_triple(Target::get_default_triple());
        let target = Target::from_triple(target_triple)
            .expect("Unsuccessful creation of target from triple");
        let target_machine = target
            .create_target_machine(
                target_triple,
                "",
                "",
                OptimizationLevel::None,
                RelocationModel::Default,
                CodeModel::Default,
            )
            .expect("Unable to create target machine from target");
        target_machine.get_target_data()
    }

    /// Every local wasm function has an extra initial argument of type `*mut InstanceContext`
    ///
    /// ```fn (*mut InstanceContext, ...) -> ...```
    ///
    /// This function creates an LLVM IR representing `*mut InstanceContext`.
    ///
    /// The structure of InstanceContext:
    ///
    /// ```
    /// struct InstanceContext {
    ///     memories: *mut *mut u8,
    ///     tables: *mut struct BoundPtr {
    ///         base_ptr: *mut u32,
    ///         size: usize,
    ///     },
    ///     globals: *mut *mut u64,
    ///     functions: *mut *const (),
    /// }
    /// ```
    fn create_instance_context_type(context: &Context, target_data: &TargetData) -> PointerType {
        let address_space = &AddressSpace::Global;
        let bound_ptr_ty = context.struct_type_with_name(
            "BoundPtr",
            &[
                context.i32_type().ptr_type(address_space).into(), // *mut u32
                context.machine_int_type(target_data, None).into(), // usize
            ],
            false,
        );
        let memories_ty: BasicType = context
            .i8_type()
            .ptr_type(address_space)
            .ptr_type(address_space)
            .into(); // *mut *mut u8
        let tables_ty: BasicType = bound_ptr_ty.ptr_type(address_space).into(); // *mut struct BoundPtr
        let globals_ty: BasicType = context
            .i64_type()
            .ptr_type(address_space)
            .ptr_type(address_space)
            .into(); // *mut *mut u64
        let functions_ty: BasicType = context
            .i8_type()
            .ptr_type(address_space)
            .ptr_type(address_space)
            .into(); // *mut *const i8 // LLVM doesn't like void pointers

        context
            .struct_type_with_name(
                "InstanceContext",
                &[memories_ty, tables_ty, globals_ty, functions_ty],
                false,
            )
            .ptr_type(address_space)
    }

    ///
    pub fn generate_module(&mut self) -> ParserResult<Module> {
        let module = self.llvm_context.create_module("wasm");

        loop {
            let state = self.parser.read();

            match state {
                // END
                ParserState::EndWasm => {
                    verbose!("Parser ended!");
                    break;
                }
                // ERRORS
                ParserState::Error(error) => {
                    return Err(ParserError::new(
                        error.message,
                        Offset::Number(error.offset),
                    ))
                }
                // TYPE
                ParserState::TypeSectionEntry(ty) => {
                    verbose!("type entry => {:?}", ty);
                    self.runtime_data.add_type(Runtime::func_type(ty)?);
                    self.llvm_func_types.push(LLVM::func_type(
                        &self.llvm_context,
                        &self.instance_context_type,
                        ty,
                    )?);
                }
                // IMPORT
                ParserState::ImportSectionEntry { module, field, ty } => {
                    debug!("import entry type => {:?}, {:?}, {:?}", module, field, ty);

                }
                // EXPORT
                ParserState::ExportSectionEntry { field, kind, index } => {
                    debug!("export entry type => {:?}, {:?}, {:?}", field, kind, index);
                    self.runtime_data
                        .add_export(field.to_string(), Runtime::export(kind, *index));
                }
                // MEMORY
                ParserState::MemorySectionEntry(ty) => {
                    debug!("memory entry type => {:?}", ty);
                }
                // TABLE
                ParserState::TableSectionEntry(ty) => {
                    debug!("table entry type => {:?}", ty);
                }
                // GLOBAL
                ParserState::BeginGlobalSectionEntry(ty) => {
                    verbose!("global section started!");
                }
                ParserState::EndGlobalSectionEntry => {
                    verbose!("global section started!");
                }
                // INIT EXPRESSION
                ParserState::BeginInitExpressionBody => {
                    verbose!("init expression started!");
                }
                ParserState::EndInitExpressionBody => {
                    verbose!("init expression concluded!");
                }
                ParserState::InitExpressionOperator(operator) => {
                    debug!("init expression operator => {:?}", operator);
                }
                // ELEMENT
                ParserState::BeginPassiveElementSectionEntry(ty) => {
                    verbose!("element section (passive) started! => {:?}", ty);
                }
                ParserState::BeginActiveElementSectionEntry(table_index) => {
                    verbose!("element section (active) started! => {:?}", table_index);
                }
                ParserState::EndElementSectionEntry => {
                    verbose!("element section concluded!");
                }
                ParserState::ElementSectionEntryBody(func_indices) => {
                    debug!("element function indices => {:?}", func_indices);
                }
                // DATA
                ParserState::BeginPassiveDataSectionEntry => {
                    verbose!("data section (passive) started!");
                }
                ParserState::BeginActiveDataSectionEntry(mem_index) => {
                    verbose!("data section (active) started! => {:?}", mem_index);
                }
                ParserState::EndDataSectionEntryBody => {
                    verbose!("data section concluded!");
                }
                ParserState::DataSectionEntryBodyChunk(bytes) => {
                    debug!("data bytes => {:?}", bytes);
                }
                // START
                ParserState::StartSectionEntry(func_index) => {
                    debug!("start function index => {:?}", func_index);
                }
                // FUNCTION
                ParserState::FunctionSectionEntry(type_index) => {
                    self.runtime_data
                        .add_function(FuncData::new(null() as _, *type_index));
                }
                // FUNCTION BODY | CODE
                ParserState::BeginFunctionBody { .. } => {
                    let func_type = self.llvm_func_types[self.function_index as usize];

                    let function = module.add_function("", func_type, None);

                    let basic_block = function.append_basic_block("entry", &self.llvm_context);

                    self.llvm_builder.position_at_end(&basic_block);

                    let mut function_codegen = FunctionGenerator::new(
                        self.function_index,
                        function,
                        &self.llvm_builder,
                        &self.llvm_context,
                    );

                    function_codegen.generate_function(&mut self.parser)?;

                    self.function_index += 1;
                }
                _ => (),
            }
        }

        if cfg!(feature = "verbose") {
            let delim = std::iter::repeat("+").take(70).collect::<String>();
            println!("●{}●\n{}\n●{}●", delim, module, delim);
        }

        Ok(module)
    }

    pub fn generate_main_function() -> FunctionValue {
        // _main function
        unimplemented!()
    }

    pub fn generate_initializers() -> FunctionValue {
        // memories, tables, globals
        unimplemented!()
    }
}
