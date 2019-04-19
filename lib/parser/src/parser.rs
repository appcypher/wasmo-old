use crate::{
    errors::ParserError,
    ir::{
        self, Data, Element, Export, ExportDesc, Function, Global, Import, ImportDesc, Local,
        Memory, Module, Operator, Section, Table, Type, ValueType::*, FuncSignature, ValueType,
    },
    kinds::{ErrorKind, SectionKind},
    stack::Stack,
};
use std::str;
use wasmo_utils::{debug, verbose};
use hashbrown::HashMap;

pub type ParserResult<T> = Result<T, ParserError>;

/// A WebAssembly module parser.
///
/// Notes
/// - Just like with body_size in function body. payload_len should be used to determine if section content stay within payload range
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    pub(crate) code: &'a [u8],             // The wasm binary to parse
    pub(crate) cursor: usize, // Used to track the current byte position as the parser advances.
    pub(crate) sections_consumed: Vec<u8>, // Holds the section ids that have been consumed. Section types cannot occur more than once.
    pub(crate) stack: Stack,
    pub(crate) operator_index: usize,
    pub(crate) label_depth: usize,
}

/// Contains the implementation of parser
/// TODO: Validation
/// - payload len
impl<'a> Parser<'a> {
    /// Creates new parser
    pub fn new(code: &'a [u8]) -> Self {
        Parser {
            code,
            cursor: 0, // cursor starts at first byte
            sections_consumed: vec![],
            stack: Stack::new(),
            operator_index: 0,
            label_depth: 0,
        }
    }

    /// Pushes an id into the parser's sections_consumed.
    pub(super) fn push_section_id(&mut self, section_id: &u8) {
        self.sections_consumed.push(*section_id);
    }

    /// TODO: TEST
    /// Generates an IR representing a parsed wasm module.
    pub fn module(&mut self) -> ParserResult<Module> {
        verbose!("-> module! <-");

        // Consume preamble.
        self.module_preamble()?;

        // TODO: Module can stop here.
        let sections = self.sections()?;

        verbose!("(module::sections = {:#?})", sections);

        Ok(Module { sections })
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
    pub fn sections(&mut self) -> ParserResult<HashMap<u8, Section>> {
        verbose!("-> module_sections! <-");

        let mut sections = HashMap::new();

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
            self.validate_section_exists(section_id)?;

            // Save id in list of sections consumed if not custpom section
            if section_id != 0 {
                self.push_section_id(&section_id);
            }

            verbose!(
                "(module_sections::section code = {:?})",
                SectionKind::from(section_id)
            );

            // Consume appropriate section based on section id.
            sections.insert(section_id, self.section(section_id, &sections)?);
        }

        Ok(sections)
    }

    /// Gets the next section id and payload.
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
        self.validate_section_exists(section_id)?;

        Ok(section_id)
    }

    /// Gets the next module section
    pub fn section(&mut self, section_id: u8, sections: &HashMap<u8, Section>) -> ParserResult<Section> {
        verbose!("-> section! <-");
        let cursor = self.cursor;

        Ok(match section_id {
            0x00 => self.custom_section()?,
            0x01 => self.type_section()?,
            0x02 => self.import_section()?,
            0x03 => self.function_section()?,
            0x04 => self.table_section()?,
            0x05 => self.memory_section()?,
            0x06 => self.global_section(sections)?,
            0x07 => self.export_section()?,
            0x08 => self.start_section()?,
            0x09 => self.element_section(sections)?,
            // Code section needs `sections` to validate function calls and function return signature
            0x0A => self.code_section(sections)?,
            0x0B => self.data_section(sections)?,
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
    pub fn type_section(&mut self) -> ParserResult<Section> {
        verbose!("-> type_section! <-");
        let cursor = self.cursor;
        let mut func_types = vec![];

        // The length of the code section in bytes.
        let payload_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedPayloadLengthInTypeSection
        );

        verbose!("(type_section::payload_len = 0x{:x})", payload_len);

        // Get the count of type entries.
        let entry_count = get_value!(
            self.varuint32(),
            cursor,
            IncompleteTypeSection,
            MalformedEntryCountInTypeSection
        );

        verbose!("(type_section::entry_count = 0x{:x})", entry_count);

        // Consume the type entries.
        for _ in 0..entry_count {
            let tyid = get_value!(
                self.varint7(),
                cursor,
                EntriesDoNotMatchEntryCountInTypeSection,
                MalformedTypeInTypeSection
            );

            verbose!("(type_section::tyid = {:?})", tyid);

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
        let mut type_indices = vec![];

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
            // TODO: Validate type exists
            let type_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            type_indices.push(type_index);
        }

        verbose!("(function_section::type_indices = {:?})", type_indices);

        // TODO
        Ok(Section::Function(type_indices))
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
    pub fn global_section(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Section> {
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
            globals.push(self.global_entry(sections)?);
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
    pub fn element_section(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Section> {
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
            elements.push(self.element_entry(sections)?);
        }
        verbose!("(element_section::element_entries = {:?})", elements);

        // TODO
        Ok(Section::Element(elements))
    }

    /// TODO: TEST
    pub fn code_section(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Section> {
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
        for (index, _) in (0..body_count).enumerate() {
            // TODO: Validate function exists

            let signature = ir::get_signature_by_body_index(index, sections).unwrap();

            function_bodies.push(self.function_body(&signature, sections)?);
        }

        verbose!("(code_section::function_bodies = {:?})", function_bodies);

        // TODO
        Ok(Section::Code(function_bodies))
    }

    /// TODO: TEST
    pub fn data_section(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Section> {
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
            data.push(self.data_entry(sections)?);
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
        let type_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionImport,
            MalformedTypeIndexInFunctionImport
        );

        verbose!("(function_import::type_index = {:?})", type_index);

        Ok(ImportDesc::Function { type_index })
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

        let content_type = ValueType::from(get_value!(
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
    pub fn global_entry(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Global> {
        verbose!("-> global_entry! <-");
        let cursor = self.cursor;

        // Get content type
        let content_type = ValueType::from(get_value!(
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
        let instructions = self.instructions(sections, None)?;

        self.reset_instructions_state();

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
    pub fn element_entry(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Element> {
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
        let instructions = self.instructions(sections, None)?;

        self.reset_instructions_state();

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
    pub fn function_body(&mut self, signature: &FuncSignature, sections: &HashMap<u8, Section>) -> ParserResult<Function> {
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
        let instructions = self.instructions(sections, Some(&locals))?;

        // Get the amount of bytes consumed for locals and code.
        let diff = self.cursor - start_pos;

        // Check if the diff matches the body size
        if (body_size as usize) != diff {
            return Err(ParserError {
                kind: ErrorKind::BodySizeDoesNotMatchContentOfFunctionBody,
                cursor,
            });
        }

        //
        debug!("Signature = {:#?}", signature);

        // Validate return signature matches stack type
        self.validate_function_return_signature(signature.clone())?;

        self.reset_instructions_state();

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
        let local_type = ValueType::from(get_value!(
            self.value_type(),
            cursor,
            IncompleteLocalEntry,
            MalformedLocalTypeInLocalEntry
        ));

        verbose!("(local_entry::local_type = {:?})", local_type);

        Ok(Local { count, local_type })
    }

    /// TODO: TEST
    pub fn instructions(&mut self, sections: &HashMap<u8, Section>, locals: Option<&[Local]>) -> ParserResult<Vec<Operator>> {
        verbose!("-> instructions! <-");

        let cursor = self.cursor;
        let mut operators = vec![];

        loop {
            //
            let opcode = get_value!(
                self.uint8(),
                cursor,
                IncompleteExpression,
                MalformedOpcodeInExpression
            );

            verbose!("(instructions::opcode = 0x{:x})", opcode);

            // If opcode is an end byte. Break!
            // We can break because this end byte will always be function end byte
            // Each operator parser consumes its corresponding end byte
            if opcode == 0x0b {
                break;
            }

            operators.push(self.operator(opcode, sections, locals)?);

            // Increment operator_index
            self.operator_index += 1;

            debug!("Stack = {:?}\n", self.stack);
        }

        debug!("Operators = {:?}\n", operators);

        Ok(operators)
    }

    /// TODO: TEST
    pub fn operator(&mut self, opcode: u8, sections: &HashMap<u8, Section>, locals: Option<&[Local]>) -> ParserResult<Operator> {
        let cursor = self.cursor;
        // Operator parse functions defined in `operator/` folder

        // Blocks and functions return some type so the stack can't contain values
        // more than the block return types at the time of block exit.
        let operation = match opcode {
            // CONTROL FLOW
            0x00 => Operator::Unreachable,
            0x01 => Operator::Nop,
            0x02 => self.operator_block(sections, locals)?,
            0x03 => unimplemented!(),
            0x04 => unimplemented!(),
            0x05 => unimplemented!(),
            0x0b => Operator::End,
            0x0c => unimplemented!(),
            0x0d => unimplemented!(),
            0x0e => unimplemented!(),
            0x0f => unimplemented!(),
            // CALL
            0x10 => unimplemented!(),
            0x11 => unimplemented!(),
            // PARAMETRIC
            0x1A => self.operator_drop()?,
            0x1B => unimplemented!(), // The only ternary operator
            // VARIABLE ACCESS
            0x20 => self.operator_local_get(locals)?,
            0x21 => self.operator_local_set(locals)?,
            0x22 => self.operator_local_tee(locals)?,
            0x23 => self.operator_global_get(sections)?,
            0x24 => self.operator_global_set(sections)?,
            // MEMORY
            0x28 => self.operator_memory_load(I32, Operator::I32Load)?,
            0x29 => self.operator_memory_load(I64, Operator::I64Load)?,
            0x2a => self.operator_memory_load(F32, Operator::F32Load)?,
            0x2b => self.operator_memory_load(F64, Operator::F64Load)?,
            0x2c => self.operator_memory_load(I32, Operator::I32Load8Signed)?,
            0x2d => self.operator_memory_load(I32, Operator::I32Load8Unsigned)?,
            0x2e => self.operator_memory_load(I32, Operator::I32Load16Signed)?,
            0x2f => self.operator_memory_load(I32, Operator::I32Load16Unsigned)?,
            0x30 => self.operator_memory_load(I64, Operator::I64Load8Signed)?,
            0x31 => self.operator_memory_load(I64, Operator::I64Load8Unsigned)?,
            0x32 => self.operator_memory_load(I64, Operator::I64Load16Signed)?,
            0x33 => self.operator_memory_load(I64, Operator::I64Load16Unsigned)?,
            0x34 => self.operator_memory_load(I64, Operator::I64Load32Signed)?,
            0x35 => self.operator_memory_load(I64, Operator::I64Load32Unsigned)?,
            0x36 => self.operator_memory_store(I32, Operator::I32Store)?,
            0x37 => self.operator_memory_store(I64, Operator::I64Store)?,
            0x38 => self.operator_memory_store(F32, Operator::F32Store)?,
            0x39 => self.operator_memory_store(F64, Operator::F64Store)?,
            0x3a => self.operator_memory_store(I32, Operator::I32Store8)?,
            0x3b => self.operator_memory_store(I32, Operator::I32Store16)?,
            0x3c => self.operator_memory_store(I64, Operator::I64Store8)?,
            0x3d => self.operator_memory_store(I64, Operator::I64Store16)?,
            0x3e => self.operator_memory_store(I64, Operator::I64Store32)?,
            0x3f => self.operator_memory_size()?,
            0x40 => self.operator_memory_grow()?,
            // CONSTANTS
            0x41 => self.operator_i32_const()?,
            0x42 => self.operator_i64_const()?,
            0x43 => self.operator_f32_const()?,
            0x44 => self.operator_f64_const()?,
            // COMPARISONS
            0x45 => self.operator_numeric_1_arg(I32, I32, Operator::I32Eqz)?,
            0x46 => self.operator_numeric_2_args(I32, I32, Operator::I32Eq)?,
            0x47 => self.operator_numeric_2_args(I32, I32, Operator::I32Ne)?,
            0x48 => self.operator_numeric_2_args(I32, I32, Operator::I32LtSigned)?,
            0x49 => self.operator_numeric_2_args(I32, I32, Operator::I32LtUnsigned)?,
            0x4a => self.operator_numeric_2_args(I32, I32, Operator::I32GtSigned)?,
            0x4b => self.operator_numeric_2_args(I32, I32, Operator::I32GtSigned)?,
            0x4c => self.operator_numeric_2_args(I32, I32, Operator::I32LeSigned)?,
            0x4d => self.operator_numeric_2_args(I32, I32, Operator::I32LeUnsigned)?,
            0x4e => self.operator_numeric_2_args(I32, I32, Operator::I32GeSigned)?,
            0x4f => self.operator_numeric_2_args(I32, I32, Operator::I32GeSigned)?,
            0x50 => self.operator_numeric_1_arg(I64, I32, Operator::I64Eqz)?,
            0x51 => self.operator_numeric_2_args(I64, I32, Operator::I64Eq)?,
            0x52 => self.operator_numeric_2_args(I64, I32, Operator::I64Ne)?,
            0x53 => self.operator_numeric_2_args(I64, I32, Operator::I64LtSigned)?,
            0x54 => self.operator_numeric_2_args(I64, I32, Operator::I64LtUnsigned)?,
            0x55 => self.operator_numeric_2_args(I64, I32, Operator::I64GtSigned)?,
            0x56 => self.operator_numeric_2_args(I64, I32, Operator::I64GtSigned)?,
            0x57 => self.operator_numeric_2_args(I64, I32, Operator::I64LeSigned)?,
            0x58 => self.operator_numeric_2_args(I64, I32, Operator::I64LeUnsigned)?,
            0x59 => self.operator_numeric_2_args(I64, I32, Operator::I64GeSigned)?,
            0x5a => self.operator_numeric_2_args(I64, I32, Operator::I64GeSigned)?,
            0x5b => self.operator_numeric_2_args(F32, I32, Operator::F32Eq)?,
            0x5c => self.operator_numeric_2_args(F32, I32, Operator::F32Ne)?,
            0x5d => self.operator_numeric_2_args(F32, I32, Operator::F32Lt)?,
            0x5e => self.operator_numeric_2_args(F32, I32, Operator::F32Gt)?,
            0x5f => self.operator_numeric_2_args(F32, I32, Operator::F32Le)?,
            0x60 => self.operator_numeric_2_args(F32, I32, Operator::F32Ge)?,
            0x61 => self.operator_numeric_2_args(F64, I32, Operator::F64Eq)?,
            0x62 => self.operator_numeric_2_args(F64, I32, Operator::F64Ne)?,
            0x63 => self.operator_numeric_2_args(F64, I32, Operator::F64Lt)?,
            0x64 => self.operator_numeric_2_args(F64, I32, Operator::F64Gt)?,
            0x65 => self.operator_numeric_2_args(F64, I32, Operator::F64Le)?,
            0x66 => self.operator_numeric_2_args(F64, I32, Operator::F64Ge)?,
            // NUMERIC
            0x67 => self.operator_numeric_1_arg(I32, I32, Operator::I32Clz)?,
            0x68 => self.operator_numeric_1_arg(I32, I32, Operator::I32Ctz)?,
            0x69 => self.operator_numeric_1_arg(I32, I32, Operator::I32Popcnt)?,
            0x6a => self.operator_numeric_2_args(I32, I32, Operator::I32Add)?,
            0x6b => self.operator_numeric_2_args(I32, I32, Operator::I32Sub)?,
            0x6c => self.operator_numeric_2_args(I32, I32, Operator::I32Mul)?,
            0x6d => self.operator_numeric_2_args(I32, I32, Operator::I32DivSigned)?,
            0x6e => self.operator_numeric_2_args(I32, I32, Operator::I32DivUnsigned)?,
            0x6f => self.operator_numeric_2_args(I32, I32, Operator::I32RemSigned)?,
            0x70 => self.operator_numeric_2_args(I32, I32, Operator::I32RemUnsigned)?,
            0x71 => self.operator_numeric_2_args(I32, I32, Operator::I32And)?,
            0x72 => self.operator_numeric_2_args(I32, I32, Operator::I32Or)?,
            0x73 => self.operator_numeric_2_args(I32, I32, Operator::I32Xor)?,
            0x74 => self.operator_numeric_2_args(I32, I32, Operator::I32Shl)?,
            0x75 => self.operator_numeric_2_args(I32, I32, Operator::I32ShrSigned)?,
            0x76 => self.operator_numeric_2_args(I32, I32, Operator::I32ShrUnsigned)?,
            0x77 => self.operator_numeric_2_args(I32, I32, Operator::I32Rotl)?,
            0x78 => self.operator_numeric_2_args(I32, I32, Operator::I32Rotr)?,
            0x79 => self.operator_numeric_1_arg(I64, I64, Operator::I64Clz)?,
            0x7a => self.operator_numeric_1_arg(I64, I64, Operator::I64Ctz)?,
            0x7b => self.operator_numeric_1_arg(I64, I64, Operator::I64Popcnt)?,
            0x7c => self.operator_numeric_2_args(I64, I64, Operator::I64Add)?,
            0x7d => self.operator_numeric_2_args(I64, I64, Operator::I64Sub)?,
            0x7e => self.operator_numeric_2_args(I64, I64, Operator::I64Mul)?,
            0x7f => self.operator_numeric_2_args(I64, I64, Operator::I64DivSigned)?,
            0x80 => self.operator_numeric_2_args(I64, I64, Operator::I64DivUnsigned)?,
            0x81 => self.operator_numeric_2_args(I64, I64, Operator::I64RemSigned)?,
            0x82 => self.operator_numeric_2_args(I64, I64, Operator::I64RemUnsigned)?,
            0x83 => self.operator_numeric_2_args(I64, I64, Operator::I64And)?,
            0x84 => self.operator_numeric_2_args(I64, I64, Operator::I64Or)?,
            0x85 => self.operator_numeric_2_args(I64, I64, Operator::I64Xor)?,
            0x86 => self.operator_numeric_2_args(I64, I64, Operator::I64Shl)?,
            0x87 => self.operator_numeric_2_args(I64, I64, Operator::I64ShrSigned)?,
            0x88 => self.operator_numeric_2_args(I64, I64, Operator::I64ShrUnsigned)?,
            0x89 => self.operator_numeric_2_args(I64, I64, Operator::I64Rotl)?,
            0x8a => self.operator_numeric_2_args(I64, I64, Operator::I64Rotr)?,
            0x8b => self.operator_numeric_1_arg(F32, F32, Operator::F32Abs)?,
            0x8c => self.operator_numeric_1_arg(F32, F32, Operator::F32Neg)?,
            0x8d => self.operator_numeric_1_arg(F32, F32, Operator::F32Ceil)?,
            0x8e => self.operator_numeric_1_arg(F32, F32, Operator::F32Floor)?,
            0x8f => self.operator_numeric_1_arg(F32, F32, Operator::F32Trunc)?,
            0x90 => self.operator_numeric_1_arg(F32, F32, Operator::F32Nearest)?,
            0x91 => self.operator_numeric_1_arg(F32, F32, Operator::F32Sqrt)?,
            0x92 => self.operator_numeric_2_args(F32, F32, Operator::F32Add)?,
            0x93 => self.operator_numeric_2_args(F32, F32, Operator::F32Sub)?,
            0x94 => self.operator_numeric_2_args(F32, F32, Operator::F32Mul)?,
            0x95 => self.operator_numeric_2_args(F32, F32, Operator::F32Div)?,
            0x96 => self.operator_numeric_2_args(F32, F32, Operator::F32Min)?,
            0x97 => self.operator_numeric_2_args(F32, F32, Operator::F32Max)?,
            0x98 => self.operator_numeric_2_args(F32, F32, Operator::F32CopySign)?,
            0x99 => self.operator_numeric_1_arg(F64, F64, Operator::F64Abs)?,
            0x9a => self.operator_numeric_1_arg(F64, F64, Operator::F64Neg)?,
            0x9b => self.operator_numeric_1_arg(F64, F64, Operator::F64Ceil)?,
            0x9c => self.operator_numeric_1_arg(F64, F64, Operator::F64Floor)?,
            0x9d => self.operator_numeric_1_arg(F64, F64, Operator::F64Trunc)?,
            0x9e => self.operator_numeric_1_arg(F64, F64, Operator::F64Nearest)?,
            0x9f => self.operator_numeric_1_arg(F64, F64, Operator::F64Sqrt)?,
            0xa0 => self.operator_numeric_2_args(F64, F64, Operator::F64Add)?,
            0xa1 => self.operator_numeric_2_args(F64, F64, Operator::F64Sub)?,
            0xa2 => self.operator_numeric_2_args(F64, F64, Operator::F64Mul)?,
            0xa3 => self.operator_numeric_2_args(F64, F64, Operator::F64Div)?,
            0xa4 => self.operator_numeric_2_args(F64, F64, Operator::F64Min)?,
            0xa5 => self.operator_numeric_2_args(F64, F64, Operator::F64Max)?,
            0xa6 => self.operator_numeric_2_args(F64, F64, Operator::F64CopySign)?,
            // CONVERSIONS
            0xa7 => self.operator_numeric_1_arg(I64, I32, Operator::I32WrapI64)?,
            0xa8 => self.operator_numeric_1_arg(F32, I32, Operator::I32TruncF32Signed)?,
            0xa9 => self.operator_numeric_1_arg(F32, I32, Operator::I32TruncF32Unsigned)?,
            0xaa => self.operator_numeric_1_arg(F64, I32, Operator::I32TruncF64Signed)?,
            0xab => self.operator_numeric_1_arg(F64, I32, Operator::I32TruncF64Unsigned)?,
            0xac => self.operator_numeric_1_arg(I32, I64, Operator::I64ExtendI32Signed)?,
            0xad => self.operator_numeric_1_arg(I32, I64, Operator::I64ExtendI32Unsigned)?,
            0xae => self.operator_numeric_1_arg(F32, I64, Operator::I64TruncF32Signed)?,
            0xaf => self.operator_numeric_1_arg(F32, I64, Operator::I64TruncF32Unsigned)?,
            0xb0 => self.operator_numeric_1_arg(F64, I64, Operator::I64TruncF64Signed)?,
            0xb1 => self.operator_numeric_1_arg(F64, I64, Operator::I64TruncF64Unsigned)?,
            0xb2 => self.operator_numeric_1_arg(I32, F32, Operator::F32ConvertI32Signed)?,
            0xb3 => self.operator_numeric_1_arg(I32, F32, Operator::F32ConvertI32Unsigned)?,
            0xb4 => self.operator_numeric_1_arg(I64, F32, Operator::F32ConvertI64Signed)?,
            0xb5 => self.operator_numeric_1_arg(I64, F32, Operator::F32ConvertI64Unsigned)?,
            0xb6 => self.operator_numeric_1_arg(F64, F32, Operator::F32DemoteF64)?,
            0xb7 => self.operator_numeric_1_arg(I32, F64, Operator::F64ConvertI32Signed)?,
            0xb8 => self.operator_numeric_1_arg(I32, F64, Operator::F64ConvertI32Unsigned)?,
            0xb9 => self.operator_numeric_1_arg(I64, F64, Operator::F64ConvertI64Signed)?,
            0xba => self.operator_numeric_1_arg(I64, F64, Operator::F64ConvertI64Unsigned)?,
            0xbb => self.operator_numeric_1_arg(F32, F64, Operator::F64PromoteF32)?,
            // REINTERPRETATIONS
            0xbc => self.operator_numeric_1_arg(F32, I32, Operator::I32ReinterpretF32)?,
            0xbd => self.operator_numeric_1_arg(F64, I64, Operator::I64ReinterpretF64)?,
            0xbe => self.operator_numeric_1_arg(I32, F32, Operator::F32ReinterpretI32)?,
            0xbf => self.operator_numeric_1_arg(I64, F64, Operator::F64ReinterpretI64)?,
            _ => {
                return Err(ParserError {
                    kind: ErrorKind::UnsupportedOperator,
                    cursor,
                });
            }
        };

        Ok(operation)
    }

    /******** DATA ********/

    /// TODO: TEST
    pub fn data_entry(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Data> {
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
        let instructions = self.instructions(sections, None)?;

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
    pub fn limits(&mut self) -> ParserResult<(u32, Option<u32>)> {
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
            let param_type = ValueType::from(get_value!(
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
            let return_type = ValueType::from(get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            ));

            returns.push(return_type);
        }
        verbose!("(func_type::return_types = {:?})", returns);

        Ok(Type::Func(FuncSignature { params, returns }))
    }

    /// TODO: TEST
    pub fn value_type(&mut self) -> Result<i8, ErrorKind> {
        // verbose!("-> value_type! <-");
        let value = self.varint7()?;

        // i32, i64, f32, f64
        match value {
            -0x04...-0x01 => Ok(value),
            _ => Err(ErrorKind::InvalidValueType),
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

    ///
    pub fn reset_instructions_state(&mut self) {
        // Reset stack and operator_index
        self.stack = Stack::new();
        self.operator_index = 0;
    }

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

    /// Consumes 8 bytes that represent a 64-bit unsigned integer
    pub fn uint64(&mut self) -> Result<u64, ErrorKind> {
        if let Some(bytes) = self.eat_bytes(8) {
            let mut shift = 0;
            let mut result = 0;
            for byte in bytes {
                result |= (*byte as u64) << shift;
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
