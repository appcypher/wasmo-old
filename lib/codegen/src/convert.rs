//! Conversions from wasmparser types.
use wasmparser::{Type, FuncType as ParserFuncType, ExternalKind};
use wasmo_runtime::types::{FuncType, ValueType, ExportKind};
use wasmo_runtime::data::Data;
use wasmo_llvm::types::{BasicType, FunctionType, fn_type, PointerType};
use wasmo_llvm::{Context, AddressSpace};

use std::iter::once;

pub struct LLVM();

pub struct Runtime();

impl LLVM {
    pub fn func_type(context: &Context, context_type: &PointerType, ty: &ParserFuncType) -> Result<FunctionType, &'static str> {
        // TODO: Support multiple return values. LLVMStructType? Julia dev!
        let params = once(Ok(context_type.clone().into()))
            .chain(ty.params.iter().map(|ty| LLVM::basic_type(context, ty)))
            .collect::<Result<Vec<BasicType>, &'static str>>()?;

        let returns = match &*ty.returns {
            &[ ty ] => LLVM::basic_type(context, &ty)?,
            &[] => context.void_type().into(),
            _ => return Err("Multiple return values not supported yet!"),
        };

        Ok(fn_type(&params, returns, false))
    }

    pub fn basic_type(context: &Context, ty: &Type) -> Result<BasicType, &'static str> {
        Ok(match ty {
            Type::I32 => context.i32_type().into(),
            Type::I64 => context.i64_type().into(),
            Type::F32 => context.f32_type().into(),
            Type::F64 => context.f64_type().into(),
            _ => return Err("Expected the following types [i32, i64, f32, f64]"),
        })
    }
}

impl Runtime {
    pub fn func_type(func_type: &ParserFuncType) -> Result<FuncType, &'static str>  {
        let params = func_type.params.iter()
            .map(|ty| Runtime::value_type(ty))
            .collect::<Result<Vec<ValueType>, &'static str>>()?;

        let returns = func_type.returns.iter()
            .map(|ty| Runtime::value_type(ty))
            .collect::<Result<Vec<ValueType>, &'static str>>()?;

        Ok(FuncType::new(params, returns))
    }

    pub fn value_type(ty: &Type) -> Result<ValueType, &'static str> {
        Ok(match ty {
            Type::I32 => ValueType::I32,
            Type::I64 => ValueType::I64,
            Type::F32 => ValueType::F32,
            Type::F64 => ValueType::F64,
            _ => return Err("Expected the following types [i32, i64, f32, f64]"),
        })
    }

    pub fn export(kind: &ExternalKind, index: u32) -> ExportKind {
        match kind {
            ExternalKind::Memory => ExportKind::Memory(index),
            ExternalKind::Table => ExportKind::Table(index),
            ExternalKind::Global => ExportKind::Global(index),
            ExternalKind::Function => ExportKind::Function(index)
        }
    }
}
