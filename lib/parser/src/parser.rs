#[macro_use]
use std::str;
use crate::{
    errors::ParserError,
    ir::{
        Export, ExportDesc, Function, Global, Import, ImportDesc, Local, Memory, Module, Operator,
        Section, Table, Type,
    },
    kinds::{ErrorKind, SectionKind},
    macros,
    utils::{int_to_section, int_to_type},
    validation::validate_section_exists,
};
use wasmlite_utils::*;

// TODO
//  - Improve error reporting.

pub type ParserResult<T> = Result<T, ParserError>;

/// A WebAssembly module parser.
///
/// The error handling mechanism
/// - Errors start at the primitive read functions like (varuint or uint8) and propagate up the call stack with each enclosing function
///   fixing the error message to provide more context.
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
        debug!("-> module! <-");

        // Consume preamble.
        self.module_preamble()?;

        self.sections().unwrap(); // Optional

        Ok(())
    }

    /// TODO: TEST
    /// Checks if the following bytes are expected
    /// wasm preamble bytes.
    pub fn module_preamble(&mut self) -> ParserResult<()> {
        debug!("-> module_preamble! <-");
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

        debug!("(module_preamble::magic_no = 0x{:08x})", magic_no);

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

        debug!("(module_preamble::version_no = 0x{:08x})", version_no);

        Ok(())
    }

    /// TODO: TEST
    pub fn sections(&mut self) -> ParserResult<Vec<Section>> {
        debug!("-> module_sections! <-");

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

            debug!(
                "(module_sections::section code = {:?})",
                int_to_section(section_id)
            );

            // Consume appropriate section based on section id.
            sections.push(self.section(section_id)?);
        }

        Ok(sections)
    }

    /// Gets the next section id and payload.
    /// Needed by reader
    pub fn section_id(&mut self) -> ParserResult<u8> {
        debug!("-> module_section_id! <-");
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
        debug!("-> section! <-");
        let cursor = self.cursor;

        Ok(match section_id {
            0x00 => self.custom_section()?,
            0x01 => self.type_section()?,
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
        debug!("-> custom_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varint32(),
            cursor,
            IncompleteCustomSection,
            MalformedPayloadLengthInCustomSection
        );

        // Get name length of custom section.
        let name_len = get_value!(
            self.varint32(),
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
    pub fn type_section(&mut self) -> ParserResult<Section> {
        debug!("-> type_section! <-");
        let cursor = self.cursor;
        let mut func_types = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedPayloadLengthInTypeSection
        );

        debug!("(type_section::payload_len = 0x{:x})", payload_len);

        // Get the count of type entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedEntryCountInTypeSection
        );

        debug!("(type_section::entry_count = 0x{:x})", entry_count);

        // Consume the type entries.
        for _ in 0..entry_count {
            let type_id = get_value!(
                self.varint7(),
                cursor,
                EntriesDoNotMatchEntryCountInTypeSection,
                MalformedTypeInTypeSection
            );

            debug!("(type_section::type_id = {:?})", type_id);

            // Type must be a func type.
            let func_type = match type_id {
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
        debug!("-> import_section! <-");
        let cursor = self.cursor;
        let mut imports = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportSection,
            MalformedPayloadLengthInImportSection
        );

        debug!("(import_section::payload_len = 0x{:x})", payload_len);

        // Get the count of import entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportSection,
            MalformedEntryCountInImportSection
        );

        debug!("(import_section::entry_count = 0x{:x})", entry_count);

        // Consume the import entries.
        for _ in 0..entry_count {
            imports.push(self.import_entry()?);
        }

        // TODO
        Ok(Section::Import(imports))
    }

    /// TODO: TEST
    pub fn function_section(&mut self) -> ParserResult<Section> {
        debug!("-> function_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedPayloadLengthInFunctionSection
        );

        debug!("(function_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function entries,
        let function_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedEntryCountInFunctionSection
        );

        debug!(
            "(function_section::function_count = 0x{:x})",
            function_count
        );

        // Consume the function entries.
        for _ in 0..function_count {
            // Index into the type section.
            let type_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            debug!("(function_section::type_index = 0x{:x})", type_index);
        }

        // TODO
        Ok(Section::Function(vec![]))
    }

    /// TODO: TEST
    pub fn table_section(&mut self) -> ParserResult<Section> {
        debug!("-> table_section! <-");
        let cursor = self.cursor;
        let mut tables = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedPayloadLengthInTableSection
        );

        debug!("(table_section::payload_len = 0x{:x})", payload_len);

        // Get the count of table entries,
        let table_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedEntryCountInTableSection
        );

        debug!("(table_section::table_count = 0x{:x})", table_count);

        // Consume the function entries.
        for _ in 0..table_count {
            // Get table type.
            let (element_type, minimum, maximum) = get_value!(
                self.table_type(),
                cursor,
                IncompleteTableSection,
                MalformedEntryInTableSection
            );

            tables.push(Table {
                element_type,
                minimum,
                maximum,
            });

            debug!(
                "(table_section::table_type = {:?})",
                (element_type, minimum, maximum)
            );
        }

        // TODO
        Ok(Section::Table(tables))
    }

    /// TODO: TEST
    pub fn memory_section(&mut self) -> ParserResult<Section> {
        debug!("-> memory_section! <-");
        let cursor = self.cursor;
        let mut memories = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedPayloadLengthInTableSection
        );

        debug!("(memory_section::payload_len = 0x{:x})", payload_len);

        // Get the count of memory entries.
        let memory_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedEntryCountInTableSection
        );

        debug!("(memory_section::memory_count = 0x{:x})", memory_count);

        // Consume the entries.
        for _ in 0..memory_count {
            // Gewt memory type.
            let (minimum, maximum) = get_value!(
                self.memory_type(),
                cursor,
                IncompleteTableSection,
                MalformedEntryInTableSection
            );

            memories.push(Memory { minimum, maximum });

            debug!("(memory_section::memory_type = {:?})", (minimum, maximum));
        }

        Ok(Section::Memory(memories))
    }

    /// TODO: TEST
    pub fn global_section(&mut self) -> ParserResult<Section> {
        debug!("-> global_section! <-");
        let cursor = self.cursor;
        let mut globals = vec![];

        // The length of the global section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedPayloadLengthInFunctionSection
        );

        debug!("(global_section::payload_len = 0x{:x})", payload_len);

        // Get the count of table entries,
        let global_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTableSection,
            MalformedEntryCountInTableSection
        );

        debug!("(global_section::global_count = 0x{:x})", global_count);

        // Consume the global entries.
        for _ in 0..global_count {
            // Get global.
            let global = get_value!(
                self.global(),
                cursor,
                IncompleteTableSection,
                MalformedEntryInTableSection
            );

            globals.push(global);

            debug!("(memory_section::global_type = {:?})", global);
        }

        Ok(Section::Global(globals))
    }

    /// TODO: TEST
    pub fn export_section(&mut self) -> ParserResult<Section> {
        debug!("-> export_section! <-");
        let cursor = self.cursor;
        let mut exports = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportSection,
            MalformedPayloadLengthInExportSection
        );

        debug!("(export_section::payload_len = 0x{:x})", payload_len);

        // Get the count of import entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportSection,
            MalformedEntryCountInExportSection
        );

        debug!("(export_section::entry_count = 0x{:x})", entry_count);

        // Consume the import entries.
        for _ in 0..entry_count {
            exports.push(self.export_entry()?);
        }

        // TODO
        Ok(Section::Export(exports))
    }

    /// TODO: TEST
    pub fn start_section(&mut self) -> ParserResult<Section> {
        debug!("-> start_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteStartSection,
            MalformedPayloadLengthInStartSection
        );

        debug!("(start_section::payload_len = 0x{:x})", payload_len);

        // Get the indes of the start function,
        let function_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteStartSection,
            MalformedEntryCountInStartSection
        );

        // TODO
        Ok(Section::Start(function_index))
    }

    /// TODO: TEST
    pub fn element_section(&mut self) -> ParserResult<Section> {
        debug!("-> element_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedPayloadLengthInFunctionSection
        );

        debug!("(function_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function entries,
        let function_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedEntryCountInFunctionSection
        );

        debug!(
            "(function_section::function_count = 0x{:x})",
            function_count
        );

        // Consume the function entries.
        for _ in 0..function_count {
            // Index into the type section.
            let type_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            debug!("(function_section::type_index = 0x{:x})", type_index);
        }

        // TODO
        Ok(Section::Function(vec![]))
    }

    /// TODO: TEST
    pub fn code_section(&mut self) -> ParserResult<Section> {
        debug!("-> code_section! <-");
        let cursor = self.cursor;
        let mut function_bodies = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCodeSection,
            MalformedPayloadLengthInCodeSection
        );

        debug!("(code_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function bodies.
        let body_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteCodeSection,
            MalformedBodyCountInCodeSection
        );

        debug!("(code_section::entry_count = 0x{:x})", body_count);

        // Consume the function bodies.
        for _ in 0..body_count {
            function_bodies.push(self.function_body()?);
        }

        // TODO
        Ok(Section::Code(function_bodies))
    }

    /// TODO: TEST
    pub fn data_section(&mut self) -> ParserResult<Section> {
        debug!("-> data_section! <-");
        let cursor = self.cursor;

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedPayloadLengthInFunctionSection
        );

        debug!("(function_section::payload_len = 0x{:x})", payload_len);

        // Get the count of function entries,
        let function_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionSection,
            MalformedEntryCountInFunctionSection
        );

        debug!(
            "(function_section::function_count = 0x{:x})",
            function_count
        );

        // Consume the function entries.
        for _ in 0..function_count {
            // Index into the type section.
            let type_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            debug!("(function_section::type_index = 0x{:x})", type_index);
        }

        // TODO
        Ok(Section::Function(vec![]))
    }

    /******** IMPORTS ********/

    /// TODO: TEST
    pub fn import_entry(&mut self) -> ParserResult<Import> {
        debug!("-> import_entry! <-");
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

        debug!("(import_entry::module_len = 0x{:x})", module_len);

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

            debug!("import_entry::_module_str = {:?}", module_name);
        }

        // Get field name length
        let field_len = get_value!(
            self.varint32(),
            cursor,
            IncompleteImportEntry,
            MalformedFieldNameLengthInImportEntry
        );

        debug!("(import_entry::field_len = 0x{:x})", field_len);

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

            debug!("(import_entry::_field_str = {:?})", field_name);
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
        debug!("-> function_import! <-");
        let cursor = self.cursor;
        let type_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionImport,
            MalformedTypeIndexInFunctionImport
        );

        debug!("(function_import::type_index = {:?})", type_index);

        Ok(ImportDesc::Function { type_index })
    }

    /// TODO: TEST
    pub fn table_import(&mut self) -> ParserResult<ImportDesc> {
        debug!("-> table_import! <-");
        let cursor = self.cursor;
        let element_type = int_to_type(match self.varint7() {
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

        debug!("(table_import::element_type = {:?})", element_type);

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

        debug!("(table_import::minimum = {:?})", minimum);

        debug!("(table_import::maximum = {:?})", maximum);

        Ok(ImportDesc::Table(Table {
            element_type,
            minimum,
            maximum,
        }))
    }

    /// TODO: TEST
    pub fn memory_import(&mut self) -> ParserResult<ImportDesc> {
        debug!("-> memory_import! <-");
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

        debug!("(memory_import::minimum = {:?})", minimum);

        debug!("(memory_import::maximum = {:?})", maximum);

        Ok(ImportDesc::Memory(Memory { minimum, maximum }))
    }

    /// TODO: TEST
    pub fn global_import(&mut self) -> ParserResult<ImportDesc> {
        debug!("-> global_import! <-");
        let cursor = self.cursor;

        let content_type = int_to_type(get_value!(
            self.value_type(),
            cursor,
            IncompleteGlobalImport,
            MalformedContentTypeInGlobalImport
        ));

        debug!("(global_import::content_type = {:?})", content_type);
        let mutability = get_value!(
            self.varuint1(),
            cursor,
            IncompleteGlobalImport,
            MalformedMutabilityInGlobalImport
        );

        debug!("(global_import::mutability = {:?})", mutability);

        Ok(ImportDesc::Global {
            content_type,
            mutability,
        })
    }

    /******** EXPORTS ********/

    /// TODO: TEST
    pub fn export_entry(&mut self) -> ParserResult<Export> {
        debug!("-> export_entry! <-");
        let cursor = self.cursor;
        let mut name = String::new();

        // Get module name length
        let name_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportEntry,
            MalformedNameLengthInExportEntry
        );

        debug!("(export_entry::name_len = 0x{:x})", name_len);

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

            debug!("export_entry::name = {:?}", name);
        }

        let export_kind = get_value!(
            self.external_kind(),
            cursor,
            IncompleteExportEntry,
            MalformedExportKindInExportEntry
        );

        debug!("export_entry::export_kind = {:?}", export_kind);

        let index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExportEntry,
            MalformedModuleNameLengthInExportEntry
        );

        debug!("export_entry::index = {:?}", index);

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

    /******** CODE ********/

    /// TODO: TEST
    /// Each function body corresponds to the functions declared in the function section.
    pub fn function_body(&mut self) -> ParserResult<Function> {
        debug!("-> function_body! <-");
        let cursor = self.cursor;
        let mut locals = vec![];
        let mut instructions = vec![];

        // The length of the code section in bytes.
        let body_size = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        debug!("(function_body::body_size = 0x{:x})", body_size);

        // Start position of body bytes.
        let start_pos = self.cursor;

        // Get count of locals.
        let local_count = get_value!(
            self.varint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        debug!("(function_body::local_count = 0x{:x})", local_count);

        // Consume locals.
        for _ in 0..local_count {
            locals.push(self.local_entry()?);
        }

        // Get the amount of bytes consumed for locals.
        let diff = self.cursor - start_pos;

        // Consume code.
        for _ in (diff + 1)..(body_size as _) {
            instructions = self.instructions()?;
        }

        // Get end byte.
        let end_byte = get_end_byte!(
            self.varint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedEndByteInFunctionBody
        );

        debug!("(function_body::end_byte = 0x{:x})", end_byte);

        Ok(Function {
            locals,
            instructions,
        })
    }

    /// TODO: TEST
    pub fn local_entry(&mut self) -> ParserResult<Local> {
        debug!("-> local_entry! <-");
        let cursor = self.cursor;

        // Get count of locals with similar types.
        let count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedCountInLocalEntry
        );

        debug!("(function_body::count = 0x{:x})", count);

        // Get type of the locals.
        let local_type = int_to_type(get_value!(
            self.value_type(),
            cursor,
            IncompleteTypeSection,
            MalformedTypeInLocalEntry
        ));

        debug!("(function_body::local_type = {:?})", local_type);

        Ok(Local { count, local_type })
    }

    /// TODO: TEST
    pub fn instructions(&mut self) -> ParserResult<Vec<Operator>> {
        debug!("-> instructions! <-");
        let cursor = self.cursor;
        let mut operators = vec![];

        let opcode = get_value!(
            self.uint8(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        loop {
            operators.push(self.operator(opcode)?);
            break;
        }

        Ok(operators)
    }

    /// TODO: TEST
    pub fn operator(&mut self, opcode: u8) -> ParserResult<Operator> {
        // Dispatch to the right
        match opcode {
            // CONTROL FLOW
            0x00 => unimplemented!(),
            0x01 => unimplemented!(),
            0x02 => unimplemented!(),
            0x03 => unimplemented!(),
            0x04 => unimplemented!(),
            0x05 => unimplemented!(),
            0x0b => unimplemented!(),
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
            _ => {}
        }

        Ok(Operator::Nop)
    }

    /******** TYPES ********/

    /// TODO: TEST
    pub fn limits(&mut self) -> Result<(u32, Option<u32>), ParserError> {
        // debug!("-> limits! <-");
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
        // debug!("-> func_type! <-");
        let cursor = self.cursor;
        let mut params = vec![];
        let mut returns = vec![];

        // Get param count.
        let param_count = get_value!(
            self.varint32(),
            cursor,
            IncompleteFunctionType,
            MalformedParamCountInFunctionType
        );

        debug!("(func_type::param_count = 0x{:x})", param_count);

        // Get param types.
        for _ in 0..param_count {
            let param_type = int_to_type(get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            ));

            params.push(param_type);

            debug!("(func_type::param_type = {:?})", param_type);
        }

        // Get return count.
        let return_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionType,
            MalformedReturnCountInFunctionType
        );

        debug!("(func_type::return_count = {:?})", return_count);

        // Get return types.
        for _ in 0..return_count {
            let return_type = int_to_type(get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            ));

            params.push(return_type);

            debug!("(func_type::return_type = {:?})", return_type);
        }

        Ok(Type::Func { params, returns })
    }

    /// TODO: TEST
    pub fn table_type(&mut self) -> ParserResult<(Type, u32, Option<u32>)> {
        let cursor = self.cursor;

        // Get element type.
        let element_type = get_value!(
            self.varint32(),
            cursor,
            IncompleteTableType,
            MalformedFuncrefInFunctionType
        );

        // Check if type is not funtref.
        if element_type != 0x70 {
            return Err(ParserError {
                kind: ErrorKind::InvalidElementTypeInTableType,
                cursor,
            });
        }

        // Get table limits.
        let (minimum, maximum) = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteTableType,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInTableType,
                    ErrorKind::MalformedMinimumInLimits => ErrorKind::MalformedMinimumInTableType,
                    ErrorKind::MalformedMaximumInLimits => ErrorKind::MalformedMaximumInTableType,
                    _ => ErrorKind::MalformedLimitsInTableType,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        Ok((int_to_type(element_type), minimum, maximum))
    }

    /// TODO: TEST
    pub fn memory_type(&mut self) -> ParserResult<(u32, Option<u32>)> {
        let cursor = self.cursor;

        // Get memory limits.
        let limits = match self.limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteMemoryType,
                    ErrorKind::MalformedFlagsInLimits => ErrorKind::MalformedFlagsInMemoryType,
                    ErrorKind::MalformedMinimumInLimits => ErrorKind::MalformedMinimumInMemoryType,
                    ErrorKind::MalformedMaximumInLimits => ErrorKind::MalformedMaximumInMemoryType,
                    _ => ErrorKind::MalformedLimitsInMemoryType,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        Ok(limits)
    }

    /// TODO: TEST
    pub fn global(&mut self) -> ParserResult<Global> {
        debug!("-> global! <-");
        let cursor = self.cursor;
        let mut instructions = vec![];

        // The length of the global in bytes.
        let body_size = get_value!(
            self.varuint32(),
            cursor,
            IncompleteGlobal,
            MalformedBodySizeInGlobal
        );

        debug!("(global::global_size = {:?})", body_size);

        // Start position of global type butes.
        let start_pos = self.cursor;

        // Get content type
        let content_type = int_to_type(get_value!(
            self.value_type(),
            cursor,
            IncompleteGlobal,
            MalformedContentTypeInGlobal
        ));

        debug!("(global::content_type = {:?})", content_type);

        // Get mutability
        let mutability = get_value!(
            self.varuint1(),
            cursor,
            IncompleteGlobal,
            MalformedMutabilityInGlobal
        );

        debug!("(global::mutability = {:?})", mutability);

        // Get init expr
        // Get the amount of bytes consumed for content_type and mutability.
        let diff = self.cursor - start_pos;

        // Consume code.
        for _ in (diff + 1)..(body_size as _) {
            instructions = self.instructions()?;
        }

        // Get end byte.
        let end_byte = get_end_byte!(
            self.varint32(),
            cursor,
            IncompleteGlobal,
            MalformedEndByteInGlobal
        );

        Ok(Global {
            content_type,
            mutability,
            instructions,
        })
    }

    /// TODO: TEST
    pub fn value_type(&mut self) -> Result<i8, ErrorKind> {
        // debug!("-> value_type! <-");
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
        debug!("-> external_kind! <-");

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
        // debug!("-> varuint32! <-");
        let mut result = 0;
        let mut shift = 0;
        while shift < 35 {
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // debug!("(count = {}, byte = 0b{:08b})", count, byte);
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
        // debug!("-> varint32! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 5 bytes
        while shift < 35 {
            // (shift = 0, 7, 14 .. 35)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // debug!("(count = {}, byte = 0b{:08b})", count, byte);
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
        // debug!("-> varint64! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 9 bytes
        while shift < 63 {
            // (shift = 0, 7, 14 .. 56)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ErrorKind::BufferEndReached),
            };
            // debug!("(count = {}, byte = 0b{:08b})", count, byte);
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
