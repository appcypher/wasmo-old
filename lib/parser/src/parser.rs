#[macro_use]
use wasmlite_utils::*;

use crate::{errors::ParserError, kinds::ErrorKind, macros, utils::*, };
use wasmlite_llvm::module::Module;

// TODO
//  - Improve error reporting.

type ParserResult = Result<(), ParserError>;

#[derive(Debug, Clone)]
/// A single-pass codegen parser.
/// Generates a Module as it deserializes a wasm binary.
pub struct Parser<'a> {
    code: &'a Vec<u8>, // The wasm binary to parse
    cursor: usize,     // Used to track the current byte position as the parser advances.
    module: Module,    // The generated module
}

/// Contains the implementation of parser
impl<'a> Parser<'a> {
    /// Creates new parser
    pub fn new(code: &'a Vec<u8>) -> Self {
        Parser {
            code,
            cursor: 0, // cursor starts at first byte
            module: Module::new(),
        }
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
                if value != 0x6d736100 {
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

        // Holds the section ids that have been consumed.
        // Section types cannot occur more than once.
        let mut sections_consumed = vec![];

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

            // Check if section has already been consumed.
            if sections_consumed.contains(&section_id) {
                return Err(ParserError {
                    kind: ErrorKind::SectionAlreadyDefined,
                    cursor,
                });
            } else {
                sections_consumed.push(section_id);
            }

            debug!("(module_sections::section code = {:?})", to_section(section_id));

            // Consume appropriate section depending on section id.
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

        debug!("(table_import::element_type = {:?})", to_type(element_type));

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
            IncompleteTypeSection,
            MalformedBodySizeInFunctionBody
        );

        debug!("(function_body::body_size = 0x{:x})", body_size);

        // Start position of body bytes.
        let start_pos = self.cursor;

        // Get count of locals.
        let local_count = get_value!(
            self.varint32(),
            cursor,
            IncompleteTypeSection,
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
        for _ in (diff+1)..(body_size as _) {
            self.instructions()?;
        }

        // Get end byte.
        let end_byte = get_value!(
            self.varint32(),
            cursor,
            IncompleteTypeSection,
            MalformedEndByteInFunctionBody
        );

        debug!("(function_body::end_byte = 0x{:x})", end_byte);

        // Check if end byte is correct.
        if end_byte != 0x0b {
            return Err(ParserError {
                kind: ErrorKind::MalformedEndByteInFunctionBody,
                cursor,
            });
        }

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

        debug!("(function_body::local_type = {:?})",  to_type(local_type));

        Ok(())
    }

    /// TODO: TEST
    pub fn instructions(&mut self) -> ParserResult {
        debug!("-> instructions! <-");
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
        let mut maximum = None;

        //
        if flags {
            // TODO: LLVM module construction
            maximum = match self.varuint32() {
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
            };
        }

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

            debug!("(func_type::param_type = {:?})",  to_type(param_type));
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

            debug!("(func_type::return_type = {:?})",  to_type(return_type));
        }

        Ok(())
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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
            let mut result = byte;
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

// pub fn compile(source: Vec<u8>) -> Module {
// }
