use llvm_sys::target_machine::{LLVMTargetMachineRef, LLVMTargetRef};

use super::{errors::TargetInit, CompilerError, CompilerResult};

///
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InitializationConfig {
    pub asm_parser: bool,
    pub asm_printer: bool,
    pub base: bool,
    pub disassembler: bool,
    pub info: bool,
    pub machine_code: bool,
}

impl Default for InitializationConfig {
    fn default() -> Self {
        InitializationConfig {
            asm_parser: true,
            asm_printer: true,
            base: true,
            disassembler: true,
            info: true,
            machine_code: true,
        }
    }
}

/// Contains data about the target to compile for
/// Also methods that are used to initialize a target.
#[derive(Debug, Eq, PartialEq)]
pub struct Target {
    target: LLVMTargetRef,
}

impl Target {
    fn new(target: LLVMTargetRef) -> Self {
        assert!(!target.is_null());

        Target { target }
    }

    ///
    pub fn initialize_native(config: &InitializationConfig) -> CompilerResult<()> {
        use llvm_sys::target::{
            LLVM_InitializeNativeAsmParser, LLVM_InitializeNativeAsmPrinter,
            LLVM_InitializeNativeDisassembler, LLVM_InitializeNativeTarget,
        };

        if config.base {
            let code = unsafe { LLVM_InitializeNativeTarget() };

            if code == 1 {
                return Err(CompilerError::TargetInit(
                    TargetInit::CantInitializeNativeTarget,
                ));
            }
        }

        if config.asm_printer {
            let code = unsafe { LLVM_InitializeNativeAsmPrinter() };

            if code == 1 {
                return Err(CompilerError::TargetInit(
                    TargetInit::CantInitializeNativeASMPrinter,
                ));
            }
        }

        if config.asm_parser {
            let code = unsafe { LLVM_InitializeNativeAsmParser() };

            if code == 1 {
                return Err(CompilerError::TargetInit(
                    TargetInit::CantInitializeNativeASMParser,
                ));
            }
        }

        if config.disassembler {
            let code = unsafe { LLVM_InitializeNativeDisassembler() };

            if code == 1 {
                return Err(CompilerError::TargetInit(
                    TargetInit::CantInitializeNativeDisassembler,
                ));
            }
        }

        Ok(())
    }

    ///
    pub fn initialize_x86(config: &InitializationConfig) {
        use llvm_sys::target::{
            LLVMInitializeX86AsmParser, LLVMInitializeX86AsmPrinter, LLVMInitializeX86Disassembler,
            LLVMInitializeX86Target, LLVMInitializeX86TargetInfo, LLVMInitializeX86TargetMC,
        };

        unsafe {
            if config.base {
                LLVMInitializeX86Target()
            }

            if config.info {
                LLVMInitializeX86TargetInfo()
            }

            if config.asm_printer {
                LLVMInitializeX86AsmPrinter()
            }

            if config.asm_parser {
                LLVMInitializeX86AsmParser()
            }

            if config.disassembler {
                LLVMInitializeX86Disassembler()
            }

            if config.machine_code {
                LLVMInitializeX86TargetMC()
            }
        }
    }

    ///
    pub fn initialize_all(config: &InitializationConfig) {
        use llvm_sys::target::{
            LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters,
            LLVM_InitializeAllDisassemblers, LLVM_InitializeAllTargetInfos,
            LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
        };

        unsafe {
            if config.base {
                LLVM_InitializeAllTargets()
            }

            if config.info {
                LLVM_InitializeAllTargetInfos()
            }

            if config.asm_parser {
                LLVM_InitializeAllAsmParsers()
            }

            if config.asm_printer {
                LLVM_InitializeAllAsmPrinters()
            }

            if config.disassembler {
                LLVM_InitializeAllDisassemblers()
            }

            if config.machine_code {
                LLVM_InitializeAllTargetMCs()
            }
        }
    }
}

/// Contains data holding target-specific information like the triple and methods for getting this information.
pub struct TargetMachine {}
