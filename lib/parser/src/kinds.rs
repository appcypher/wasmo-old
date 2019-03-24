#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SectionKind {
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
}

impl From<u8> for SectionKind {
    fn from(value: u8) -> SectionKind {
        match value {
            0x1 => SectionKind::Type,
            0x2 => SectionKind::Import,
            0x3 => SectionKind::Function,
            0x4 => SectionKind::Table,
            0x5 => SectionKind::Memory,
            0x6 => SectionKind::Global,
            0x7 => SectionKind::Export,
            0x8 => SectionKind::Start,
            0x9 => SectionKind::Element,
            0xA => SectionKind::Code,
            0xB => SectionKind::Data,
            _ => unreachable!(),
        }
    }
}

use crate::ir;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    //------------ PREAMBLE -------------//

    // Preamble
    IncompletePreamble,
    MalformedMagicNumber,
    InvalidMagicNumber,
    MalformedVersionNumber,
    InvalidVersionNumber,

    //------------ SECTIONS -------------//

    // Sections
    IncompleteSection,
    SectionAlreadyDefined,
    UnsupportedSection,
    MalformedSectionId,
    SectionPayloadDoesNotMatchPayloadLength,
    // Custom Section
    IncompleteCustomSection,
    MalformedPayloadLengthInCustomSection,
    MalformedNameLengthInCustomSection,
    // Type Section
    IncompleteTypeSection,
    MalformedPayloadLengthInTypeSection,
    MalformedEntryCountInTypeSection,
    EntriesDoNotMatchEntryCountInTypeSection,
    MalformedTypeInTypeSection,
    UnsupportedTypeInTypeSection,
    // Import Section
    IncompleteImportSection,
    MalformedPayloadLengthInImportSection,
    MalformedEntryCountInImportSection,
    MalformedEntryInImportSection,
    // Function Section
    IncompleteFunctionSection,
    MalformedPayloadLengthInFunctionSection,
    MalformedEntryCountInFunctionSection,
    MalformedEntryInFunctionSection,
    // Table Section
    IncompleteTableSection,
    MalformedPayloadLengthInTableSection,
    MalformedEntryCountInTableSection,
    MalformedEntryInTableSection,
    // Memory Section
    IncompleteMemorySection,
    MalformedPayloadLengthInMemorySection,
    MalformedEntryCountInMemorySection,
    MalformedEntryInMemorySection,
    // Global Section
    IncompleteGlobalSection,
    MalformedPayloadLengthInGlobalSection,
    MalformedEntryCountInGlobalSection,
    MalformedEntryInGlobalSection,
    // Export Section
    IncompleteExportSection,
    MalformedPayloadLengthInExportSection,
    MalformedEntryCountInExportSection,
    MalformedEntryInExportSection,
    // Start Section
    IncompleteStartSection,
    MalformedPayloadLengthInStartSection,
    MalformedFunctionIndexInStartSection,
    // Element Section
    IncompleteElementSection,
    MalformedPayloadLengthInElementSection,
    MalformedEntryCountInElementSection,
    MalformedEntryInElementSection,
    // Code Section
    IncompleteCodeSection,
    MalformedPayloadLengthInCodeSection,
    MalformedBodyCountInCodeSection,
    MalformedBodyInCodeSection,
    // Data Section
    IncompleteDataSection,
    MalformedPayloadLengthInDataSection,
    MalformedEntryCountInDataSection,
    MalformedEntryInDataSection,

    //------------ ENTRIES -------------//

    // Import Entry
    IncompleteImportEntry,
    MalformedModuleNameLengthInImportEntry,
    ModuleStringDoesNotMatchModuleLengthInImportEntry,
    MalformedFieldNameLengthInImportEntry,
    FieldStringDoesNotMatchFieldLengthInImportEntry,
    MalformedImportTypeInImportEntry,
    InvalidImportTypeInImportEntry,
    // Function Import
    IncompleteFunctionImport,
    MalformedTypeIndexInFunctionImport,
    InvalidTypeIndexInFunctionImport,
    // Table Import
    IncompleteTableImport,
    MalformedElementTypeInTableImport,
    MalformedFlagsInTableImport,
    MalformedMinimumInTableImport,
    MalformedMaximumInTableImport,
    MalformedLimitsInTableImport,
    // Memory Import
    IncompleteMemoryImport,
    MalformedFlagsInMemoryImport,
    MalformedMinimumInMemoryImport,
    MalformedMaximumInMemoryImport,
    MalformedLimitsInMemoryImport,
    // Global Import
    IncompleteGlobalImport,
    MalformedContentTypeInGlobalImport,
    MalformedMutabilityInGlobalImport,
    // Function Type
    IncompleteFunctionType,
    MalformedParamCountInFunctionType,
    ParamsDoesNotMatchParamCountInFunctionType,
    MalformedParamTypeInFunctionType,
    MalformedReturnCountInFunctionType,
    MalformedReturnTypeInFunctionType,
    ReturnTypeDoesNotMatchReturnCountInFunctionType,
    // Table Entry
    IncompleteTableEntry,
    MalformedElementTypeInTableEntry,
    InvalidElementTypeInTableEntry,
    MalformedLimitsInTableEntry,
    MalformedMaximumInTableEntry,
    MalformedMinimumInTableEntry,
    MalformedFlagsInTableEntry,
    // Memory Entry
    IncompleteMemoryEntry,
    MalformedLimitsInMemoryEntry,
    MalformedMaximumInMemoryEntry,
    MalformedMinimumInMemoryEntry,
    MalformedFlagsInMemoryEntry,
    // Global Entry
    IncompleteGlobalEntry,
    MalformedContentTypeInGlobalEntry,
    MalformedMutabilityInGlobalEntry,
    // Export Entry
    IncompleteExportEntry,
    MalformedNameLengthInExportEntry,
    MalformedExportKindInExportEntry,
    InvalidExportTypeInExportEntry,
    MalformedExportIndexInExportEntry,
    // Element Entry
    IncompleteElementEntry,
    MalformedInstructionInElementEntry,
    MalformedTableIndexInElementEntry,
    MalformedFunctionCountInElementEntry,
    MalformedFunctionIndexInElementEntry,
    // Function Body
    IncompleteFunctionBody,
    MalformedBodySizeInFunctionBody,
    BodySizeDoesNotMatchContentOfFunctionBody,
    // Local Entry
    IncompleteLocalEntry,
    MalformedCountInLocalEntry,
    MalformedLocalTypeInLocalEntry,
    // Instructions
    IncompleteExpression,
    MalformedOpcodeInExpression,
    MalformedEndByteInExpression,
    // Data Entry
    IncompleteDataEntry,
    MalformedTableIndexInDataEntry,
    MalformedInstructionInDataEntry,
    MalformedByteCountInDataEntry,

    //------------ UTILS -------------//

    // Limits
    IncompleteLimits,
    MalformedFlagsInLimits,
    MalformedMinimumInLimits,
    MalformedMaximumInLimits,
    // Storage
    BufferEndReached,
    MalformedVaruint1,
    MalformedVaruint7,
    MalformedVarint7,
    MalformedVaruint32,
    MalformedVarint32,
    MalformedVarint64,
    // Types
    InvalidValueType,
    // ExternalKind
    InvalidImportType,

    //------------ VARIABLES --------//

    LocalDoesNotExist,
    GlobalDoesNotExist,

    //------------ OPERATORS --------//

    UnsupportedOperator,
    // Types
    MismatchedOperandTypes {
        expected: Vec<ir::ValueType>,
        found: Vec<ir::ValueType>,
    },
    MismatchedFunctionSignature {
        expected: ir::FuncSignature,
        found: Vec<ir::ValueType>,
    },
    MismatchedFunctionReturnSignature {
        expected: ir::FuncSignature,
        return_type_found: Vec<ir::ValueType>,
    },
    MismatchedBlockResultSignature {
        expected: Vec<ir::BlockType>,
        found: Vec<ir::ValueType>,
    },
    // Memory
    IncompleteMemoryOperator,
    MalformedAlignmentInMemoryOperator,
    MalformedOffsetInMemoryOperator,
}
