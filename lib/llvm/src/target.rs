use llvm_sys::target_machine::{
    LLVMCreateTargetDataLayout, LLVMCreateTargetMachine, LLVMDisposeTargetMachine,
    LLVMGetDefaultTargetTriple, LLVMGetTargetDescription, LLVMGetTargetFromTriple,
    LLVMNormalizeTargetTriple, LLVMTargetMachineRef, LLVMTargetRef,
};

use llvm_sys::target::{
    LLVMCreateTargetData, LLVMDisposeTargetData, LLVMIntPtrType, LLVMIntPtrTypeForAS,
    LLVMTargetDataRef,
};

use super::{errors::TargetInit, CompilerError, CompilerResult};

use crate::types::IntType;

use crate::enums::{CodeModel, OptimizationLevel, RelocationModel};

use crate::AddressSpace;

use std::ffi::{CStr, CString};

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

    pub fn from_triple(triple: &str) -> CompilerResult<Self> {
        let c_string = CString::new(triple)
            .expect("Conversion of triple string to cstring failed unexpectedly");

        let mut target = std::ptr::null_mut();

        let mut error_string = unsafe { std::mem::zeroed() };

        let error_code =
            unsafe { LLVMGetTargetFromTriple(c_string.as_ptr(), &mut target, &mut error_string) };

        if error_code != 0 {
            let error_string = unsafe {
                CStr::from_ptr(error_string)
                    .to_str()
                    .expect("Conversion of error string from cstring failed unexpectedly")
            };
            return Err(CompilerError::TargetInit(
                TargetInit::CantCreateTargetFromTriple(error_string),
            ));
        }

        Ok(Target::new(target))
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

    pub fn get_default_triple() -> &'static str {
        unsafe {
            CStr::from_ptr(LLVMGetDefaultTargetTriple())
                .to_str()
                .expect("Conversion of default triple string from cstring failed unexpectedly")
        }
    }

    pub fn normalize_target_triple(triple: &str) -> &str {
        let c_string = CString::new(triple)
            .expect("Conversion of triple string to cstring failed unexpectedly");

        unsafe {
            CStr::from_ptr(LLVMNormalizeTargetTriple(c_string.as_ptr()))
                .to_str()
                .expect("Conversion of triple string from cstring failed unexpectedly")
        }
    }

    pub fn create_target_machine(
        &self,
        triple: &str,
        cpu: &str,
        features: &str,
        level: OptimizationLevel,
        reloc_mode: RelocationModel,
        code_model: CodeModel,
    ) -> Option<TargetMachine> {
        let triple = CString::new(triple)
            .expect("Conversion of triple string to cstring failed unexpectedly");
        let cpu =
            CString::new(cpu).expect("Conversion of cpu string to cstring failed unexpectedly");
        let features = CString::new(features)
            .expect("Conversion of features string to cstring failed unexpectedly");

        let target_machine = unsafe {
            LLVMCreateTargetMachine(
                self.target,
                triple.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                level.to_llvm(),
                reloc_mode.to_llvm(),
                code_model.to_llvm(),
            )
        };

        if target_machine.is_null() {
            return None;
        }

        Some(TargetMachine::new(target_machine))
    }

    pub fn get_target_description(&self) -> &str {
        unsafe {
            CStr::from_ptr(LLVMGetTargetDescription(self.target))
                .to_str()
                .expect("Conversion of target description string from cstring failed unexpectedly")
        }
    }
}

/// Contains data holding target-specific information like the triple and methods for getting this information.
pub struct TargetMachine {
    machine: LLVMTargetMachineRef,
}

impl TargetMachine {
    fn new(target_machine: LLVMTargetMachineRef) -> Self {
        assert!(!target_machine.is_null());

        TargetMachine {
            machine: target_machine,
        }
    }

    pub fn get_target_data(&self) -> TargetData {
        let data_layout = unsafe { LLVMCreateTargetDataLayout(self.machine) };

        TargetData::new(data_layout)
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeTargetMachine(self.machine) }
    }
}

///
pub struct TargetData {
    pub(crate) data: LLVMTargetDataRef,
}

impl TargetData {
    fn new(target_data: LLVMTargetDataRef) -> TargetData {
        assert!(!target_data.is_null());

        TargetData { data: target_data }
    }

    // Fails if datalayout spec is wrong
    pub fn create(description: &str) -> Option<Self> {
        let c_string = CString::new(description)
            .expect("Conversion of target description to cstring failed unexpectedly");

        let target_data = unsafe { LLVMCreateTargetData(c_string.as_ptr()) };

        if target_data.is_null() {
            return None;
        }

        Some(TargetData::new(target_data))
    }

    pub fn machine_int_type(&self, address_space: Option<&AddressSpace>) -> IntType {
        let ty = match address_space {
            Some(address_space) => unsafe { LLVMIntPtrTypeForAS(self.data, *address_space as _) },
            None => unsafe { LLVMIntPtrType(self.data) },
        };

        IntType::new(ty)
    }
}

impl Drop for TargetData {
    fn drop(&mut self) {
        unsafe { LLVMDisposeTargetData(self.data) }
    }
}
