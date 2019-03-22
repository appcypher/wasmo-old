use crate::{
    errors::ParserError,
    kinds::ErrorKind,
    parser::{Parser, ParserResult},
    ValueType::{self, *},
    ir::FuncSignature,
};

// TODO: MAKE THESE VALIDATIONS AN IMPL OF

impl<'a> Parser<'a> {
    /// Implementation based on Unicode Standard 11.0, Section 3.9, Table 3-7.
    // TODO: TEST THOROUGHLY!! (Codepoints, Grapheme Clusters, etc.)
    pub fn validate_utf8(&self, bytes: &[u8]) -> bool {
        let cursor = self.cursor;

        let length = bytes.len();
        let mut cursor = 0;

        // There must be at least a byte for a valid utf-8 string
        if length == 0 {
            return false;
        }

        //
        while cursor < length {
            //
            if bytes[cursor] < 0x80 {
                // 1-byte encoding
                cursor += 1;
            } else if
            // 2-byte encoding
            // Check if there is at least 2 bytes needed by following conditions.
            cursor < length - 2 &&
            (bytes[cursor] >= 0xC2 && bytes[cursor] <= 0xDF) && // byte_1
            (bytes[cursor + 1] >= 0x80 && bytes[cursor + 1] <= 0xBF)
            {
                // byte_2
                cursor += 2;
            } else if
            // 3-byte encoding
            // Check if there is at least 3 bytes needed by following conditions.
            cursor < length - 3
                && (
                    (bytes[cursor] == 0xE0) && // byte_1
                (bytes[cursor + 1] >= 0xA0 && bytes[cursor + 1] <= 0xBF) && // byte_2
                (bytes[cursor + 2] >= 0x80 && bytes[cursor + 3] <= 0xBF)
                    // byte_3
                )
                || (
                    ((bytes[cursor] >= 0xE1 && bytes[cursor] <= 0xEC) || (bytes[cursor] >= 0xEE && bytes[cursor] <= 0xEF)) && // byte_1
                (bytes[cursor + 1] >= 0x80 && bytes[cursor + 1] <= 0xBF) && // byte_2
                (bytes[cursor + 2] >= 0x80 && bytes[cursor + 3] <= 0xBF)
                    // byte_3
                )
                || (
                    (bytes[cursor] == 0xED) && // byte_1
                (bytes[cursor + 1] >= 0x80 && bytes[cursor + 1] <= 0x9F) && // byte_2
                (bytes[cursor + 2] >= 0x80 && bytes[cursor + 3] <= 0xBF)
                    // byte_3
                )
            {
                cursor += 3;
            } else if
            // 4-byte encoding
            // Check if there is at least 4 bytes needed by following conditions.
            cursor < length - 4
                && (
                    (bytes[cursor] == 0xF0) && // byte_1
                (bytes[cursor + 1] >= 0x80 && bytes[cursor + 1] <= 0xBF) && // byte_2
                (bytes[cursor + 2] >= 0x7F && bytes[cursor + 2] <= 0xBF) && // byte_3
                (bytes[cursor + 3] >= 0x7F && bytes[cursor + 3] <= 0xBF)
                    // byte_4
                )
                || (
                    ((bytes[cursor] >= 0xF1 && bytes[cursor] <= 0xF3) || (bytes[cursor] == 0xF4)) && // byte_1
                (bytes[cursor + 1] >= 0x80 && bytes[cursor + 1] <= 0xBF) && // byte_2
                (bytes[cursor + 2] >= 0x80 && bytes[cursor + 2] <= 0xBF) && // byte_3
                (bytes[cursor + 3] >= 0x80 && bytes[cursor + 3] <= 0xBF)
                    // byte_4
                )
            {
                cursor += 4;
            } else {
                return false;
            }
        }
        true
    }

    /// Validates that a particular section hasn't already been defined.
    pub fn validate_section_exists(&mut self, section_id: u8) -> ParserResult<()> {
        let cursor = self.cursor;

        // Check if section has already been consumed.
        if self.sections_consumed.contains(&section_id) {
            return Err(ParserError {
                kind: ErrorKind::SectionAlreadyDefined,
                cursor,
            });
        }

        // Save section_id in parser's consumed section.
        self.push_section_id(&section_id);

        Ok(())
    }

    ///
    pub fn validate_signature_match(&self, expected_types: &[ValueType]) -> ParserResult<()> {
        let cursor = self.cursor;

        if !self.stack.check_types(expected_types) {
            let stack_types = self.stack.types();

            return Err(ParserError {
                cursor,
                kind: ErrorKind::MismatchedOperandTypes {
                    expected: expected_types.to_vec(),
                    found: stack_types,
                }
            });
        }
        Ok(())
    }

    ///
    pub fn validate_function_return_signature(&self, expected_signature: FuncSignature) -> ParserResult<()> {
        let cursor = self.cursor;

        if !self.stack.check_types(&expected_signature.returns) {
            let stack_types = self.stack.types();

            return Err(ParserError {
                cursor,
                kind: ErrorKind::MismatchedFunctionReturnSignature {
                    expected: expected_signature,
                    return_type_found: stack_types,
                }
            });
        }
        Ok(())
    }
}




// LIST
// mem operations Alignment= power df two, not more than 4
