use super::Value;

use llvm_sys::core::LLVMIsAInstruction;
use llvm_sys::prelude::LLVMValueRef;

use super::AsValueRef;

///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct InstructionValue {
    pub(crate) val: Value,
}

impl InstructionValue {
    pub(crate) fn new(instruction_value: LLVMValueRef) -> Self {
        debug_assert!(!instruction_value.is_null());
        debug_assert!(unsafe { !LLVMIsAInstruction(instruction_value).is_null() });

        Self {
            val: Value::new(instruction_value),
        }
    }
}

impl AsValueRef for InstructionValue {
    fn as_ref(&self) -> LLVMValueRef {
        self.val.val
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionOpcode {
    // Actual Instructions:
    Add,
    AddrSpaceCast,
    Alloca,
    And,
    AShr,
    AtomicCmpXchg,
    AtomicRMW,
    BitCast,
    Br,
    Call,
    CatchPad,
    CatchRet,
    CatchSwitch,
    CleanupPad,
    CleanupRet,
    ExtractElement,
    ExtractValue,
    FAdd,
    FCmp,
    FDiv,
    Fence,
    FMul,
    FPExt,
    FPToSI,
    FPToUI,
    FPTrunc,
    FRem,
    FSub,
    GetElementPtr,
    ICmp,
    IndirectBr,
    InsertElement,
    InsertValue,
    IntToPtr,
    Invoke,
    LandingPad,
    Load,
    LShr,
    Mul,
    Or,
    Phi,
    PtrToInt,
    Resume,
    Return,
    SDiv,
    Select,
    SExt,
    Shl,
    ShuffleVector,
    SIToFP,
    SRem,
    Store,
    Sub,
    Switch,
    Trunc,
    UDiv,
    UIToFP,
    Unreachable,
    URem,
    UserOp1,
    UserOp2,
    VAArg,
    Xor,
    ZExt,
}
