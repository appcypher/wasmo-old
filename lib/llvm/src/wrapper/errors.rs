pub type CompilerResult<T> = Result<T, CompilerError>;

///
#[derive(Debug, PartialEq, Eq)]
pub enum CompilerError {
    FunctionLookUp(FunctionLookUp),
    TargetInit(TargetInit),
    ExecutionEngine(String),
    GetValue(GetValue),
}

//-------------- JIT LOOKUP --------------------//

///
#[derive(Debug, PartialEq, Eq)]
pub enum FunctionLookUp {
    JITNotEnabled,
    FunctionNotFound,
}

//-------------- TARGET INITIALIZATION --------------------//

///
#[derive(Debug, PartialEq, Eq)]
pub enum TargetInit {
    CantInitializeNativeTarget,
    CantInitializeNativeASMPrinter,
    CantInitializeNativeASMParser,
    CantInitializeNativeDisassembler,
}

//-------------- VALUES --------------------//

///
#[derive(Debug, PartialEq, Eq)]
pub enum GetValue {
    CantGetNthParam(u32),
    CantGetFirstParam,
    CantGetLastParam,
}
