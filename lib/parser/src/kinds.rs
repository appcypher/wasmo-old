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
    // Import Entry
    IncompleteImportEntry,
    MalformedModuleLengthInImportEntry,
    ModuleStringDoesNotMatchModuleLengthInImportEntry,
    MalformedFieldLengthInImportEntry,
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
    MalformedInitialInTableImport,
    MalformedMaximumInTableImport,
    MalformedResizableLimitInTableImport,
    // Memory Import
    IncompleteMemoryImport,
    MalformedFlagsInMemoryImport,
    MalformedInitialInMemoryImport,
    MalformedMaximumInMemoryImport,
    MalformedResizableLimitInMemoryImport,
    // Global Import
    IncompleteGlobalImport,
    MalformedContentTypeInGlobalImport,
    MalformedMutabilityInGlobalImport,
    // Function Body
    MalformedBodySizeInFunctionBody,
    MalformedEndByteInFunctionBody,
    // Local Entry
    MalformedCountInLocalEntry,
    MalformedTypeInLocalEntry,
    // Resizable Limits
    IncompleteResizableLimits,
    MalformedFlagsInResizableLimits,
    MalformedInitialInResizableLimits,
    MalformedMaximumInResizableLimits,
    // Function Type
    IncompleteFunctionType,
    MalformedParamCountInFunctionType,
    ParamsDoesNotMatchParamCountInFunctionType,
    MalformedParamTypeInFunctionType,
    MalformedReturnCountInFunctionType,
    MalformedReturnTypeInFunctionType,
    ReturnTypeDoesNotMatchReturnCountInFunctionType,
}
