#![allow(clippy::cognitive_complexity)]

use crate::{
    convert::Runtime,
    convert::LLVM,
    error::{Offset, ParserError, ParserResult},
    generator::FunctionGenerator,
    options::CodegenOptions,
};
use std::ptr::null;
use wasmo_llvm::target::{Target, TargetData};
use wasmo_llvm::types::{
    function_type, BasicType, FloatType, FunctionType, IntType, PointerType, StructType,
};
use wasmo_llvm::values::{BasicValue, FloatValue, FunctionValue, InstructionValue, IntValue};
use wasmo_llvm::{AddressSpace, BasicBlock, Builder, Context, InitializationConfig, Module};
use wasmo_llvm::{CodeModel, OptimizationLevel, RelocationModel};
use wasmo_runtime::data::{FuncData, ModuleData};
use wasmo_utils::{debug, verbose};
use wasmparser::{FuncType, Operator, Parser, ParserState, WasmDecoder};

///
pub struct Reusables {
    pub(crate) i8_type: IntType,
    pub(crate) i32_type: IntType,
    pub(crate) i64_type: IntType,
    pub(crate) f32_type: FloatType,
    pub(crate) f64_type: FloatType,
}

impl Reusables {
    ///
    fn new(context: &Context) -> Self {
        Self {
            i8_type: context.i8_type(),
            i32_type: context.i32_type(),
            i64_type: context.i64_type(),
            f32_type: context.f32_type(),
            f64_type: context.f64_type(),
        }
    }
}

///
pub struct ModuleGenerator<'a> {
    parser: Parser<'a>,
    instance_context_type: PointerType,
    context: Context,
    builder: Builder,
    function_types: Vec<FunctionType>,
    function_index: u32,
    options: CodegenOptions,
    reusables: Reusables,
}

impl<'a> ModuleGenerator<'a> {
    ///
    pub fn new(wasm_binary: &'a [u8], options: &CodegenOptions) -> Self {
        let context = Context::create();
        let llvm_target_data = ModuleGenerator::create_target_data();
        let builder = context.create_builder();
        let reusables = Reusables::new(&context);
        let instance_context_type =
            ModuleGenerator::create_instance_context_type(&context, &reusables, &llvm_target_data, );

        Self {
            context,
            parser: Parser::new(&wasm_binary),
            instance_context_type,
            builder,
            function_types: Vec::new(),
            function_index: 0,
            options: *options,
            reusables,
        }
    }

    ///
    fn create_target_data() -> TargetData {
        Target::initialize_native(&InitializationConfig::default())
            .expect("Unsuccessful initilalization of native target");

        let target_triple = Target::get_default_triple().to_string();

        let normalized_target_triple = Target::normalize_target_triple(&target_triple).to_string();

        let target = Target::from_triple(&normalized_target_triple)
            .expect("Unsuccessful creation of target from triple");

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "",
                "",
                OptimizationLevel::None,
                RelocationModel::Default,
                CodeModel::Default,
            )
            .expect("Unable to create target machine from target");
        target_machine.get_target_data()
    }

    /// This function creates an LLVM IR representing `*mut InstanceContext`.
    ///
    /// All wasm functions take an initial argument of type `*mut InstanceContext`.
    ///
    /// ```
    /// fn (*mut InstanceContext, ...) -> ...
    /// ```
    ///
    /// Conceptually, InstanceContext has the following type. Although, we cannot express dynamic
    /// arrays in Rust.
    ///
    /// ```rust
    /// struct InstanceContext {
    ///     memories_offset: usize,
    ///     tables_offset: usize,
    ///     globals_offset: usize,
    ///     functions_offset: usize,
    ///     intrinsic_function_offset: usize,
    ///     memories: dyn [*mut u8; memory_count],
    ///     tables: dyn [*mut u32; table_count],
    ///     globals: dyn [*mut u64; global_count],
    ///     functions: dyn [*const (); function_count],
    ///     intrinsic_functions: dyn [*const (); intrinsic_function_count],
    /// }
    /// ```
    fn create_instance_context_type(context: &Context, reusables: &Reusables, target_data: &TargetData) -> PointerType {
        let address_space = &AddressSpace::Global;
        let bound_ptr_ty = context.struct_type_with_name(
            "BoundPtr",
            &[
                context.i32_type().ptr_type(address_space).into(), // *mut u32
                context.machine_int_type(target_data, None).into(), // usize
            ],
            false,
        );
        let memories_ty: BasicType = reusables
            .i8_type
            .ptr_type(address_space)
            .ptr_type(address_space)
            .into(); // *mut *mut u8
        let tables_ty: BasicType = bound_ptr_ty.ptr_type(address_space).into(); // *mut struct BoundPtr
        let globals_ty: BasicType = reusables
            .i64_type
            .ptr_type(address_space)
            .ptr_type(address_space)
            .into(); // *mut *mut u64
        let functions_ty: BasicType = reusables
            .i8_type
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
    pub fn generate_module(&mut self) -> ParserResult<(Module, ModuleData)> {
        //
        let mut module = self.context.create_module("wasm");
        let mut runtime_data = ModuleData::new();

        loop {
            let state = self.parser.read();

            match state {
                // END
                ParserState::EndWasm => {
                    verbose!("Parser ended!");
                    // Generate `main` function.
                    let mut function_codegen = FunctionGenerator::new();
                    function_codegen.generate_main_function(
                        &mut module,
                        &self.builder,
                        &self.context,
                        &self.reusables,
                    )?;

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
                    runtime_data.add_type(Runtime::func_type(ty)?);
                    self.function_types.push(LLVM::func_type(
                        &self.context,
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
                    runtime_data.add_export(field.to_string(), Runtime::export(kind, *index));
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
                    runtime_data.add_function(FuncData::new(null() as _, *type_index));
                }
                // FUNCTION BODY | CODE
                ParserState::BeginFunctionBody { .. } => {
                    // Generate function.
                    let mut function_codegen = FunctionGenerator::new();
                    function_codegen.generate_function(
                        &mut module,
                        &mut self.parser,
                        &self.function_types,
                        &self.builder,
                        &self.context,
                        &self.reusables,
                        self.function_index,
                    )?;

                    self.function_index += 1;
                }
                _ => (),
            }
        }

        if cfg!(feature = "verbose") {
            let delim = std::iter::repeat("+").take(70).collect::<String>();
            println!("●{}●\n{}\n●{}●", delim, module, delim);
        }

        Ok((module, runtime_data))
    }
}
