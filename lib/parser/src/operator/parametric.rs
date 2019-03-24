use crate::{
    errors::ParserError,
    ir::{Operator, Section, Local},
    kinds::ErrorKind,
    parser::{Parser, ParserResult},
    stack::{StackValue, Stack},
    ValueType::{self, *},
};
use wasmlite_utils::verbose;
use hashbrown::HashMap;

// Extends Parser implementation
impl<'a> Parser<'a> {
    pub fn operator_drop(&mut self) -> ParserResult<Operator> {
        self.stack.pop();
        Ok(Operator::Drop)
    }
}
