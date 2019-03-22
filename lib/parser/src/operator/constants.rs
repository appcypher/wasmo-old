use crate::{
    errors::ParserError,
    ir::Operator,
    kinds::ErrorKind,
    parser::{Parser, ParserResult},
    stack::StackValue,
    ValueType::{self, *},
};
use wasmlite_utils::verbose;

// Extends Parser implementation
impl<'a> Parser<'a> {
    pub fn operator_i32_const(&mut self) -> ParserResult<Operator> {
        verbose!("-> operator_i32_const! <-");
        let cursor = self.cursor;

        let value = get_value!(
            self.varint32(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(I32, self.operator_index));

        Ok(Operator::I32Const(value))
    }

    pub fn operator_i64_const(&mut self) -> ParserResult<Operator> {
        verbose!("-> operator_i64_const! <-");
        let cursor = self.cursor;

        let value = get_value!(
            self.varint64(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(I64, self.operator_index));

        Ok(Operator::I64Const(value))
    }

    pub fn operator_f32_const(&mut self) -> ParserResult<Operator> {
        verbose!("-> operator_f32_const! <-");
        let cursor = self.cursor;

        let value = get_value!(
            self.uint32(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(F32, self.operator_index));

        Ok(Operator::F32Const(value as _))
    }

    pub fn operator_f64_const(&mut self) -> ParserResult<Operator> {
        verbose!("-> operator_f64_const! <-");
        let cursor = self.cursor;

        let value = get_value!(
            self.uint64(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(F64, self.operator_index));

        Ok(Operator::F64Const(value as _))
    }
}
