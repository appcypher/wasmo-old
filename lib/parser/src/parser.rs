use crate::{
    errors::ParserError,
    ir::{
        Data, Element, Export, ExportDesc, Function, Global, Import, ImportDesc, Local, Memory,
        Operator, Section, Table, Type,
    },
    kinds::{ErrorKind, SectionKind},
    macros,
    validation::validate_section_exists,
};
use std::str;
use wasmlite_utils::{debug, verbose};

// TODO
//  - Improve error reporting.

pub type ParserResult<T> = Result<T, ParserError>;

/// A WebAssembly module parser.
///
/// Notes
/// - Just like with body_size in function body. payload_len should be used to determine if section content stay within payload range
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    code: &'a [u8],                        // The wasm binary to parse
    cursor: usize, // Used to track the current byte position as the parser advances.
    pub(super) sections_consumed: Vec<u8>, // Holds the section ids that have been consumed. Section types cannot occur more than once.
}

/// Contains the implementation of parser
impl<'a> Parser<'a> {
    /// Creates new parser
    pub fn new(code: &'a [u8]) -> Self {
        Parser {
            code,
            cursor: 0, // cursor starts at first byte
            sections_consumed: vec![],
        }
    }

    /// Sets the parser's cursor.
    pub(super) fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    /// Pushes an id into the parser's sections_consumed.
    pub(super) fn push_section_id(&mut self, section_id: &u8) {
        self.sections_consumed.push(*section_id);
    }

    /// TODO: TEST
    /// Generates an IR rpresenting a parsed wasm module.
    pub fn module(&mut self) -> ParserResult<()> {
        verbose!("-> module! <-");

        // Consume preamble.
        self.module_preamble()?;

        // TODO: Module can stop here.
        let sections = self.sections().unwrap();

        debug!("(module::sections = {:#?})", sections);

        Ok(())
    }

    /// TODO: TEST
    /// Checks if the following bytes are expected
    /// wasm preamble bytes.
    pub fn module_preamble(&mut self) -> ParserResult<()> {
        verbose!("-> module_preamble! <-");
        let cursor = self.cursor;

        // Consume magic number.
        let magic_no = match self.uint32() {
            Ok(value) => {
                // Magic number must be `\0asm`
                if value != 0x6d73_6100 {
                    return Err(ParserError {
                        kind: ErrorKind::InvalidMagicNumber,
                        cursor,
                    });
                }
                value
            }
            Err(error) => {
                if error == ErrorKind::BufferEndReached {
                    return Err(ParserError {
                        kind: ErrorKind::IncompletePreamble,
                        cursor,
                    });
                } else {
                    return Err(ParserError {
                        kind: ErrorKind::InvalidVersionNumber,
                        cursor,
                    });
                }
            }
        };

        verbose!("(module_preamble::magic_no = 0x{:08x})", magic_no);

        // Consume version number.
        let version_no = match self.uint32() {
            Ok(value) => {
                // Only version 0x01 supported for now.
                if value != 0x1 {
                    return Err(ParserError {
                        kind: ErrorKind::MalformedVersionNumber,
                        cursor,
                    });
                }
                value
            }
            Err(error) => {
                if error == ErrorKind::BufferEndReached {
                    return Err(ParserError {
                        kind: ErrorKind::IncompletePreamble,
                        cursor,
                    });
                } else {
                    return Err(ParserError {
                        kind: ErrorKind::MalformedVersionNumber,
                        cursor,
                    });
                }
            }
        };

        verbose!("(module_preamble::version_no = 0x{:08x})", version_no);

        Ok(())
    }

    /// TODO: TEST
    pub fn sections(&mut self) -> ParserResult<Vec<Section>> {
        verbose!("-> module_sections! <-");

        let mut sections = vec![];

        // Iterate and Consume the section
        loop {
            let cursor = self.cursor;
            // Get section id.
            let section_id = match self.varuint7() {
                Ok(value) => value,
                Err(error) => {
                    // Break loop when there is no more section to consume.
                    if error == ErrorKind::BufferEndReached {
                        break;
                    } else {
                        return Err(ParserError {
                            kind: ErrorKind::MalformedSectionId,
                            cursor,
                        });
                    }
                }
            };

            // Validate that section id exists
            validate_section_exists(self, section_id, cursor)?;

            // Save id in list of sections consumed if not custpom section
            if section_id != 0 {
                self.push_section_id(&section_id);
            }

            verbose!(
                "(module_sections::section code = {:?})",
                SectionKind::from(section_id)
            );

            // Consume appropriate section based on section id.
            sections.push(self.section(section_id)?);
        }

        Ok(sections)
    }

    /// Gets the next section id and payload.
    /// Needed by reader
    pub fn section_id(&mut self) -> ParserResult<u8> {
        verbose!("-> module_section_id! <-");
        let cursor = self.cursor;

        // Get section id.
        let section_id = get_value!(
            self.varuint7(),
            cursor,
            IncompleteSection,
            MalformedSectionId
        );

        // Validate that section id exists.
        validate_section_exists(self, section_id, cursor)?;

        Ok(section_id)
    }

    /// Gets the next module section
    pub fn section(&mut self, section_id: u8) -> ParserResult<Section> {
        verbose!("-> section! <-");
        let cursor = self.cursor;

        Ok(match section_id {
            0x00 => self.custom_section()?,
            0x01 => self.tysection()?,
            0x02 => self.import_section()?,
            0x03 => self.function_section()?,
            0x04 => self.table_section()?,
            0x05 => self.memory_section()?,
            0x06 => self.global_section()?,
            0x07 => self.export_section()?,
            0x08 => self.start_section()?,
            0x09 => self.element_section()?,
            0x0A => self.code_section()?,
            0x0B => self.data_section()?,
            _ => {
                return Err(ParserError {
                    kind: ErrorKind::UnsupportedSection,
                    cursor,
                });
            }
        })
    }

    /******** SECTIONS ********/

    /// TODO: TEST
    /// TODO: Name section and linking section.
    pub fn custom_section(&mut self) -> ParserResult<Section> {
        verbose!("-> custom_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCustomSection,
            MalformedPayloadLengthInCustomSection
        );

        // Get name length of custom section.
        let name_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCustomSection,
            MalformedEntryCountInTypeSection
        );

        {
            // TODO: Validate UTF-8
            // Skip name bytes for now.
            let _name = match self.eat_bytes(name_len as _) {
                Some(value) => value,
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteCustomSection,
                        cursor,
                    });
                }
            };
        }

        // Skip payload bytes as well for now.
        let _payload_data = match self.eat_bytes(payload_len as _) {
            Some(value) => value,
            None => {
                return Err(ParserError {
                    kind: ErrorKind::IncompleteCustomSection,
                    cursor,
                });
            }
        };

        Ok(Section::Custom)
    }

    /// TODO: TEST
    pub fn tysection(&mut self) -> ParserResult<Section> {
        verbose!("-> tysection! <-");
        let cursor = self.cursor;
        let mut func_types = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedPayloadLengthInTypeSection
        );

        verbose!("(tysection::payload_len = 0x{:x})", payload_len);

        // Get the count of type entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedEntryCountInTypeSection
        );

        verbose!("(tysection::entry_count = 0x{:x})", entry_count);

        // Consume the type entries.
        for _ in 0..entry_count {
            let tyid = get_value!(
                self.varint7(),
                cursor,
                EntriesDoNotMatchEntryCountInTypeSection,
                MalformedTypeInTypeSection
            );

            verbose!("(tysection::tyid = {:?})", tyid);

            // Type must be a func type.
            let func_type = match tyid {
                -0x20 => self.func_type()?,
                _ => {
                    return Err(ParserError {
                        kind: ErrorKind::UnsupportedTypeInTypeSection,
                        cursor,
                    });
                }
            };

            func_types.push(func_type);
        }

        // TODO
        Ok(Section::Type(func_types))
    }

    /// TODO: TEST
    pub fn import_section(&mut self) -> ParserResult<Section> {
        verbose!("-> import_section! <-");
        let cursor = self.cursor;
        let mut imports = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportSection,
            MalformedPayloadLengthInImportSection
        );

        verbose!("(import_section::payload_len = 0x{:x})", payload_len);

        // Get the count of import entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportSection,
            MalformedEntryCountInImportSection
        );

        verbose!("(import_section::entry_count = 0x{:x})", entry_count);

        // Consume the import entries.
        for _ in 0..entry_count {
            imports.push(self.import_entry()?);
        }
        verbose!("(import_section::import_entries = {:?})", imports);

        // TODO
        Ok(Section::Import(imports))
    }

    /// TODO: TEST
    pub fn function_section(&mut self) -> ParserResult<Section> {
        verbose!("-> function_section! <-");
        let cursor = self.cursor;
        let mut tyindices = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedPayloadLengthInFunctionSection
        );

        verbose!("(function_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function entries,
        let function_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedEntryCountInFunctionSection
        );

        verbose!(
            "(function_section::function_count = 0x{:x})",
            function_count
        );

        // Consume the function index entries.
        for _ in 0..function_count {
            let tyindex = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            tyindices.push(tyindex);
        }
        verbose!("(function_section::tyindices = {:?})", tyindices);

        // TODO
        Ok(Section::Function(tyindices))
    }

    /// TODO: TEST
    pub fn table_section(&mut self) -> ParserResult<Section> {
        verbose!("-> table_section! <-");
        let cursor = self.cursor;
        let mut tables = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedPayloadLengthInTableSection
        );

        verbose!("(table_section::payload_len = 0x{:x})", payload_len);

        // Get the count of table entries,
        let table_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedEntryCountInTableSection
        );

        verbose!("(table_section::table_count = 0x{:x})", table_count);

        // Consume the function entries.
        for _ in 0..table_count {
            tables.push(self.table_entry()?);
        }

        verbose!("(table_section::table_entries = {:?})", tables);

        // TODO
        Ok(Section::Table(tables))
    }

    /// TODO: TEST
    pub fn memory_section(&mut self) -> ParserResult<Section> {
        verbose!("-> memory_section! <-");
        let cursor = self.cursor;
        let mut memories = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemorySection,
            MalformedPayloadLengthInMemorySection
        );

        verbose!("(memory_section::payload_len = 0x{:x})", payload_len);

        // Get the count of memory entries.
        let memory_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemorySection,
            MalformedEntryCountInMemorySection
        );

        verbose!("(memory_section::memory_count = 0x{:x})", memory_count);

        // Consume the entries.
        for _ in 0..memory_count {
            memories.push(self.memory_entry()?);
        }
        verbose!("(memory_section::memory_entries = {:?})", memories);

        Ok(Section::Memory(memories))
    }

    /// TODO: TEST
    pub fn global_section(&mut self) -> ParserResult<Section> {
        verbose!("-> global_section! <-");
        let cursor = self.cursor;
        let mut globals = vec![];

        // The length of the global section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteGlobalSection,
            MalformedPayloadLengthInGlobalSection
        );

        verbose!("(global_section::payload_len = 0x{:x})", payload_len);

        // Get the count of global entries,
        let global_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteGlobalSection,
            MalformedEntryCountInGlobalSection
        );

        verbose!("(global_section::global_count = 0x{:x})", global_count);

        // Consume the global entries.
        for _ in 0..global_count {
            globals.push(self.global_entry()?);
        }
        verbose!("(global_section::global_entries = {:?})", globals);

        Ok(Section::Global(globals))
    }

    /// TODO: TEST
    pub fn export_section(&mut self) -> ParserResult<Section> {
        verbose!("-> export_section! <-");
        let cursor = self.cursor;
        let mut exports = vec![];

        // The length of the export section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportSection,
            MalformedPayloadLengthInExportSection
        );

        verbose!("(export_section::payload_len = 0x{:x})", payload_len);

        // Get the count of export entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportSection,
            MalformedEntryCountInExportSection
        );

        verbose!("(export_section::entry_count = 0x{:x})", entry_count);

        // Consume the export entries.
        for _ in 0..entry_count {
            exports.push(self.export_entry()?);
        }
        verbose!("(export_section::export_entries = {:?})", exports);

        // TODO
        Ok(Section::Export(exports))
    }

    /// TODO: TEST
    pub fn start_section(&mut self) -> ParserResult<Section> {
        verbose!("-> start_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteStartSection,
            MalformedPayloadLengthInStartSection
        );

        verbose!("(start_section::payload_len = 0x{:x})", payload_len);

        // Get the indes of the start function,
        let function_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteStartSection,
            MalformedFunctionIndexInStartSection
        );

        // TODO
        Ok(Section::Start(function_index))
    }

    /// TODO: TEST
    pub fn element_section(&mut self) -> ParserResult<Section> {
        verbose!("-> element_section! <-");
        let cursor = self.cursor;
        let mut elements = vec![];

        // The length of the element section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteElementSection,
            MalformedPayloadLengthInElementSection
        );

        verbose!("(element_section::payload_len = 0x{:x})", payload_len);

        // Get the count of element entries,
        let element_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteElementSection,
            MalformedEntryCountInElementSection
        );

        verbose!("(element_section::element_count = 0x{:x})", element_count);

        // Consume the element entries.
        for _ in 0..element_count {
            elements.push(self.element_entry()?);
        }
        verbose!("(element_section::element_entries = {:?})", elements);

        // TODO
        Ok(Section::Element(elements))
    }

    /// TODO: TEST
    pub fn code_section(&mut self) -> ParserResult<Section> {
        verbose!("-> code_section! <-");
        let cursor = self.cursor;
        let mut function_bodies = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCodeSection,
            MalformedPayloadLengthInCodeSection
        );

        verbose!("(code_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function bodies.
        let body_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCodeSection,
            MalformedBodyCountInCodeSection
        );

        verbose!("(code_section::entry_count = 0x{:x})", body_count);

        // Consume the function bodies.
        for _ in 0..body_count {
            function_bodies.push(self.function_body()?);
        }
        verbose!("(code_section::function_bodies = {:?})", function_bodies);

        // TODO
        Ok(Section::Code(function_bodies))
    }

    /// TODO: TEST
    pub fn data_section(&mut self) -> ParserResult<Section> {
        verbose!("-> data_section! <-");
        let cursor = self.cursor;
        let mut data = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteDataSection,
            MalformedPayloadLengthInDataSection
        );

        verbose!("(data_section::payload_len = 0x{:x})", payload_len);

        // Get the count of data entries,
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteDataSection,
            MalformedEntryCountInDataSection
        );

        verbose!("(data_section::function_count = 0x{:x})", entry_count);

        // Consume the function entries.
        for _ in 0..entry_count {
            data.push(self.data_entry()?);
        }
        verbose!("(data_section::data_entries = {:?})", data);

        // TODO
        Ok(Section::Data(data))
    }

    /******** IMPORTS ********/

    /// TODO: TEST
    pub fn import_entry(&mut self) -> ParserResult<Import> {
        verbose!("-> import_entry! <-");
        let cursor = self.cursor;
        let mut module_name = String::new();
        let mut field_name = String::new();

        // Get module name length
        let module_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportEntry,
            MalformedModuleNameLengthInImportEntry
        );

        verbose!("(import_entry::module_len = 0x{:x})", module_len);

        {
            // TODO: Validate UTF-8
            module_name = match self.eat_bytes(module_len as _) {
                Some(value) => str::from_utf8(value).unwrap().into(),
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteImportEntry,
                        cursor,
                    });
                }
            };

            verbose!("import_entry::_module_str = {:?}", module_name);
        }

        // Get field name length
        let field_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportEntry,
            MalformedFieldNameLengthInImportEntry
        );

        verbose!("(import_entry::field_len = 0x{:x})", field_len);

        {
            // TODO: Validate UTF-8
            field_name = match self.eat_bytes(field_len as _) {
                Some(value) => str::from_utf8(value).unwrap().into(),
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteImportEntry,
                        cursor,
                    });
                }
            };

            verbose!("(import_entry::_field_str = {:?})", field_name);
        }

        let external_kind = get_value!(
            self.external_kind(),
            cursor,
            IncompleteImportEntry,
            MalformedImportTypeInImportEntry
        );

        let desc = match external_kind {
            // Function import
            0x00 => self.function_import()?,
            // Table import
            0x01 => self.table_import()?,
            // Memory import
            0x02 => self.memory_import()?,
            // Global import
            0x03 => self.global_import()?,
            _ => {
                return Err(ParserError {
                    kind: ErrorKind::InvalidImportTypeInImportEntry,
                    cursor,
                });
            }
        };

        Ok(Import {
            module_name,
            field_name,
            desc,
        })
    }

    /// TODO: TEST
    pub fn function_import(&mut self) -> ParserResult<ImportDesc> {
        verbose!("-> function_import! <-");
        let cursor = self.cursor;
        let tyindex = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionImport,
            MalformedTypeIndexInFunctionImport
        );

        verbose!("(function_import::tyindex = {:?})", tyindex);

        Ok(ImportDesc::Function { tyindex })
    }

    /// TODO: TEST
    pub fn table_import(&mut self) -> ParserResult<ImportDesc> {
        verbose!("-> table_import! <-");
        let cursor = self.cursor;
        let element_type = Type::from(match self.varint7() {
            Ok(value) => {
                // Must be anyfunc
                if value != -0x10 {
                    return Err(ParserError {
                        kind: ErrorKind::MalformedElementTypeInTableImport,
                        cursor,
                    });
                }
                value
            }
            Err(error) => {
                if error == ErrorKind::BufferEndReached {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteTableImport,
                        cursor,
                    });
                } else {
                    return Err(ParserError {
                        kind: ErrorKind::MalformedElementTypeInTableImport,
                        cursor,
                    });
                }
            }
        });

        verbose!("(table_import::element_type = {:?})", element_type);

        // Get limits
        let (minimum, maximum) = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteTableImport,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInTableImport,
                    ErrorKind::MalformedMinimumInLimits => ErrorKind::MalformedMinimumInTableImport,
                    ErrorKind::MalformedMaximumInLimits => ErrorKind::MalformedMaximumInTableImport,
                    _ => ErrorKind::MalformedLimitsInTableImport,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        verbose!("(table_import::minimum = {:?})", minimum);

        verbose!("(table_import::maximum = {:?})", maximum);

        Ok(ImportDesc::Table(Table {
            element_type,
            minimum,
            maximum,
        }))
    }

    /// TODO: TEST
    pub fn memory_import(&mut self) -> ParserResult<ImportDesc> {
        verbose!("-> memory_import! <-");
        let cursor = self.cursor;

        // Get limits
        let (minimum, maximum) = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteMemoryImport,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInMemoryImport,
                    ErrorKind::MalformedMinimumInLimits => {
                        ErrorKind::MalformedMinimumInMemoryImport
                    }
                    ErrorKind::MalformedMaximumInLimits => {
                        ErrorKind::MalformedMaximumInMemoryImport
                    }
                    _ => ErrorKind::MalformedLimitsInMemoryImport,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        verbose!("(memory_import::minimum = {:?})", minimum);

        verbose!("(memory_import::maximum = {:?})", maximum);

        Ok(ImportDesc::Memory(Memory { minimum, maximum }))
    }

    /// TODO: TEST
    pub fn global_import(&mut self) -> ParserResult<ImportDesc> {
        verbose!("-> global_import! <-");
        let cursor = self.cursor;

        let content_type = Type::from(get_value!(
            self.value_type(),
            cursor,
            IncompleteGlobalImport,
            MalformedContentTypeInGlobalImport
        ));

        verbose!("(global_import::content_type = {:?})", content_type);
        let mutability = get_value!(
            self.varuint1(),
            cursor,
            IncompleteGlobalImport,
            MalformedMutabilityInGlobalImport
        );

        verbose!("(global_import::mutability = {:?})", mutability);

        Ok(ImportDesc::Global {
            content_type,
            mutability,
        })
    }

    /******** TABLE, MEMORY, GLOBAL ********/

    /// TODO: TEST
    pub fn table_entry(&mut self) -> ParserResult<Table> {
        let cursor = self.cursor;

        // Get element type.
        let element_type = get_value!(
            self.varint7(),
            cursor,
            IncompleteTableEntry,
            MalformedElementTypeInTableEntry
        );

        // Check if type is not funtref.
        if element_type != -0x10 {
            return Err(ParserError {
                kind: ErrorKind::InvalidElementTypeInTableEntry,
                cursor,
            });
        }

        // Get table limits.
        let (minimum, maximum) = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteTableEntry,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInTableEntry,
                    ErrorKind::MalformedMinimumInLimits => ErrorKind::MalformedMinimumInTableEntry,
                    ErrorKind::MalformedMaximumInLimits => ErrorKind::MalformedMaximumInTableEntry,
                    _ => ErrorKind::MalformedLimitsInTableEntry,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        Ok(Table {
            element_type: element_type.into(),
            minimum,
            maximum,
        })
    }

    /// TODO: TEST
    pub fn memory_entry(&mut self) -> ParserResult<Memory> {
        let cursor = self.cursor;

        // Get memory limits.
        let (minimum, maximum) = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteMemoryEntry,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInMemoryEntry,
                    ErrorKind::MalformedMinimumInLimits => ErrorKind::MalformedMinimumInMemoryEntry,
                    ErrorKind::MalformedMaximumInLimits => ErrorKind::MalformedMaximumInMemoryEntry,
                    _ => ErrorKind::MalformedLimitsInMemoryEntry,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        Ok(Memory { minimum, maximum })
    }

    /// TODO: TEST
    pub fn global_entry(&mut self) -> ParserResult<Global> {
        verbose!("-> global_entry! <-");
        let cursor = self.cursor;

        // Get content type
        let content_type = Type::from(get_value!(
            self.value_type(),
            cursor,
            IncompleteGlobalEntry,
            MalformedContentTypeInGlobalEntry
        ));

        verbose!("(global_entry::content_type = {:?})", content_type);

        // Get mutability
        let mutability = get_value!(
            self.varuint1(),
            cursor,
            IncompleteGlobalEntry,
            MalformedMutabilityInGlobalEntry
        );

        verbose!("(global_entry::mutability = {:?})", mutability);

        // Consume instructions
        let instructions = self.instructions()?;

        Ok(Global {
            content_type,
            mutability,
            instructions,
        })
    }

    /******** EXPORTS ********/

    /// TODO: TEST
    pub fn export_entry(&mut self) -> ParserResult<Export> {
        verbose!("-> export_entry! <-");
        let cursor = self.cursor;
        let mut name = String::new();

        // Get module name length
        let name_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportEntry,
            MalformedNameLengthInExportEntry
        );

        verbose!("(export_entry::name_len = 0x{:x})", name_len);

        {
            // TODO: Validate UTF-8
            name = match self.eat_bytes(name_len as _) {
                Some(value) => str::from_utf8(value).unwrap().into(),
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteExportEntry,
                        cursor,
                    });
                }
            };

            verbose!("export_entry::name = {:?}", name);
        }

        let export_kind = get_value!(
            self.external_kind(),
            cursor,
            IncompleteExportEntry,
            MalformedExportKindInExportEntry
        );

        verbose!("export_entry::export_kind = {:?}", export_kind);

        let index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportEntry,
            MalformedExportIndexInExportEntry
        );

        verbose!("export_entry::index = {:?}", index);

        let desc = match export_kind {
            // Function export
            0x00 => ExportDesc::Function(index),
            // Table export
            0x01 => ExportDesc::Table(index),
            // Memory export
            0x02 => ExportDesc::Memory(index),
            // Global export
            0x03 => ExportDesc::Global(index),
            _ => {
                return Err(ParserError {
                    kind: ErrorKind::InvalidExportTypeInExportEntry,
                    cursor,
                });
            }
        };

        Ok(Export { name, desc })
    }

    /******** ELEMENT ********/

    /// TODO: TEST
    pub fn element_entry(&mut self) -> ParserResult<Element> {
        verbose!("-> element_entry! <-");
        let cursor = self.cursor;
        let mut func_indices = vec![];

        // Get table index.
        let table_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteElementEntry,
            MalformedTableIndexInElementEntry
        );

        verbose!("(element_entry::table_index = 0x{:x})", table_index);

        // Consume code.
        let instructions = self.instructions()?;

        // Get count of function indices.
        let func_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteElementEntry,
            MalformedFunctionCountInElementEntry
        );

        verbose!("(element_entry::func_count = 0x{:x})", func_count);

        // Consume function indices
        for _ in 0..func_count {
            let func_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteElementEntry,
                MalformedFunctionIndexInElementEntry
            );

            func_indices.push(func_index);
        }

        Ok(Element {
            table_index,
            instructions,
            func_indices,
        })
    }

    /******** CODE ********/

    /// TODO: TEST
    /// Each function body corresponds to the functions declared in the function section.
    pub fn function_body(&mut self) -> ParserResult<Function> {
        verbose!("-> function_body! <-");
        let cursor = self.cursor;
        let mut locals = vec![];

        // The length of the code section in bytes.
        let body_size = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        verbose!("(function_body::body_size = 0x{:x})", body_size);

        // Start position of body bytes.
        let start_pos = self.cursor;

        // Get count of locals.
        let local_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        verbose!("(function_body::local_count = 0x{:x})", local_count);

        // Consume locals.
        for _ in 0..local_count {
            locals.push(self.local_entry()?);
        }

        // Consume code.
        let instructions = self.instructions()?;

        // Get the amount of bytes consumed for locals and code.
        let diff = self.cursor - start_pos;

        // Check if the diff matches the body size
        if (body_size as usize) != diff {
            return Err(ParserError {
                kind: ErrorKind::BodySizeDoesNotMatchContentOfFunctionBody,
                cursor,
            });
        }

        Ok(Function {
            locals,
            instructions,
        })
    }

    /// TODO: TEST
    pub fn local_entry(&mut self) -> ParserResult<Local> {
        verbose!("-> local_entry! <-");
        let cursor = self.cursor;

        // Get count of locals with similar types.
        let count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteLocalEntry,
            MalformedCountInLocalEntry
        );

        verbose!("(local_entry::count = 0x{:x})", count);

        // Get type of the locals.
        let local_type = Type::from(get_value!(
            self.value_type(),
            cursor,
            IncompleteLocalEntry,
            MalformedLocalTypeInLocalEntry
        ));

        verbose!("(local_entry::local_type = {:?})", local_type);

        Ok(Local { count, local_type })
    }

    /// TODO: TEST
    pub fn instructions(&mut self) -> ParserResult<Vec<Operator>> {
        verbose!("-> instructions! <-");
        let cursor = self.cursor;
        let mut operators = vec![];

        loop {
            let opcode = get_value!(
                self.uint8(),
                cursor,
                IncompleteExpression,
                MalformedOpcodeInExpression
            );

            verbose!("(instructions::opcode = 0x{:x})", opcode);

            // If opcode is an end byte. Break!
            if opcode == 0x0b {
                break;
            }

            operators.push(self.operator(opcode)?);
        }

        Ok(operators)
    }

    /// TODO: TEST
    pub fn operator(&mut self, opcode: u8) -> ParserResult<Operator> {
        // Dispatch to the right
        let operation = match opcode {
            // CONTROL FLOW
            0x0b => Operator::End,
            0x00 => Operator::Unreachable,
            0x01 => Operator::Nop,
            0x02 => unimplemented!(),
            0x03 => unimplemented!(),
            0x04 => unimplemented!(),
            0x05 => unimplemented!(),
            0x0c => unimplemented!(),
            0x0d => unimplemented!(),
            0x0e => unimplemented!(),
            0x0f => unimplemented!(),
            // CALL
            0x10 => unimplemented!(),
            0x11 => unimplemented!(),
            // PARAMETRIC
            0x1A => unimplemented!(),
            0x1B => unimplemented!(),
            // VARIABLE ACCESS
            0x20 => unimplemented!(),
            0x21 => unimplemented!(),
            0x22 => unimplemented!(),
            0x23 => unimplemented!(),
            0x24 => unimplemented!(),
            // MEMORY
            0x28 => unimplemented!(),
            0x29 => unimplemented!(),
            0x2a => unimplemented!(),
            0x2b => unimplemented!(),
            0x2c => unimplemented!(),
            0x2d => unimplemented!(),
            0x2e => unimplemented!(),
            0x2f => unimplemented!(),
            0x30 => unimplemented!(),
            0x31 => unimplemented!(),
            0x32 => unimplemented!(),
            0x33 => unimplemented!(),
            0x34 => unimplemented!(),
            0x35 => unimplemented!(),
            0x36 => unimplemented!(),
            0x37 => unimplemented!(),
            0x38 => unimplemented!(),
            0x39 => unimplemented!(),
            0x3a => unimplemented!(),
            0x3b => unimplemented!(),
            0x3c => unimplemented!(),
            0x3d => unimplemented!(),
            0x3e => unimplemented!(),
            0x3f => unimplemented!(),
            0x40 => unimplemented!(),
            // CONSTANTS
            0x41 => unimplemented!(),
            0x42 => unimplemented!(),
            0x43 => unimplemented!(),
            0x44 => unimplemented!(),
            // COMPARISONS
            0x45 => unimplemented!(),
            0x46 => unimplemented!(),
            0x47 => unimplemented!(),
            0x48 => unimplemented!(),
            0x49 => unimplemented!(),
            0x4a => unimplemented!(),
            0x4b => unimplemented!(),
            0x4c => unimplemented!(),
            0x4d => unimplemented!(),
            0x4e => unimplemented!(),
            0x4f => unimplemented!(),
            0x50 => unimplemented!(),
            0x51 => unimplemented!(),
            0x52 => unimplemented!(),
            0x53 => unimplemented!(),
            0x54 => unimplemented!(),
            0x55 => unimplemented!(),
            0x56 => unimplemented!(),
            0x57 => unimplemented!(),
            0x58 => unimplemented!(),
            0x59 => unimplemented!(),
            0x5a => unimplemented!(),
            0x5b => unimplemented!(),
            0x5c => unimplemented!(),
            0x5d => unimplemented!(),
            0x5e => unimplemented!(),
            0x5f => unimplemented!(),
            0x60 => unimplemented!(),
            0x61 => unimplemented!(),
            0x62 => unimplemented!(),
            0x63 => unimplemented!(),
            0x64 => unimplemented!(),
            0x65 => unimplemented!(),
            0x66 => unimplemented!(),
            // NUMERIC
            0x67 => unimplemented!(),
            0x68 => unimplemented!(),
            0x69 => unimplemented!(),
            0x6a => unimplemented!(),
            0x6b => unimplemented!(),
            0x6c => unimplemented!(),
            0x6d => unimplemented!(),
            0x6e => unimplemented!(),
            0x6f => unimplemented!(),
            0x70 => unimplemented!(),
            0x71 => unimplemented!(),
            0x72 => unimplemented!(),
            0x73 => unimplemented!(),
            0x74 => unimplemented!(),
            0x75 => unimplemented!(),
            0x76 => unimplemented!(),
            0x77 => unimplemented!(),
            0x78 => unimplemented!(),
            0x79 => unimplemented!(),
            0x7a => unimplemented!(),
            0x7b => unimplemented!(),
            0x7c => unimplemented!(),
            0x7d => unimplemented!(),
            0x7e => unimplemented!(),
            0x7f => unimplemented!(),
            0x80 => unimplemented!(),
            0x81 => unimplemented!(),
            0x82 => unimplemented!(),
            0x83 => unimplemented!(),
            0x84 => unimplemented!(),
            0x85 => unimplemented!(),
            0x86 => unimplemented!(),
            0x87 => unimplemented!(),
            0x88 => unimplemented!(),
            0x89 => unimplemented!(),
            0x8a => unimplemented!(),
            0x8b => unimplemented!(),
            0x8c => unimplemented!(),
            0x8d => unimplemented!(),
            0x8e => unimplemented!(),
            0x8f => unimplemented!(),
            0x90 => unimplemented!(),
            0x91 => unimplemented!(),
            0x92 => unimplemented!(),
            0x93 => unimplemented!(),
            0x94 => unimplemented!(),
            0x95 => unimplemented!(),
            0x96 => unimplemented!(),
            0x97 => unimplemented!(),
            0x98 => unimplemented!(),
            0x99 => unimplemented!(),
            0x9a => unimplemented!(),
            0x9b => unimplemented!(),
            0x9c => unimplemented!(),
            0x9d => unimplemented!(),
            0x9e => unimplemented!(),
            0x9f => unimplemented!(),
            0xa0 => unimplemented!(),
            0xa1 => unimplemented!(),
            0xa2 => unimplemented!(),
            0xa3 => unimplemented!(),
            0xa4 => unimplemented!(),
            0xa5 => unimplemented!(),
            0xa6 => unimplemented!(),
            // CONVERSIONS
            0xa7 => unimplemented!(),
            0xa8 => unimplemented!(),
            0xa9 => unimplemented!(),
            0xaa => unimplemented!(),
            0xab => unimplemented!(),
            0xac => unimplemented!(),
            0xad => unimplemented!(),
            0xae => unimplemented!(),
            0xaf => unimplemented!(),
            0xb0 => unimplemented!(),
            0xb1 => unimplemented!(),
            0xb2 => unimplemented!(),
            0xb3 => unimplemented!(),
            0xb4 => unimplemented!(),
            0xb5 => unimplemented!(),
            0xb6 => unimplemented!(),
            0xb7 => unimplemented!(),
            0xb8 => unimplemented!(),
            0xb9 => unimplemented!(),
            0xba => unimplemented!(),
            0xbb => unimplemented!(),
            // REINTERPRETATIONS
            0xbc => unimplemented!(),
            0xbd => unimplemented!(),
            0xbe => unimplemented!(),
            0xbf => unimplemented!(),
            _ => unimplemented!(),
        };

        Ok(operation)
    }

    /******** DATA ********/

    /// TODO: TEST
    pub fn data_entry(&mut self) -> ParserResult<Data> {
        verbose!("-> data_entry! <-");
        let cursor = self.cursor;

        // Get memory index.
        let memory_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteDataEntry,
            MalformedTableIndexInDataEntry
        );

        verbose!("(data_entry::memory_index = 0x{:x})", memory_index);

        // Consume code.
        let instructions = self.instructions()?;

        // Get count of following bytes.
        let byte_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteDataEntry,
            MalformedByteCountInDataEntry
        );

        verbose!("(data_entry::func_count = 0x{:x})", byte_count);

        // Consume bytes.
        let bytes = match self.eat_bytes(byte_count as _) {
            Some(value) => value.to_vec(),
            None => {
                return Err(ParserError {
                    kind: ErrorKind::IncompleteDataEntry,
                    cursor,
                });
            }
        };

        Ok(Data {
            memory_index,
            instructions,
            bytes,
        })
    }

    /******** TYPES ********/

    /// TODO: TEST
    pub fn limits(&mut self) -> Result<(u32, Option<u32>), ParserError> {
        // verbose!("-> limits! <-");
        let cursor = self.cursor;
        let flags = get_value!(
            self.varuint1(),
            cursor,
            IncompleteLimits,
            MalformedFlagsInLimits
        );

        let minimum = get_value!(
            self.varuint32(),
            cursor,
            IncompleteLimits,
            MalformedMinimumInLimits
        );

        // Get maximum if specified.
        let maximum = if flags {
            match self.varuint32() {
                Ok(value) => Some(value),
                Err(error) => {
                    if error == ErrorKind::BufferEndReached {
                        return Err(ParserError {
                            kind: ErrorKind::IncompleteLimits,
                            cursor,
                        });
                    } else {
                        return Err(ParserError {
                            kind: ErrorKind::MalformedMaximumInLimits,
                            cursor,
                        });
                    }
                }
            }
        } else {
            None
        };

        Ok((minimum, maximum))
    }

    /// TODO: TEST
    /// TODO: Supports a single return type for now.
    pub fn func_type(&mut self) -> ParserResult<Type> {
        // verbose!("-> func_type! <-");
        let cursor = self.cursor;
        let mut params = vec![];
        let mut returns = vec![];

        // Get param count.
        let param_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionType,
            MalformedParamCountInFunctionType
        );

        verbose!("(func_type::param_count = 0x{:x})", param_count);

        // Get param types.
        for _ in 0..param_count {
            let param_type = Type::from(get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            ));

            params.push(param_type);
        }
        verbose!("(func_type::param_types = {:?})", params);

        // Get return count.
        let return_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionType,
            MalformedReturnCountInFunctionType
        );

        verbose!("(func_type::return_count = {:?})", return_count);

        // Get return types.
        for _ in 0..return_count {
            let return_type = Type::from(get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            ));

            returns.push(return_type);
        }
        verbose!("(func_type::return_types = {:?})", returns);

        Ok(Type::Func { params, returns })
    }

    /// TODO: TEST
    pub fn value_type(&mut self) -> Result<i8, ErrorKind> {
        // verbose!("-> value_type! <-");
        let value = self.varint7()?;

        // i32, i64, f32, f64
        if value == -0x01 || value == -0x02 || value == -0x03 || value == -0x04 {
            Ok(value as _)
        } else {
            Err(ErrorKind::InvalidValueType)
        }
    }

    /// TODO: TEST
    pub fn external_kind(&mut self) -> Result<u8, ErrorKind> {
        verbose!("-> external_kind! <-");

        let value = self.uint8()?;

        // function_import, table_import, memory_imoort, global_import
        if value == 0x00 || value == 0x01 || value == 0x02 || value == 0x03 {
            Ok(value as _)
        } else {
            Err(ErrorKind::InvalidImportType)
        }
    }

    /******** UTILS ********/

    /// TODO: TEST
    pub fn skip_to_section(&mut self, section_id: u8) -> ParserResult<()> {
        loop {
            if section_id == self.section_id()? {
                break;
            }
            let payload_len = self.peek_varuint32()?;
            if !self.skip(payload_len as _) {
                return Err(ParserError {
                    kind: ErrorKind::BufferEndReached,
                    cursor: self.cursor,
                });
            };
        }
        Ok(())
    }

    /// TODO: TEST
    pub fn peek_varuint32(&mut self) -> ParserResult<u32> {
        let cursor = self.cursor;
        let value = self.varuint32();
        self.cursor = cursor;
        match value {
            Ok(value) => Ok(value),
            Err(kind) => Err(ParserError { kind, cursor }),
        }
    }

    /// TODO: TEST
    pub fn skip(&mut self, len: usize) -> bool {
        let jump = self.cursor + len + 1;
        // Check if jump is within code buffer bounds
        if jump > self.code.len() {
            return false;
        }
        true
    }

    /// Gets a byte from the code buffer and (if available)
    /// advances the cursor.
    pub fn eat_byte(&mut self) -> Option<u8> {
        let index = self.cursor;
        // Check if range is within code buffer bounds
        if index < self.code.len() {
            // Advance the cursor
            self.cursor += 1;
            return Some(self.code[index]);
        }
        None
    }

    /// Gets the next `range` slice of bytes from the code buffer
    /// (if available) and advances the token.
    pub fn eat_bytes(&mut self, range: usize) -> Option<&[u8]> {
        let start = self.cursor;
        let end = start + range;
        // Check if range is within code buffer bounds
        if end > self.code.len() {
            return None;
        }
        // Advance the cursor
        self.cursor = end;
        Some(&self.code[start..end])
    }

    /// Consumes 1 byte that represents an 8-bit unsigned integer
    pub fn uint8(&mut self) -> Result<u8, ErrorKind> {
        if let Some(byte) = self.eat_byte() {
            return Ok(byte);
        }
        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes 2 bytes that represent a 16-bit unsigned integer
    pub fn uint16(&mut self) -> Result<u16, ErrorKind> {
        if let Some(bytes) = self.eat_bytes(2) {
            let mut shift = 0;
            let mut result = 0;
            for byte in bytes {
                result |= (*byte as u16) << shift;
                shift += 8;
            }
            return Ok(result);
        }
        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes 4 bytes that represent a 32-bit unsigned integer
    pub fn uint32(&mut self) -> Result<u32, ErrorKind> {
        if let Some(bytes) = self.eat_bytes(4) {
            let mut shift = 0;
            let mut result = 0;
            for byte in bytes {
                result |= (*byte as u32) << shift;
                shift += 8;
            }
            return Ok(result);
        }
        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes a byte that represents a 1-bit LEB128 unsigned integer encoding
    pub fn varuint1(&mut self) -> Result<bool, ErrorKind> {
        if let Some(byte) = self.eat_byte() {
            return match byte {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(ErrorKind::MalformedVaruint1),
            };
        }
        // We expect the if statement to return an Ok result. If it doesn't
        // then we are trying to read more than 1 byte, which is malformed for a varuint1
        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes a byte that represents a 7-bit LEB128 unsigned integer encoding
    pub fn varuint7(&mut self) -> Result<u8, ErrorKind> {
        if let Some(byte) = self.eat_byte() {
            let result = byte;
            // Check if msb is unset.
            if result & 0b1000_0000 != 0 {
                return Err(ErrorKind::MalformedVaruint7);
            }
            return Ok(result);
        }
        // We expect the if statement to return an Ok result. If it doesn't
        // then we are trying to read more than 1 byte, which is malformed for a varuint7
        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes 1-5 bytes that represent a 32-bit LEB128 unsigned integer encoding
    pub fn varuint32(&mut self) -> Result<u32, ErrorKind> {
        // verbose!("-> varuint32! <-");
        let mut result = 0;
        let mut shift = 0;
        while shift < 35 {
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // verbose!("(count = {}, byte = 0b{:08b})", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b1000_0000) as u32) << shift;
            result |= value;
            // Return if any of the bytes has an unset msb
            if byte & 0b1000_0000 == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        // We expect the loop to terminate early and return an Ok result. If it doesn't
        // then we are trying to read more than 5 bytes, which is malformed for a varuint32
        Err(ErrorKind::MalformedVaruint32)
    }

    /// Consumes a byte that represents a 7-bit LEB128 signed integer encoding
    pub fn varint7(&mut self) -> Result<i8, ErrorKind> {
        if let Some(byte) = self.eat_byte() {
            let mut result = byte;
            // Check if msb is unset.
            if result & 0b1000_0000 != 0 {
                return Err(ErrorKind::MalformedVarint7);
            }
            // If the 7-bit value is signed, extend the sign.
            if result & 0b0100_0000 == 0b0100_0000 {
                result |= 0b1000_0000;
            }
            return Ok(result as i8);
        }

        Err(ErrorKind::BufferEndReached)
    }

    /// Consumes 1-5 bytes that represent a 32-bit LEB128 signed integer encoding
    pub fn varint32(&mut self) -> Result<i32, ErrorKind> {
        // verbose!("-> varint32! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 5 bytes
        while shift < 35 {
            // (shift = 0, 7, 14 .. 35)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // verbose!("(count = {}, byte = 0b{:08b})", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b1000_0000) as i32) << shift;
            result |= value;
            // Return if any of the bytes has an unset msb
            if byte & 0b1000_0000 == 0 {
                // Extend sign if sign bit is set. We don't bother when we are on the 5th byte
                // (hence shift < 28) because it gives an 32-bit value, so no need for sign
                // extension there
                if shift < 28 && byte & 0b0100_0000 != 0 {
                    result |= -1 << (7 + shift); // -1 == 0xff_ff_ff_ff
                }
                return Ok(result);
            }
            shift += 7;
        }
        // We expect the loop to terminate early and return an Ok result. If it doesn't
        // then we are trying to read more than 5 bytes, which is malformed for a varint32
        Err(ErrorKind::MalformedVarint32)
    }

    /// TODO: TEST
    /// Consumes 1-9 bytes that represent a 64-bit LEB128 signed integer encoding
    pub fn varint64(&mut self) -> Result<i64, ErrorKind> {
        // verbose!("-> varint64! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 9 bytes
        while shift < 63 {
            // (shift = 0, 7, 14 .. 56)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // verbose!("(count = {}, byte = 0b{:08b})", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b1000_0000) as i64) << shift;
            result |= value;
            // Return if any of the bytes has an unset msb
            if byte & 0b1000_0000 == 0 {
                // Extend sign if sign bit is set. We don't bother when we are on the 9th byte
                // (hence shift < 56) because it gives an 64-bit value, so no need for sign
                // extension there
                if shift < 56 && byte & 0b0100_0000 != 0 {
                    result |= -1 << (7 + shift); // -1 == 0xff_ff_ff_ff
                }
                return Ok(result);
            }
            shift += 7;
        }
        // We expect the loop to terminate early and return an Ok result. If it doesn't
        // then we are trying to read more than 5 bytes, which is malformed for a varint64
        Err(ErrorKind::MalformedVarint64)
    }
}
