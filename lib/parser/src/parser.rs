#[macro_use]
use wasmlite_utils::*;

use wasmlite_llvm::module::Module;
use crate::{
    errors::ParserError,
};

#[derive(Debug, Clone)]
/// A single-pass codegen parser.
/// Generates a Module as it deserializes a wasm binary.
pub struct Parser<'a> {
    code: &'a Vec<u8>, // The wasm binary to parse
    cursor: usize, // Used to track the current byte position as the parser advances.
    module: Module, // The generated module
}

/// Contains the implementation of parser
impl <'a> Parser<'a> {
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
    pub fn module(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> module! <-");

        // Consume preamble. Panic if it returns an error.
        self.module_preamble().unwrap();

        self.module_sections().unwrap(); // Optional

        Ok(())
    }

    /// TODO: TEST
    /// Checks if the following bytes are expected
    /// wasm preamble bytes.
    pub fn module_preamble(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> module_preamble! <-");
        let start_position = self.cursor;

        // Consume magic number.
        let magic_no = match self.uint32() {
            Ok(value) => {
                // Magic number must be `\0asm`
                if value != 0x6d736100 {
                    return Err((ParserError::InvalidMagicNumber, start_position));
                }
                value
            },
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompletePreamble, start_position));
                } else {
                    return Err((ParserError::InvalidVersionNumber, start_position));
                }
            },
        };

        debug!("module_preamble::magic_no = 0x{:08x}", magic_no);

        // Consume version number.
        let version_no = match self.uint32() {
            Ok(value) => {
                // Only version 0x01 supported for now.
                if value != 0x1 {
                    return Err((ParserError::MalformedVersionNumber, start_position));
                }
                value
            },
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompletePreamble, start_position));
                } else {
                    return Err((ParserError::MalformedVersionNumber, start_position));
                }
            },
        };

        debug!("module_preamble::version_no = 0x{:08x}", version_no);

        Ok(())
    }

    /// TODO: TEST
    pub fn module_sections(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> module_sections! <-");

        //
        let mut sections_consumed = vec![];

        //
        loop {
            let start_position = self.cursor;
            //
            let section_id = match self.varuint7() {
                Ok(value) => value,
                Err(error) => {
                    //
                    if error == ParserError::BufferEndReached {
                        break;
                    } else {
                        return Err((ParserError::MalformedSectionId, start_position));
                    }
                },
            };

            //
            if sections_consumed.contains(&section_id) {
                return Err((ParserError::SectionAlreadyDefined, start_position));
            } else {
                sections_consumed.push(section_id);
            }

            //
            match section_id {
                0x00 => self.custom_section()?,
                0x01 => self.type_section()?,
                0x02 => self.import_section()?,
                _ => {
                    return Err((ParserError::UnsupportedSection, start_position));
                },
            };
        }
        Ok(())
    }

    /// TODO: TEST
    pub fn custom_section(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> custom_section! <-");
        let start_position = self.cursor;

        //
        let payload_len = match self.varint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteCustomSection, start_position));
                } else {
                    return Err((ParserError::MalformedPayloadLengthInCustomSection, start_position));
                }
            }
        };

        //
        let name_len = match self.varint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteCustomSection, start_position));
                } else {
                    return Err((ParserError::MalformedEntryCountInTypeSection, start_position));
                }
            }
        };

        {
            // TODO: Validate UTF-8
            // Skip payload bytes
            let _name = match self.eat_bytes(name_len as _) {
                Some(value) => value,
                None => {
                    return Err((ParserError::IncompleteCustomSection, start_position));
                }
            };
        }

        // Skip payload bytes
        let _payload_data = match self.eat_bytes(payload_len as _) {
            Some(value) => value,
            None => {
                return Err((ParserError::IncompleteCustomSection, start_position));
            }
        };

        Ok(())
    }

    /// TODO: TEST
    pub fn type_section(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> type_section! <-");
        let start_position = self.cursor;

        //
        let payload_len = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedPayloadLengthInTypeSection, start_position));
                }
            }
        };

        debug!("type_section::payload_len = 0x{:x}", payload_len);

        //
        let entry_count = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedEntryCountInTypeSection, start_position));
                }
            }
        };

        debug!("type_section::entry_count = 0x{:x}", entry_count);

        //
        for i in 0..entry_count {
            let type_id = match self.varint7() {
                Ok(value) => value,
                Err(error) => {
                    //
                    if error == ParserError::BufferEndReached {
                        return Err((ParserError::EntriesDoNotMatchEntryCountInTypeSection, start_position));
                    } else {
                        return Err((ParserError::MalformedTypeInTypeSection, start_position));
                    }
                },
            };

            debug!("type_section::type_id = {:?}", type_id);

            match type_id {
                -0x20 => self.func_type()?,
                _ => {
                    return Err((ParserError::UnsupportedTypeInTypeSection, start_position));
                },
            };
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn import_section(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> import_section! <-");
        let start_position = self.cursor;

        //
        let payload_len = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedPayloadLengthInTypeSection, start_position));
                }
            }
        };
        debug!("import_section::payload_len = 0x{:x}", payload_len);

        //
        let entry_count = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedEntryCountInTypeSection, start_position));
                }
            }
        };

        debug!("import_section::entry_count = 0x{:x}", entry_count);

        //
        for i in 0..entry_count {
            self.import_entry()?;
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn function_section(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> import_section! <-");
        let start_position = self.cursor;

        //
        let payload_len = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedPayloadLengthInTypeSection, start_position));
                }
            }
        };
        debug!("import_section::payload_len = 0x{:x}", payload_len);

        //
        let entry_count = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTypeSection, start_position));
                } else {
                    return Err((ParserError::MalformedEntryCountInTypeSection, start_position));
                }
            }
        };

        debug!("import_section::entry_count = 0x{:x}", entry_count);

        //
        for i in 0..entry_count {
            self.import_entry()?;
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn import_entry(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> import_entry! <-");
        let start_position = self.cursor;

        //
        let module_len = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteImportEntry, start_position));
                } else {
                    return Err((ParserError::MalformedModuleLengthInImportEntry, start_position));
                }
            }
        };

        debug!("import_entry::module_len = 0x{:x}", module_len);

        {
            // TODO: Validate UTF-8
            let _module_str = match self.eat_bytes(module_len as _) {
                Some(value) => value,
                None => {
                    return Err((ParserError::IncompleteImportEntry, start_position));
                }
            };

            debug!("import_entry::_module_str = {:?}", std::str::from_utf8(_module_str));
        }

        //
        let field_len = match self.varint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteImportEntry, start_position));
                } else {
                    return Err((ParserError::MalformedFieldLengthInImportEntry, start_position));
                }
            }
        };

        debug!("import_entry::field_len = 0x{:x}", field_len);

        {
            // TODO: Validate UTF-8
            let _field_str = match self.eat_bytes(field_len as _) {
                Some(value) => value,
                None => {
                    return Err((ParserError::IncompleteImportEntry, start_position));
                }
            };

            debug!("import_entry::_field_str = {:?}", std::str::from_utf8(_field_str));
        }

        let external_kind =  match self.external_kind() {
            Ok(value) => value,
            Err(error) => {
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteImportEntry, start_position));
                } else {
                    return Err((ParserError::MalformedImportTypeInImportEntry, start_position));
                }
            }
        };

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
                return Err((ParserError::InvalidImportTypeInImportEntry, start_position));
            }
        }

        Ok(())
    }

    /// TODO: TEST
    pub fn function_import(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> function_import! <-");
        let start_position = self.cursor;

        // TODO: LLVM module construction
        let type_index = match self.varuint32() {
            // TODO. Validate type_index
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteFunctionImport, start_position));
                } else {
                    return Err((ParserError::MalformedTypeIndexInFunctionImport, start_position));
                }
            }
        };

        debug!("function_import::type_index = {:?}", type_index);

        Ok(())
    }

    /// TODO: TEST
    pub fn table_import(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> table_import! <-");
        let start_position = self.cursor;

        // TODO: LLVM module construction
        let element_type = match self.varint7() {
            Ok(value) => {
                // Must be anyfunc
                if value != -0x10 {
                    return Err((ParserError::MalformedElementTypeInTableImport, start_position));
                }
                value
            },
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteTableImport, start_position));
                } else {
                    return Err((ParserError::MalformedElementTypeInTableImport, start_position));
                }
            }
        };

        debug!("table_import::element_type = {:?}", element_type);

        //
        let (initial, maximum) = match self.resizable_limits() {
            Ok(value) => value,
            Err(error) => {
                // TODO: LLVM module construction
                let err = match error {
                    (ParserError::BufferEndReached, _) => ParserError::IncompleteTableImport,
                    (ParserError::MalformedFlagsInResizableLimits, _) => ParserError::MalformedFlagsInTableImport,
                    (ParserError::MalformedInitialInResizableLimits, _) => ParserError::MalformedInitialInTableImport,
                    (ParserError::MalformedMaximumInResizableLimits, _) => ParserError::MalformedMaximumInTableImport,
                    (_, _) => ParserError::MalformedResizableLimitInTableImport,
                };

                return Err((err, start_position))
            }
        };

        debug!("table_import::initial = {:?}", initial);

        debug!("table_import::maximum = {:?}", maximum);

        Ok(())
    }

    /// TODO: TEST
    pub fn memory_import(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> memory_import! <-");
        let start_position = self.cursor;

        //
        let (initial, maximum) = match self.resizable_limits() {
            Ok(value) => value,
            Err(error) => {
                /// TODO: LLVM module construction
                let err = match error {
                    (ParserError::BufferEndReached, _) => ParserError::IncompleteMemoryImport,
                    (ParserError::MalformedFlagsInResizableLimits, _) => ParserError::MalformedFlagsInMemoryImport,
                    (ParserError::MalformedInitialInResizableLimits, _) => ParserError::MalformedInitialInMemoryImport,
                    (ParserError::MalformedMaximumInResizableLimits, _) => ParserError::MalformedMaximumInMemoryImport,
                    (_, _) => ParserError::MalformedResizableLimitInMemoryImport,
                };

                return Err((err, start_position))
            }
        };

        debug!("memory_import::initial = {:?}", initial);

        debug!("memory_import::maximum = {:?}", maximum);

        Ok(())
    }

    /// TODO: TEST
    pub fn global_import(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> global_import! <-");
        let start_position = self.cursor;

        // TODO: LLVM module construction
        let content_type = match self.value_type() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteGlobalImport, start_position));
                } else {
                    return Err((ParserError::MalformedContentTypeInGlobalImport, start_position));
                }
            }
        };

        debug!("global_import::content_type = {:?}", content_type);

        // TODO: LLVM module construction
        let mutability = match self.varuint1() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteGlobalImport, start_position));
                } else {
                    return Err((ParserError::MalformedMutabilityInGlobalImport, start_position));
                }
            }
        };

        debug!("global_import::mutability = {:?}", mutability);

        Ok(())
    }

    /// TODO: TEST
    pub fn resizable_limits(&mut self) -> Result<(u32, Option<u32>), (ParserError, usize)> {
        debug!("-> resizable_limits! <-");
        let start_position = self.cursor;

        /// TODO: LLVM module construction
        let flags = match self.varuint1() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteResizableLimits, start_position));
                } else {
                    return Err((ParserError::MalformedFlagsInResizableLimits, start_position));
                }
            }
        };

        /// TODO: LLVM module construction
        let initial = match self.varuint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteResizableLimits, start_position));
                } else {
                    return Err((ParserError::MalformedInitialInResizableLimits, start_position));
                }
            }
        };

        //
        let mut maximum = None;

        //
        if flags {
            // TODO: LLVM module construction
            maximum = match self.varuint32() {
                Ok(value) => Some(value),
                Err(error) => {
                    //
                    if error == ParserError::BufferEndReached {
                        return Err((ParserError::IncompleteResizableLimits, start_position));
                    } else {
                        return Err((ParserError::MalformedMaximumInResizableLimits, start_position));
                    }
                }
            };
        }

        Ok((initial, maximum))
    }

    /// TODO: TEST
    pub fn func_type(&mut self) -> Result<(), (ParserError, usize)> {
        debug!("-> func_type! <-");
        let start_position = self.cursor;

        //
        let param_count = match self.varint32() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteFunctionType, start_position));
                } else {
                    return Err((ParserError::MalformedParamCountInFunctionType, start_position));
                }
            }
        };

        debug!("func_type::param_count = 0x{:x}", param_count);

        //
        for i in 0..param_count {
            /// TODO: LLVM module construction
            let param_type = match self.value_type() {
                Ok(value) => value,
                Err(error) => {
                    if error == ParserError::BufferEndReached {
                        return Err((ParserError::IncompleteFunctionType, start_position));
                    } else {
                        return Err((ParserError::MalformedParamTypeInFunctionType, start_position));
                    }
                }
            };

            debug!("func_type::param_type = {:?}", param_type);
        }

        //
        let return_count = match self.varuint1() {
            Ok(value) => value,
            Err(error) => {
                //
                if error == ParserError::BufferEndReached {
                    return Err((ParserError::IncompleteFunctionType, start_position));
                } else {
                    return Err((ParserError::MalformedReturnCountInFunctionType, start_position));
                }
            }
        };

        debug!("func_type::return_count = {:?}", return_count);

        if return_count {
            /// TODO: LLVM module construction
            let return_type = match self.value_type() {
                Ok(value) => value,
                Err(error) => {
                    if error == ParserError::BufferEndReached {
                        return Err((ParserError::IncompleteFunctionType, start_position));
                    } else {
                        return Err((ParserError::MalformedReturnTypeInFunctionType, start_position));
                    }
                }
            };

            debug!("func_type::return_type = {:?}", return_type);
        }

        Ok(())
    }

    #[inline]
    /// TODO: TEST
    pub fn value_type(&mut self) -> Result<i8, ParserError> {
        debug!("-> value_type! <-");

        let value = self.varint7()?;

        // i32, i64, f32, f64
        if value == -0x01 || value == -0x02 || value == -0x03 || value == -0x04 {
            Ok(value as _)
        } else {
            Err(ParserError::InvalidValueType)
        }
    }

    #[inline]
    /// TODO: TEST
    pub fn external_kind(&mut self) -> Result<u8, ParserError> {
        debug!("-> external_kind! <-");

        let value = self.uint8()?;

        // function_import, table_import, memory_imoort, global_import
        if value == 0x00 || value == 0x01 || value == 0x02 || value == 0x03 {
            Ok(value as _)
        } else {
            Err(ParserError::InvalidImportType)
        }
    }

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
    pub fn uint8(&mut self) -> Result<u8, ParserError> {
        if let Some(byte) = self.eat_byte() {
            return Ok(byte);
        }
        Err(ParserError::BufferEndReached)
    }

    /// Consumes 2 bytes that represent a 16-bit unsigned integer
    pub fn uint16(&mut self) -> Result<u16, ParserError> {
        if let Some(bytes) = self.eat_bytes(2) {
            let mut shift = 0;
            let mut result = 0;
            for byte in bytes {
                result |= (*byte as u16) << shift;
                shift += 8;
            }
            return Ok(result);
        }
        Err(ParserError::BufferEndReached)
    }

    /// Consumes 4 bytes that represent a 32-bit unsigned integer
    pub fn uint32(&mut self) -> Result<u32, ParserError> {
        if let Some(bytes) = self.eat_bytes(4) {
            let mut shift = 0;
            let mut result = 0;
            for byte in bytes {
                result |= (*byte as u32) << shift;
                shift += 8;
            }
            return Ok(result);
        }
        Err(ParserError::BufferEndReached)
    }

    /// Consumes a byte that represents a 1-bit LEB128 unsigned integer encoding
    pub fn varuint1(&mut self) -> Result<bool, ParserError> {
        if let Some(byte) = self.eat_byte() {
            return match byte {
                1 => Ok(true),
                0 => Ok(false),
                _ => Err(ParserError::MalformedVaruint1),
            };
        }
        // We expect the if statement to return an Ok result. If it doesn't
        // then we are trying to read more than 1 byte, which is malformed for a varuint1
        Err(ParserError::BufferEndReached)
    }

    /// Consumes a byte that represents a 7-bit LEB128 unsigned integer encoding
    pub fn varuint7(&mut self) -> Result<u8, ParserError> {
        if let Some(byte) = self.eat_byte() {
            let mut result = byte;
            // Check if msb is unset.
            if result & 0b1000_0000 != 0 {
                return Err(ParserError::MalformedVaruint7);
            }
            return Ok(result);
        }
        // We expect the if statement to return an Ok result. If it doesn't
        // then we are trying to read more than 1 byte, which is malformed for a varuint7
        Err(ParserError::BufferEndReached)
    }

    /// Consumes 1-5 bytes that represent a 32-bit LEB128 unsigned integer encoding
    pub fn varuint32(&mut self) -> Result<u32, ParserError> {
        // debug!("= varuint32! <-");
        let mut result = 0;
        let mut shift = 0;
        while shift < 35 {
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ParserError::BufferEndReached),
            };
            // debug!("count = {}, byte = 0b{:08b}", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b10000000) as u32) << shift;
            result |= value;
            // Return if any of the bytes has an unset msb
            if byte & 0b1000_0000 == 0 {
                return Ok(result);
            }
            shift += 7;
        }
        // We expect the loop to terminate early and return an Ok result. If it doesn't
        // then we are trying to read more than 5 bytes, which is malformed for a varuint32
        Err(ParserError::MalformedVaruint32)
    }

    /// Consumes a byte that represents a 7-bit LEB128 signed integer encoding
    pub fn varint7(&mut self) -> Result<i8, ParserError> {
        if let Some(byte) = self.eat_byte() {
            let mut result = byte;
            // Check if msb is unset.
            if result & 0b1000_0000 != 0 {
                return Err(ParserError::MalformedVarint7);
            }
            // If the 7-bit value is signed, extend the sign.
		    if result & 0b0100_0000 == 0b0100_0000 {
                result |= 0b1000_0000;
            }
            return Ok(result as i8);
        }

        Err(ParserError::BufferEndReached)
    }

    /// Consumes 1-5 bytes that represent a 32-bit LEB128 signed integer encoding
    pub fn varint32(&mut self) -> Result<i32, ParserError> {
        // debug!("-> varint32! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 5 bytes
        while shift < 35 { // (shift = 0, 7, 14 .. 35)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ParserError::BufferEndReached),
            };
            // debug!("count = {}, byte = 0b{:08b}", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b10000000) as i32) << shift;
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
        Err(ParserError::MalformedVarint32)
    }

    /// TODO: TEST
    /// Consumes 1-9 bytes that represent a 64-bit LEB128 signed integer encoding
    pub fn varint64(&mut self) -> Result<i64, ParserError> {
        // debug!("= varint64! <-");
        let mut result = 0;
        let mut shift = 0;
        // Can consume at most 9 bytes
        while shift < 63 { // (shift = 0, 7, 14 .. 56)
            let byte = match self.eat_byte() {
                Some(value) => value,
                None => return Err(ParserError::BufferEndReached),
            };
            // debug!("count = {}, byte = 0b{:08b}", count, byte);
            // Unset the msb and shift by multiples of 7 to the left
            let value = ((byte & !0b10000000) as i64) << shift;
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
        Err(ParserError::MalformedVarint64)
    }
}

// pub fn compile(source: Vec<u8>) -> Module {
// }
