use super::module::Reusables;
use crate::convert::LLVM;
use wasmo_llvm::types::{function_type, BasicType, FunctionType};
use wasmo_llvm::values::{BasicValue, FloatValue, FunctionValue, IntValue};
use wasmo_llvm::{Builder, Context, Module};
use wasmo_utils::{debug, verbose};
use wasmparser::{FuncType, Operator, Parser, ParserState, WasmDecoder};

///
#[derive(Debug)]
struct Local {
    ty: BasicType,
    current_ssa_reference: Option<BasicValue>,
}

impl Local {
    fn new(ty: &BasicType) -> Self {
        Self {
            ty: *ty,
            current_ssa_reference: None,
        }
    }
}

///
pub struct FunctionGenerator {
    stack: Vec<BasicValue>,
    locals: Vec<Local>,
}

impl FunctionGenerator {
    ///
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            locals: Vec::new(),
        }
    }

    ///
    pub fn generate_function<'b>(
        &mut self,
        module: &mut Module,
        parser: &mut Parser<'b>,
        function_types: &[FunctionType],
        builder: &Builder,
        context: &Context,
        reusables: &Reusables,
        index: u32,
    ) -> Result<(), &'static str> {
        //
        let function_type = function_types[index as usize];
        let function = module.add_function("", function_type, None);
        let basic_block = function.append_basic_block("entry", &context);
        builder.position_at_end(&basic_block);

        loop {
            //
            let state = parser.read();

            match state {
                ParserState::EndFunctionBody => {
                    // Create a function return type from what's left on stack.
                    match self.stack.as_slice() {
                        &[value] => builder.build_return(Some(value)),
                        &[] => builder.build_return(None),
                        _ => return Err("Multiple return values not supported yet"),
                    };

                    break;
                }
                ParserState::FunctionBodyLocals { locals } => {
                    // Get argument locals
                    for ty in function
                        .get_type()
                        .as_ref()
                        .unwrap()
                        .get_param_types()
                        .iter()
                    {
                        self.locals.push(Local::new(ty))
                    }

                    // Get body locals.
                    for data in locals.iter() {
                        for _ in 0..(data.0) {
                            self.locals
                                .push(Local::new(&LLVM::basic_type(context, &data.1)?))
                        }
                    }
                }
                ParserState::CodeOperator(operator) => {
                    self.generate_operator_code(operator, &function, builder, context, reusables)?;
                }
                _ => break,
            };
        }

        Ok(())
    }

    ///
    pub fn generate_operator_code(
        &mut self,
        operator: &Operator,
        function: &FunctionValue,
        builder: &Builder,
        context: &Context,
        reusables: &Reusables,
    ) -> Result<(), &'static str> {
        match operator {
            Operator::Unreachable => {}
            Operator::Nop => {}
            Operator::Block { ty } => {}
            Operator::Loop { ty } => {}
            Operator::If { ty } => {}
            Operator::Else => {}
            Operator::End => {}
            Operator::Br { relative_depth } => {}
            Operator::BrIf { relative_depth } => {}
            Operator::BrTable { table } => {}
            Operator::Return => {}
            Operator::Call { function_index } => {}
            Operator::CallIndirect { index, table_index } => {}
            Operator::Drop => {
                self.stack.pop();
            }
            Operator::Select => {}
            Operator::GetLocal { local_index } => {
                // Skip the first instance_context_type param
                let local_index = *local_index + 1;
                let local = &mut self.locals[local_index as usize];

                // If this is our first local.get for a specific index.
                if local.current_ssa_reference.is_none() {
                    local.current_ssa_reference = Some(
                        // If local is a param, get param, otherwise initialize to zero.
                        if local_index < function.count_params() {
                            function.get_nth_param(local_index).unwrap()
                        } else {
                            local.ty.zero(false)?
                        },
                    );
                }

                // Push value to stack.
                // Guarantee: The checks above ensure local holds a basic value.
                self.stack
                    .push(*local.current_ssa_reference.as_ref().unwrap());
            }
            Operator::SetLocal { local_index } => {
                // Skip the first instance_context_type param
                let local_index = *local_index + 1;
                // Guarantee: parser already done stack validation
                let stack_value = self.stack.pop().unwrap();
                let local = &mut self.locals[local_index as usize];
                local.current_ssa_reference = Some(stack_value);
            }
            Operator::TeeLocal { local_index } => {
                // Skip the first instance_context_type param
                let local_index = *local_index + 1;
                // Guarantee: parser already done stack validation
                let stack_value = self.stack.last().unwrap();
                let local = &mut self.locals[local_index as usize];
                local.current_ssa_reference = Some(stack_value.clone());
            }
            Operator::GetGlobal { global_index } => {}
            Operator::SetGlobal { global_index } => {}
            Operator::I32Load { memarg } => {}
            Operator::I64Load { memarg } => {}
            Operator::F32Load { memarg } => {}
            Operator::F64Load { memarg } => {}
            Operator::I32Load8S { memarg } => {}
            Operator::I32Load8U { memarg } => {}
            Operator::I32Load16S { memarg } => {}
            Operator::I32Load16U { memarg } => {}
            Operator::I64Load8S { memarg } => {}
            Operator::I64Load8U { memarg } => {}
            Operator::I64Load16S { memarg } => {}
            Operator::I64Load16U { memarg } => {}
            Operator::I64Load32S { memarg } => {}
            Operator::I64Load32U { memarg } => {}
            Operator::I32Store { memarg } => {}
            Operator::I64Store { memarg } => {}
            Operator::F32Store { memarg } => {}
            Operator::F64Store { memarg } => {}
            Operator::I32Store8 { memarg } => {}
            Operator::I32Store16 { memarg } => {}
            Operator::I64Store8 { memarg } => {}
            Operator::I64Store16 { memarg } => {}
            Operator::I64Store32 { memarg } => {}
            Operator::MemorySize { reserved } => {}
            Operator::MemoryGrow { reserved } => {}
            Operator::I32Const { value } => {
                let value: BasicValue = reusables.i32_type.const_int(*value as _, false).into();
                self.stack.push(value);
            }
            Operator::I64Const { value } => {
                let value: BasicValue = reusables.i64_type.const_int(*value as _, false).into();
                self.stack.push(value);
            }
            Operator::F32Const { value } => {
                let value: BasicValue = reusables
                    .f32_type
                    .const_float((value.bits() as f32).into())
                    .into();
                self.stack.push(value);
            }
            Operator::F64Const { value } => {
                let value: BasicValue = reusables.f64_type.const_float(value.bits() as f64).into();
                self.stack.push(value);
            }
            Operator::RefNull => {}
            Operator::RefIsNull => {}
            Operator::I32Eqz => {}
            Operator::I32Eq => {}
            Operator::I32Ne => {}
            Operator::I32LtS => {}
            Operator::I32LtU => {}
            Operator::I32GtS => {}
            Operator::I32GtU => {}
            Operator::I32LeS => {}
            Operator::I32LeU => {}
            Operator::I32GeS => {}
            Operator::I32GeU => {}
            Operator::I64Eqz => {}
            Operator::I64Eq => {}
            Operator::I64Ne => {}
            Operator::I64LtS => {}
            Operator::I64LtU => {}
            Operator::I64GtS => {}
            Operator::I64GtU => {}
            Operator::I64LeS => {}
            Operator::I64LeU => {}
            Operator::I64GeS => {}
            Operator::I64GeU => {}
            Operator::F32Eq => {}
            Operator::F32Ne => {}
            Operator::F32Lt => {}
            Operator::F32Gt => {}
            Operator::F32Le => {}
            Operator::F32Ge => {}
            Operator::F64Eq => {}
            Operator::F64Ne => {}
            Operator::F64Lt => {}
            Operator::F64Gt => {}
            Operator::F64Le => {}
            Operator::F64Ge => {}
            Operator::I32Clz => {}
            Operator::I32Ctz => {}
            Operator::I32Popcnt => {}
            Operator::I32Add => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_add::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i32.add",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I32Sub => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_sub::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i32.sub",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I32Mul => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_mul::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i32.mul",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I32DivS => {
                // Div by zero checks!
            }
            Operator::I32DivU => {}
            Operator::I32RemS => {}
            Operator::I32RemU => {}
            Operator::I32And => {}
            Operator::I32Or => {}
            Operator::I32Xor => {}
            Operator::I32Shl => {}
            Operator::I32ShrS => {}
            Operator::I32ShrU => {}
            Operator::I32Rotl => {}
            Operator::I32Rotr => {}
            Operator::I64Clz => {}
            Operator::I64Ctz => {}
            Operator::I64Popcnt => {}
            Operator::I64Add => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_add::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i64.add",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I64Sub => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_sub::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i64.sub",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I64Mul => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_int_mul::<IntValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "i64.mul",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::I64DivS => {}
            Operator::I64DivU => {}
            Operator::I64RemS => {}
            Operator::I64RemU => {}
            Operator::I64And => {}
            Operator::I64Or => {}
            Operator::I64Xor => {}
            Operator::I64Shl => {}
            Operator::I64ShrS => {}
            Operator::I64ShrU => {}
            Operator::I64Rotl => {}
            Operator::I64Rotr => {}
            Operator::F32Abs => {}
            Operator::F32Neg => {}
            Operator::F32Ceil => {}
            Operator::F32Floor => {}
            Operator::F32Trunc => {}
            Operator::F32Nearest => {}
            Operator::F32Sqrt => {}
            Operator::F32Add => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_add::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f32.add",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F32Sub => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_sub::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f32.sub",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F32Mul => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_mul::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f32.mul",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F32Div => {}
            Operator::F32Min => {}
            Operator::F32Max => {}
            Operator::F32Copysign => {}
            Operator::F64Abs => {}
            Operator::F64Neg => {}
            Operator::F64Ceil => {}
            Operator::F64Floor => {}
            Operator::F64Trunc => {}
            Operator::F64Nearest => {}
            Operator::F64Sqrt => {}
            Operator::F64Add => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_add::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f64.add",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F64Sub => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_sub::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f64.sub",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F64Mul => {
                // Guarantee: parser already type checked stack values.
                let value: BasicValue = builder
                    .build_float_sub::<FloatValue>(
                        self.stack.pop().unwrap().into(),
                        self.stack.pop().unwrap().into(),
                        "f32.mul",
                    )
                    .into();
                self.stack.push(value);
            }
            Operator::F64Div => {
                // Div by zero checks!
            }
            Operator::F64Min => {}
            Operator::F64Max => {}
            Operator::F64Copysign => {}
            Operator::I32WrapI64 => {}
            Operator::I32TruncSF32 => {}
            Operator::I32TruncUF32 => {}
            Operator::I32TruncSF64 => {}
            Operator::I32TruncUF64 => {}
            Operator::I64ExtendSI32 => {}
            Operator::I64ExtendUI32 => {}
            Operator::I64TruncSF32 => {}
            Operator::I64TruncUF32 => {}
            Operator::I64TruncSF64 => {}
            Operator::I64TruncUF64 => {}
            Operator::F32ConvertSI32 => {}
            Operator::F32ConvertUI32 => {}
            Operator::F32ConvertSI64 => {}
            Operator::F32ConvertUI64 => {}
            Operator::F32DemoteF64 => {}
            Operator::F64ConvertSI32 => {}
            Operator::F64ConvertUI32 => {}
            Operator::F64ConvertSI64 => {}
            Operator::F64ConvertUI64 => {}
            Operator::F64PromoteF32 => {}
            Operator::I32ReinterpretF32 => {}
            Operator::I64ReinterpretF64 => {}
            Operator::F32ReinterpretI32 => {}
            Operator::F64ReinterpretI64 => {}
            Operator::I32Extend8S => {}
            Operator::I32Extend16S => {}
            Operator::I64Extend8S => {}
            Operator::I64Extend16S => {}
            Operator::I64Extend32S => {}

            // 0xFC operators
            // Non-trapping Float-to-int Conversions
            Operator::I32TruncSSatF32 => {}
            Operator::I32TruncUSatF32 => {}
            Operator::I32TruncSSatF64 => {}
            Operator::I32TruncUSatF64 => {}
            Operator::I64TruncSSatF32 => {}
            Operator::I64TruncUSatF32 => {}
            Operator::I64TruncSSatF64 => {}
            Operator::I64TruncUSatF64 => {}

            // 0xFC operators
            // bulk memory https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md
            Operator::MemoryInit { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::DataDrop { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::MemoryCopy => return Err("Bulk memory ops not yet supported!"),
            Operator::MemoryFill => return Err("Bulk memory ops not yet supported!"),
            Operator::TableInit { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::ElemDrop { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::TableCopy => return Err("Bulk memory ops not yet supported!"),
            Operator::TableGet { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::TableSet { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::TableGrow { .. } => return Err("Bulk memory ops not yet supported!"),
            Operator::TableSize { .. } => return Err("Bulk memory ops not yet supported!"),

            // 0xFE operators
            // https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md
            Operator::Wake { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32Wait { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64Wait { .. } => return Err("Thread ops not yet suported!"),
            // Operator::Fence { flags: u8 } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicLoad { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicLoad { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicLoad8U { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicLoad16U { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicLoad8U { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicLoad16U { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicLoad32U { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicStore { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicStore { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicStore8 { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicStore16 { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicStore8 { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicStore16 { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicStore32 { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UAdd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwSub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwSub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8USub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16USub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8USub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16USub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32USub { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UAnd { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UOr { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UXor { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UXchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmwCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmwCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw8UCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I32AtomicRmw16UCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw8UCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw16UCmpxchg { .. } => return Err("Thread ops not yet suported!"),
            Operator::I64AtomicRmw32UCmpxchg { .. } => return Err("Thread ops not yet suported!"),

            // 0xFD operators
            // SIMD https://github.com/WebAssembly/simd/blob/master/proposals/simd/BinarySIMD.md
            Operator::V128Load { .. } => return Err("SIMD ops not yet supported"),
            Operator::V128Store { .. } => return Err("SIMD ops not yet supported"),
            Operator::V128Const { value: V128 } => return Err("SIMD ops not yet supported"),
            Operator::I8x16Splat => return Err("SIMD ops not yet supported"),
            Operator::I8x16ExtractLaneS { .. } => return Err("SIMD ops not yet supported"),
            Operator::I8x16ExtractLaneU { .. } => return Err("SIMD ops not yet supported"),
            Operator::I8x16ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I16x8Splat => return Err("SIMD ops not yet supported"),
            Operator::I16x8ExtractLaneS { .. } => return Err("SIMD ops not yet supported"),
            Operator::I16x8ExtractLaneU { .. } => return Err("SIMD ops not yet supported"),
            Operator::I16x8ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I32x4Splat => return Err("SIMD ops not yet supported"),
            Operator::I32x4ExtractLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I32x4ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I64x2Splat => return Err("SIMD ops not yet supported"),
            Operator::I64x2ExtractLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I64x2ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::F32x4Splat => return Err("SIMD ops not yet supported"),
            Operator::F32x4ExtractLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::F32x4ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::F64x2Splat => return Err("SIMD ops not yet supported"),
            Operator::F64x2ExtractLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::F64x2ReplaceLane { .. } => return Err("SIMD ops not yet supported"),
            Operator::I8x16Eq => return Err("SIMD ops not yet supported"),
            Operator::I8x16Ne => return Err("SIMD ops not yet supported"),
            Operator::I8x16LtS => return Err("SIMD ops not yet supported"),
            Operator::I8x16LtU => return Err("SIMD ops not yet supported"),
            Operator::I8x16GtS => return Err("SIMD ops not yet supported"),
            Operator::I8x16GtU => return Err("SIMD ops not yet supported"),
            Operator::I8x16LeS => return Err("SIMD ops not yet supported"),
            Operator::I8x16LeU => return Err("SIMD ops not yet supported"),
            Operator::I8x16GeS => return Err("SIMD ops not yet supported"),
            Operator::I8x16GeU => return Err("SIMD ops not yet supported"),
            Operator::I16x8Eq => return Err("SIMD ops not yet supported"),
            Operator::I16x8Ne => return Err("SIMD ops not yet supported"),
            Operator::I16x8LtS => return Err("SIMD ops not yet supported"),
            Operator::I16x8LtU => return Err("SIMD ops not yet supported"),
            Operator::I16x8GtS => return Err("SIMD ops not yet supported"),
            Operator::I16x8GtU => return Err("SIMD ops not yet supported"),
            Operator::I16x8LeS => return Err("SIMD ops not yet supported"),
            Operator::I16x8LeU => return Err("SIMD ops not yet supported"),
            Operator::I16x8GeS => return Err("SIMD ops not yet supported"),
            Operator::I16x8GeU => return Err("SIMD ops not yet supported"),
            Operator::I32x4Eq => return Err("SIMD ops not yet supported"),
            Operator::I32x4Ne => return Err("SIMD ops not yet supported"),
            Operator::I32x4LtS => return Err("SIMD ops not yet supported"),
            Operator::I32x4LtU => return Err("SIMD ops not yet supported"),
            Operator::I32x4GtS => return Err("SIMD ops not yet supported"),
            Operator::I32x4GtU => return Err("SIMD ops not yet supported"),
            Operator::I32x4LeS => return Err("SIMD ops not yet supported"),
            Operator::I32x4LeU => return Err("SIMD ops not yet supported"),
            Operator::I32x4GeS => return Err("SIMD ops not yet supported"),
            Operator::I32x4GeU => return Err("SIMD ops not yet supported"),
            Operator::F32x4Eq => return Err("SIMD ops not yet supported"),
            Operator::F32x4Ne => return Err("SIMD ops not yet supported"),
            Operator::F32x4Lt => return Err("SIMD ops not yet supported"),
            Operator::F32x4Gt => return Err("SIMD ops not yet supported"),
            Operator::F32x4Le => return Err("SIMD ops not yet supported"),
            Operator::F32x4Ge => return Err("SIMD ops not yet supported"),
            Operator::F64x2Eq => return Err("SIMD ops not yet supported"),
            Operator::F64x2Ne => return Err("SIMD ops not yet supported"),
            Operator::F64x2Lt => return Err("SIMD ops not yet supported"),
            Operator::F64x2Gt => return Err("SIMD ops not yet supported"),
            Operator::F64x2Le => return Err("SIMD ops not yet supported"),
            Operator::F64x2Ge => return Err("SIMD ops not yet supported"),
            Operator::V128Not => return Err("SIMD ops not yet supported"),
            Operator::V128And => return Err("SIMD ops not yet supported"),
            Operator::V128Or => return Err("SIMD ops not yet supported"),
            Operator::V128Xor => return Err("SIMD ops not yet supported"),
            Operator::V128Bitselect => return Err("SIMD ops not yet supported"),
            Operator::I8x16Neg => return Err("SIMD ops not yet supported"),
            Operator::I8x16AnyTrue => return Err("SIMD ops not yet supported"),
            Operator::I8x16AllTrue => return Err("SIMD ops not yet supported"),
            Operator::I8x16Shl => return Err("SIMD ops not yet supported"),
            Operator::I8x16ShrS => return Err("SIMD ops not yet supported"),
            Operator::I8x16ShrU => return Err("SIMD ops not yet supported"),
            Operator::I8x16Add => return Err("SIMD ops not yet supported"),
            Operator::I8x16AddSaturateS => return Err("SIMD ops not yet supported"),
            Operator::I8x16AddSaturateU => return Err("SIMD ops not yet supported"),
            Operator::I8x16Sub => return Err("SIMD ops not yet supported"),
            Operator::I8x16SubSaturateS => return Err("SIMD ops not yet supported"),
            Operator::I8x16SubSaturateU => return Err("SIMD ops not yet supported"),
            Operator::I8x16Mul => return Err("SIMD ops not yet supported"),
            Operator::I16x8Neg => return Err("SIMD ops not yet supported"),
            Operator::I16x8AnyTrue => return Err("SIMD ops not yet supported"),
            Operator::I16x8AllTrue => return Err("SIMD ops not yet supported"),
            Operator::I16x8Shl => return Err("SIMD ops not yet supported"),
            Operator::I16x8ShrS => return Err("SIMD ops not yet supported"),
            Operator::I16x8ShrU => return Err("SIMD ops not yet supported"),
            Operator::I16x8Add => return Err("SIMD ops not yet supported"),
            Operator::I16x8AddSaturateS => return Err("SIMD ops not yet supported"),
            Operator::I16x8AddSaturateU => return Err("SIMD ops not yet supported"),
            Operator::I16x8Sub => return Err("SIMD ops not yet supported"),
            Operator::I16x8SubSaturateS => return Err("SIMD ops not yet supported"),
            Operator::I16x8SubSaturateU => return Err("SIMD ops not yet supported"),
            Operator::I16x8Mul => return Err("SIMD ops not yet supported"),
            Operator::I32x4Neg => return Err("SIMD ops not yet supported"),
            Operator::I32x4AnyTrue => return Err("SIMD ops not yet supported"),
            Operator::I32x4AllTrue => return Err("SIMD ops not yet supported"),
            Operator::I32x4Shl => return Err("SIMD ops not yet supported"),
            Operator::I32x4ShrS => return Err("SIMD ops not yet supported"),
            Operator::I32x4ShrU => return Err("SIMD ops not yet supported"),
            Operator::I32x4Add => return Err("SIMD ops not yet supported"),
            Operator::I32x4Sub => return Err("SIMD ops not yet supported"),
            Operator::I32x4Mul => return Err("SIMD ops not yet supported"),
            Operator::I64x2Neg => return Err("SIMD ops not yet supported"),
            Operator::I64x2AnyTrue => return Err("SIMD ops not yet supported"),
            Operator::I64x2AllTrue => return Err("SIMD ops not yet supported"),
            Operator::I64x2Shl => return Err("SIMD ops not yet supported"),
            Operator::I64x2ShrS => return Err("SIMD ops not yet supported"),
            Operator::I64x2ShrU => return Err("SIMD ops not yet supported"),
            Operator::I64x2Add => return Err("SIMD ops not yet supported"),
            Operator::I64x2Sub => return Err("SIMD ops not yet supported"),
            Operator::F32x4Abs => return Err("SIMD ops not yet supported"),
            Operator::F32x4Neg => return Err("SIMD ops not yet supported"),
            Operator::F32x4Sqrt => return Err("SIMD ops not yet supported"),
            Operator::F32x4Add => return Err("SIMD ops not yet supported"),
            Operator::F32x4Sub => return Err("SIMD ops not yet supported"),
            Operator::F32x4Mul => return Err("SIMD ops not yet supported"),
            Operator::F32x4Div => return Err("SIMD ops not yet supported"),
            Operator::F32x4Min => return Err("SIMD ops not yet supported"),
            Operator::F32x4Max => return Err("SIMD ops not yet supported"),
            Operator::F64x2Abs => return Err("SIMD ops not yet supported"),
            Operator::F64x2Neg => return Err("SIMD ops not yet supported"),
            Operator::F64x2Sqrt => return Err("SIMD ops not yet supported"),
            Operator::F64x2Add => return Err("SIMD ops not yet supported"),
            Operator::F64x2Sub => return Err("SIMD ops not yet supported"),
            Operator::F64x2Mul => return Err("SIMD ops not yet supported"),
            Operator::F64x2Div => return Err("SIMD ops not yet supported"),
            Operator::F64x2Min => return Err("SIMD ops not yet supported"),
            Operator::F64x2Max => return Err("SIMD ops not yet supported"),
            Operator::I32x4TruncSF32x4Sat => return Err("SIMD ops not yet supported"),
            Operator::I32x4TruncUF32x4Sat => return Err("SIMD ops not yet supported"),
            Operator::I64x2TruncSF64x2Sat => return Err("SIMD ops not yet supported"),
            Operator::I64x2TruncUF64x2Sat => return Err("SIMD ops not yet supported"),
            Operator::F32x4ConvertSI32x4 => return Err("SIMD ops not yet supported"),
            Operator::F32x4ConvertUI32x4 => return Err("SIMD ops not yet supported"),
            Operator::F64x2ConvertSI64x2 => return Err("SIMD ops not yet supported"),
            Operator::F64x2ConvertUI64x2 => return Err("SIMD ops not yet supported"),
            // Operator::V8x16Swizzle => return Err("SIMD ops not yet supported"),
            Operator::V8x16Shuffle { .. } => return Err("SIMD ops not yet supported"),
            // Operator::I8x16LoadSplat { .. } => return Err("SIMD ops not yet supported"),
            // Operator::I16x8LoadSplat { .. } => return Err("SIMD ops not yet supported"),
            // Operator::I32x4LoadSplat { .. } => return Err("SIMD ops not yet supported"),
            // Operator::I64x2LoadSplat { .. } => return Err("SIMD ops not yet supported"),
        }

        Ok(())
    }

    ///
    pub fn generate_main_function(
        &mut self,
        module: &mut Module,
        builder: &Builder,
        context: &Context,
        reusables: &Reusables,
    ) -> Result<(), &'static str> {
        //
        let function_type = function_type(&[], reusables.i32_type.into(), false);
        let function = module.add_function("main", function_type, None);
        let basic_block = function.append_basic_block("entry", &context);

        builder.position_at_end(&basic_block);

        // TODO: Abstraction ahead!

        // CREATIONS
        // Create memories -> call mmap(...) dynamic libc.dylib | call VirtualAlloc(...) dynamic win32.dll
        // Create global -> build_global
        // Create tables -> call mmap(...) dynamic libc.dylib | call VirtualAlloc(...) dynamic win32.dll

        // INITIALIZATIONS
        // initialize memories -> loop and place const values
        // initialize tables -> loop and place const values

        // INSTANCECONTEXT
        // create global -> build_global instancecontext
        // assign to instancecontext -> assign pointer values from CREATIONS to fields
        // assign to instancecontext.functions -> get address of each function

        // START
        // Call start -> call start

        builder.build_return(Some(reusables.i32_type.const_int(0, false).into()));

        Ok(())
    }
}
