use crate::{
    errors::ParserError,
    ir::{Operator, Section, Local, BlockType},
    kinds::ErrorKind,
    parser::{Parser, ParserResult},
    stack::{StackValue, Stack},
    ValueType::{self, *},
};
use wasmo_utils::debug;
use hashbrown::HashMap;

// Extends Parser implementation
impl<'a> Parser<'a> {
    ///
    pub fn operator_block(&mut self, sections: &HashMap<u8, Section>, locals: Option<&[Local]>) -> ParserResult<Operator> {
        let cursor = self.cursor;

        // Keep original values
        let old_stack_values = self.stack.values();
        let old_operator_index = self.operator_index;

        // Reset stack.
        self.reset_instructions_state();

        debug!("== ENTER Block");

        let type_id = get_value!(
            self.varint7(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        // Consume instructions
        let operators = self.instructions(sections, locals)?;

        // Validate block result signature matches stack types
        self.validate_block_result_signature(type_id)?;

        debug!("== LEAVE Block");

        // Reset stack.
        self.reset_instructions_state();

        // Restore previous stack state
        self.stack.push_values(old_stack_values);
        self.operator_index = old_operator_index;

        // Push value to stack
        if let BlockType::Type(ty) = BlockType::from(type_id) {
            self.stack.push(StackValue::new(ty, self.operator_index));
        }

        Ok(Operator::Block(operators))
    }
}
