#[macro_use]
use llvm_sys::LLVMLinkage;
use llvm_sys::target_machine::LLVMCodeGenFileType;

#[derive(Debug, PartialEq, Eq)]
pub enum OptimizationLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelocationModel {
    PIC,
    Static,
    DynamicNoPIC,
    ROPI,
    RWPI,
    ROPI_RWPI,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CodeModel {
    Default,
    JITDefault,
    Small,
    Kernel,
    Medium,
    Large,
}

enum_rename! {
    LLVMCodeGenFileType >> CodeGenFileType {
        LLVMAssemblyFile >> AssemblyFile,
        LLVMObjectFile >> ObjectFile
    }
}

enum_rename! {
    /// This enum defines how to link a global variable or function in a module. The variant documenation is
    /// mostly taken straight from LLVM's own documentation except for some minor clarification.
    ///
    /// It is illegal for a function declaration to have any linkage type other than external or extern_weak.
    ///
    /// All Global Variables, Functions and Aliases can have one of the following DLL storage class: `DLLImport`
    /// & `DLLExport`.
    LLVMLinkage >> Linkage {
        /// `Appending` linkage may only be applied to global variables of pointer to array type. When two global
        /// variables with appending linkage are linked together, the two global arrays are appended together.
        /// This is the LLVM, typesafe, equivalent of having the system linker append together "sections" with
        /// identical names when .o files are linked. Unfortunately this doesn't correspond to any feature in .o
        /// files, so it can only be used for variables like llvm.global_ctors which llvm interprets specially.
        LLVMAppendingLinkage >> Appending,
        /// Globals with `AvailableExternally` linkage are never emitted into the object file corresponding to
        /// the LLVM module. From the linker's perspective, an `AvailableExternally` global is equivalent to an
        /// external declaration. They exist to allow inlining and other optimizations to take place given
        /// knowledge of the definition of the global, which is known to be somewhere outside the module. Globals
        /// with `AvailableExternally` linkage are allowed to be discarded at will, and allow inlining and other
        /// optimizations. This linkage type is only allowed on definitions, not declarations.
        LLVMAvailableExternallyLinkage >> AvailableExternally,
        /// `Common` linkage is most similar to "weak" linkage, but they are used for tentative definitions
        /// in C, such as "int X;" at global scope. Symbols with Common linkage are merged in the same way as
        /// weak symbols, and they may not be deleted if unreferenced. `Common` symbols may not have an explicit
        /// section, must have a zero initializer, and may not be marked 'constant'. Functions and aliases may
        /// not have `Common` linkage.
        LLVMCommonLinkage >> Common,
        /// `DLLExport` causes the compiler to provide a global pointer to a pointer in a DLL, so that it can be
        /// referenced with the dllimport attribute. On Microsoft Windows targets, the pointer name is formed by
        /// combining __imp_ and the function or variable name. Since this storage class exists for defining a dll
        /// interface, the compiler, assembler and linker know it is externally referenced and must refrain from
        /// deleting the symbol.
        LLVMDLLExportLinkage >> DLLExport,
        /// `DLLImport` causes the compiler to reference a function or variable via a global pointer to a pointer
        /// that is set up by the DLL exporting the symbol. On Microsoft Windows targets, the pointer name is
        /// formed by combining __imp_ and the function or variable name.
        LLVMDLLImportLinkage >> DLLImport,
        /// If none of the other identifiers are used, the global is externally visible, meaning that it
        /// participates in linkage and can be used to resolve external symbol references.
        LLVMExternalLinkage >> External,
        /// The semantics of this linkage follow the ELF object file model: the symbol is weak until linked,
        /// if not linked, the symbol becomes null instead of being an undefined reference.
        LLVMExternalWeakLinkage >> ExternalWeak,
        /// FIXME: Unknown linkage type
        LLVMGhostLinkage >> Ghost,
        /// Similar to private, but the value shows as a local symbol (STB_LOCAL in the case of ELF) in the object
        /// file. This corresponds to the notion of the 'static' keyword in C.
        LLVMInternalLinkage >> Internal,
        /// FIXME: Unknown linkage type
        LLVMLinkerPrivateLinkage >> LinkerPrivate,
        /// FIXME: Unknown linkage type
        LLVMLinkerPrivateWeakLinkage >> LinkerPrivateWeak,
        /// Globals with `LinkOnceAny` linkage are merged with other globals of the same name when linkage occurs.
        /// This can be used to implement some forms of inline functions, templates, or other code which must be
        /// generated in each translation unit that uses it, but where the body may be overridden with a more
        /// definitive definition later. Unreferenced `LinkOnceAny` globals are allowed to be discarded. Note that
        /// `LinkOnceAny` linkage does not actually allow the optimizer to inline the body of this function into
        /// callers because it doesn’t know if this definition of the function is the definitive definition within
        /// the program or whether it will be overridden by a stronger definition. To enable inlining and other
        /// optimizations, use `LinkOnceODR` linkage.
        LLVMLinkOnceAnyLinkage >> LinkOnceAny,
        /// FIXME: Unknown linkage type
        LLVMLinkOnceODRAutoHideLinkage >> LinkOnceODRAutoHide,
        /// Some languages allow differing globals to be merged, such as two functions with different semantics.
        /// Other languages, such as C++, ensure that only equivalent globals are ever merged (the "one definition
        /// rule" — "ODR"). Such languages can use the `LinkOnceODR` and `WeakODR` linkage types to indicate that
        /// the global will only be merged with equivalent globals. These linkage types are otherwise the same
        /// as their non-odr versions.
        LLVMLinkOnceODRLinkage >> LinkOnceODR,
        /// Global values with `Private` linkage are only directly accessible by objects in the current module.
        /// In particular, linking code into a module with a private global value may cause the private to be
        /// renamed as necessary to avoid collisions. Because the symbol is private to the module, all references
        /// can be updated. This doesn’t show up in any symbol table in the object file.
        LLVMPrivateLinkage >> Private,
        /// `WeakAny` linkage has the same merging semantics as linkonce linkage, except that unreferenced globals
        /// with weak linkage may not be discarded. This is used for globals that are declared WeakAny in C source code.
        LLVMWeakAnyLinkage >> WeakAny,
        /// Some languages allow differing globals to be merged, such as two functions with different semantics.
        /// Other languages, such as C++, ensure that only equivalent globals are ever merged (the "one definition
        /// rule" — "ODR"). Such languages can use the `LinkOnceODR` and `WeakODR` linkage types to indicate that
        /// the global will only be merged with equivalent globals. These linkage types are otherwise the same
        /// as their non-odr versions.
        LLVMWeakODRLinkage >> WeakODR
    }
}
