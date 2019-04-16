use crate::{
    parser::{Parser, ParserResult},
    ValueType::{self, *},
};

// Extends Parser implementation
impl<'a> Parser<'a> {
    pub fn get_2_stack_args(&mut self, types: &[ValueType]) -> ParserResult<(usize, usize)> {
        // Check expected types match what's on stack
        self.validate_signature_match(types)?;

        let rhs = self.stack.pop().cloned().unwrap().operator_ref;
        let lhs = self.stack.pop().cloned().unwrap().operator_ref;

        Ok((lhs, rhs))
    }

    pub fn get_1_stack_arg(&mut self, ty: ValueType) -> ParserResult<usize> {
        // Check expected types match what's on stack
        self.validate_signature_match(&[ty])?;

        let arg = self.stack.pop().cloned().unwrap().operator_ref;

        Ok(arg)
    }
}
