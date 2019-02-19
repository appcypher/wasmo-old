#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    BufferEndReached,
    // Storage
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
    // Preamble
    IncompletePreamble,
    MalformedMagicNumber,
    InvalidMagicNumber,
    MalformedVersionNumber,
    InvalidVersionNumber,
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
    // Code Section
    IncompleteCodeSection,
    MalformedPayloadLengthInCodeSection,
    MalformedBodyCountInCodeSection,
    MalformedBodyInCodeSection,
    // Table Section
    IncompleteTableSection,
    MalformedPayloadLengthInTableSection,
    MalformedEntryCountInTableSection,
    MalformedEntryInTableSection,
    // Export Section
    IncompleteExportSection,
    MalformedPayloadLengthInExportSection,
    MalformedEntryCountInExportSection,
    // Start Section
    IncompleteStartSection,
    MalformedPayloadLengthInStartSection,
    MalformedEntryCountInStartSection,
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
    // Function Body
    IncompleteFunctionBody,
    MalformedBodySizeInFunctionBody,
    MalformedEndByteInFunctionBody,
    // Local Entry
    MalformedCountInLocalEntry,
    MalformedTypeInLocalEntry,
    // Limits
    IncompleteLimits,
    MalformedFlagsInLimits,
    MalformedMinimumInLimits,
    MalformedMaximumInLimits,
    // Function Type
    IncompleteFunctionType,
    MalformedParamCountInFunctionType,
    ParamsDoesNotMatchParamCountInFunctionType,
    MalformedParamTypeInFunctionType,
    MalformedReturnCountInFunctionType,
    MalformedReturnTypeInFunctionType,
    ReturnTypeDoesNotMatchReturnCountInFunctionType,
    // Table Type
    IncompleteTableType,
    InvalidElementTypeInTableType,
    MalformedLimitsInTableType,
    MalformedMaximumInTableType,
    MalformedMinimumInTableType,
    MalformedFlagsInTableType,
    // Memory Type
    IncompleteMemoryType,
    MalformedLimitsInMemoryType,
    MalformedMaximumInMemoryType,
    MalformedMinimumInMemoryType,
    MalformedFlagsInMemoryType,
    // Global
    IncompleteGlobal,
    MalformedEndByteInGlobal,
    MalformedContentTypeInGlobal,
    MalformedBodySizeInGlobal,
    MalformedMutabilityInGlobal,
    // Export Entry
    IncompleteExportEntry,
    // MalformedPayloadLengthInExportSection,
    // MalformedEntryCountInExportSection,
    MalformedNameLengthInExportEntry,
    MalformedImportTypeInExportEntry,
    InvalidExportTypeInExportEntry,
}

#[derive(Debug)]
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
