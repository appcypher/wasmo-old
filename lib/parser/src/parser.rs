#[macro_use]
use wasmlite_utils::*;
use crate::{
    macros,
    errors::ParserError,
    kinds::ErrorKind,
    utils::{int_to_section, int_to_type},
    validation::validate_section_exists,
};

// TODO
//  - Improve error reporting.

pub type ParserResult = Result<(), ParserError>;

/// A WebAssembly module eager parser.
///
/// The error handling mechanism
/// - Errors start at the primitive read functions like (varuint or uint8) and propagate up the call stack with each enclosing function
///   fixing the error message to provide more context.
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    code: &'a [u8], // The wasm binary to parse
    cursor: usize,  // Used to track the current byte position as the parser advances.
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
    /// Generates the `module` object by calling functions
    /// that parse a wasm module.
    pub fn module(&mut self) -> ParserResult {
        debug!("-> module! <-");

        // Consume preamble.
        self.module_preamble()?;

        self.module_sections().unwrap(); // Optional

        Ok(())
    }

    /// TODO: TEST
    /// Checks if the following bytes are expected
    /// wasm preamble bytes.
    pub fn module_preamble(&mut self) -> ParserResult {
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
    pub fn module_sections(&mut self) -> ParserResult {
        debug!("-> module_sections! <-");

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

            debug!(
                "(module_sections::section code = {:?})",
                int_to_section(section_id)
            );

            // Consume appropriate section based on section id.
            match section_id {
                0x00 => self.custom_section()?,
                0x01 => self.type_section()?,
                0x02 => self.import_section()?,
                0x03 => self.function_section()?,
                0x0A => self.code_section()?,
                _ => {
                    return Err(ParserError {
                        kind: ErrorKind::UnsupportedSection,
                        cursor,
                    });
                }
            };
        }
        Ok(())
    }

    /// Gets the next section id and payload.
    /// Needed by reader
    pub fn module_section_id(&mut self) -> ParserResult {
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

        Ok(())
    }

    /******** SECTIONS ********/

    /// TODO: TEST
    /// TODO: Name section and linking section.
    pub fn custom_section(&mut self) -> ParserResult {
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

        Ok(())
    }

    /// TODO: TEST
    pub fn type_section(&mut self) -> ParserResult {
        debug!("-> type_section! <-");
        let cursor = self.cursor;

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

            // Type must a be a func type.
            match type_id {
                -0x20 => self.func_type()?,
                _ => {
                    return Err(ParserError {
                        kind: ErrorKind::UnsupportedTypeInTypeSection,
                        cursor,
                    });
                }
            };
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn import_section(&mut self) -> ParserResult {
        debug!("-> import_section! <-");
        let cursor = self.cursor;

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
            self.import_entry()?;
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn function_section(&mut self) -> ParserResult {
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
            // TODO: LLVM module construction.
            // Index into the type section.
            let type_index = get_value!(
                self.varuint32(),
                cursor,
                IncompleteFunctionSection,
                MalformedEntryInFunctionSection
            );

            debug!("(function_section::type_index = 0x{:x})", type_index);
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn code_section(&mut self) -> ParserResult {
        debug!("-> code_section! <-");
        let cursor = self.cursor;

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
            self.function_body()?;
        }

        Ok(())
    }

    /******** IMPORTS ********/

    /// TODO: TEST
    pub fn import_entry(&mut self) -> ParserResult {
        debug!("-> import_entry! <-");
        let cursor = self.cursor;

        //
        let module_len = get_value!(
            self.varuint32(),
            cursor,
            IncompleteImportEntry,
            MalformedModuleLengthInImportEntry
        );

        debug!("(import_entry::module_len = 0x{:x})", module_len);

        {
            // TODO: Validate UTF-8
            let _module_str = match self.eat_bytes(module_len as _) {
                Some(value) => value,
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteImportEntry,
                        cursor,
                    });
                }
            };

            debug!(
                "import_entry::_module_str = {:?}",
                std::str::from_utf8(_module_str)
            );
        }

        //
        let field_len = get_value!(
            self.varint32(),
            cursor,
            IncompleteImportEntry,
            MalformedFieldLengthInImportEntry
        );

        debug!("(import_entry::field_len = 0x{:x})", field_len);

        {
            // TODO: Validate UTF-8
            let _field_str = match self.eat_bytes(field_len as _) {
                Some(value) => value,
                None => {
                    return Err(ParserError {
                        kind: ErrorKind::IncompleteImportEntry,
                        cursor,
                    });
                }
            };

            debug!(
                "(import_entry::_field_str = {:?})",
                std::str::from_utf8(_field_str)
            );
        }

        let external_kind = get_value!(
            self.external_kind(),
            cursor,
            IncompleteImportEntry,
            MalformedImportTypeInImportEntry
        );

        match external_kind {
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
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn function_import(&mut self) -> ParserResult {
        debug!("-> function_import! <-");
        let cursor = self.cursor;

        // TODO: LLVM module construction
        let type_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteFunctionImport,
            MalformedTypeIndexInFunctionImport
        );

        debug!("(function_import::type_index = {:?})", type_index);

        Ok(())
    }

    /// TODO: TEST
    pub fn table_import(&mut self) -> ParserResult {
        debug!("-> table_import! <-");
        let cursor = self.cursor;

        // TODO: LLVM module construction
        let element_type = match self.varint7() {
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
        };

        debug!("(table_import::element_type = {:?})", int_to_type(element_type));

        //
        let (initial, maximum) = match self.resizable_limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                // TODO: LLVM module construction
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteTableImport,
                    ErrorKind::MalformedFlagsInResizableLimits => {
                        ErrorKind::MalformedFlagsInTableImport
                    }
                    ErrorKind::MalformedInitialInResizableLimits => {
                        ErrorKind::MalformedInitialInTableImport
                    }
                    ErrorKind::MalformedMaximumInResizableLimits => {
                        ErrorKind::MalformedMaximumInTableImport
                    }
                    _ => ErrorKind::MalformedResizableLimitInTableImport,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        debug!("(table_import::initial = {:?})", initial);

        debug!("(table_import::maximum = {:?})", maximum);

        Ok(())
    }

    /// TODO: TEST
    pub fn memory_import(&mut self) -> ParserResult {
        debug!("-> memory_import! <-");
        let cursor = self.cursor;

        //
        let (initial, maximum) = match self.resizable_limits() {
            Ok(value) => value,
            Err(ParserError { kind, .. }) => {
                // TODO: LLVM module construction
                let err = match kind {
                    ErrorKind::BufferEndReached => ErrorKind::IncompleteMemoryImport,
                    ErrorKind::MalformedFlagsInResizableLimits => {
                        ErrorKind::MalformedFlagsInMemoryImport
                    }
                    ErrorKind::MalformedInitialInResizableLimits => {
                        ErrorKind::MalformedInitialInMemoryImport
                    }
                    ErrorKind::MalformedMaximumInResizableLimits => {
                        ErrorKind::MalformedMaximumInMemoryImport
                    }
                    _ => ErrorKind::MalformedResizableLimitInMemoryImport,
                };

                return Err(ParserError { kind, cursor });
            }
        };

        debug!("(memory_import::initial = {:?})", initial);

        debug!("(memory_import::maximum = {:?})", maximum);

        Ok(())
    }

    /// TODO: TEST
    pub fn global_import(&mut self) -> ParserResult {
        debug!("-> global_import! <-");
        let cursor = self.cursor;

        // TODO: LLVM module construction
        let content_type = get_value!(
            self.value_type(),
            cursor,
            IncompleteGlobalImport,
            MalformedContentTypeInGlobalImport
        );

        debug!("(global_import::content_type = {:?})", content_type);

        // TODO: LLVM module construction
        let mutability = get_value!(
            self.varuint1(),
            cursor,
            IncompleteGlobalImport,
            MalformedMutabilityInGlobalImport
        );

        debug!("(global_import::mutability = {:?})", mutability);

        Ok(())
    }

    /******** CODE ********/

    /// TODO: TEST
    /// Each function body corresponds to the functions declared in the function section.
    pub fn function_body(&mut self) -> ParserResult {
        debug!("-> function_body! <-");
        let cursor = self.cursor;

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
            self.local_entry()?;
        }

        // Get the amount of bytes consumed for locals.
        let diff = self.cursor - start_pos;

        // Consume code.
        for _ in (diff + 1)..(body_size as _) {
            self.operator()?;
        }

        // Get end byte.
        let end_byte = get_end_byte!(
            self.varint32(),
            cursor,
            IncompleteFunctionBody,
            MalformedEndByteInFunctionBody
        );

        debug!("(function_body::end_byte = 0x{:x})", end_byte);

        Ok(())
    }

    /// TODO: TEST
    pub fn local_entry(&mut self) -> ParserResult {
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
        let local_type = get_value!(
            self.value_type(),
            cursor,
            IncompleteTypeSection,
            MalformedTypeInLocalEntry
        );

        debug!("(function_body::local_type = {:?})", int_to_type(local_type));

        Ok(())
    }

    /// TODO: TEST
    pub fn operator(&mut self) -> ParserResult {
        debug!("-> instructions! <-");
        let cursor = self.cursor;

        let op_code = get_value!(
            self.uint8(),
            cursor,
            IncompleteFunctionBody,
            MalformedBodySizeInFunctionBody
        );

        // Dispatch to the right
        match op_code {
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

        Ok(())
    }

    /********* OPERATORS *********/
    /// TODO: TEST
    pub fn operator_i32_add(&mut self) -> ParserResult {
        debug!("-> operator_i32_add! <-");
        let cursor = self.cursor;

        // TODO

        Ok(())
    }

    /******** TYPES ********/

    /// TODO: TEST
    pub fn resizable_limits(&mut self) -> Result<(u32, Option<u32>), ParserError> {
        // debug!("-> resizable_limits! <-");
        let cursor = self.cursor;

        // TODO: LLVM module construction
        let flags = get_value!(
            self.varuint1(),
            cursor,
            IncompleteResizableLimits,
            MalformedFlagsInResizableLimits
        );

        // TODO: LLVM module construction
        let initial = get_value!(
            self.varuint32(),
            cursor,
            IncompleteResizableLimits,
            MalformedInitialInResizableLimits
        );

        //
        let maximum = if flags {
            match self.varuint32() {
                Ok(value) => Some(value),
                Err(error) => {
                    if error == ErrorKind::BufferEndReached {
                        return Err(ParserError {
                            kind: ErrorKind::IncompleteResizableLimits,
                            cursor,
                        });
                    } else {
                        return Err(ParserError {
                            kind: ErrorKind::MalformedMaximumInResizableLimits,
                            cursor,
                        });
                    }
                }
            }
        } else {
            None
        };

        Ok((initial, maximum))
    }

    /// TODO: TEST
    pub fn func_type(&mut self) -> ParserResult {
        // debug!("-> func_type! <-");
        let cursor = self.cursor;

        //
        let param_count = get_value!(
            self.varint32(),
            cursor,
            IncompleteFunctionType,
            MalformedParamCountInFunctionType
        );

        debug!("(func_type::param_count = 0x{:x})", param_count);

        //
        for _ in 0..param_count {
            // TODO: LLVM module construction
            let param_type = get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedParamTypeInFunctionType
            );

            debug!("(func_type::param_type = {:?})", int_to_type(param_type));
        }

        //
        let return_count = get_value!(
            self.varuint1(),
            cursor,
            IncompleteFunctionType,
            MalformedReturnCountInFunctionType
        );

        debug!("(func_type::return_count = {:?})", return_count);

        if return_count {
            // TODO: LLVM module construction
            let return_type = get_value!(
                self.value_type(),
                cursor,
                IncompleteFunctionType,
                MalformedReturnTypeInFunctionType
            );

            debug!("(func_type::return_type = {:?})", int_to_type(return_type));
        }

        Ok(())
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
