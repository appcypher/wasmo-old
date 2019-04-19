use crate::{
    errors::ParserError,
    ir::Operator,
    kinds::ErrorKind,
    parser::{Parser, ParserResult},
    stack::StackValue,
    ValueType::{self, *},
};
use wasmo_utils::verbose;

// Extends Parser implementation
impl<'a> Parser<'a> {
    pub fn operator_memory_load(
        &mut self,
        ty: ValueType,
        operator_variant_func: fn(u32, u32, usize) -> Operator,
    ) -> ParserResult<Operator> {
        let cursor = self.cursor;

        // TODO: Abstract

        // TODO: Validation => alignment_log2 in [1, 2, 4]
        let alignment_log2 = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemoryOperator,
            MalformedAlignmentInMemoryOperator
        );

        // TODO: Validation
        let offset = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemoryOperator,
            MalformedOffsetInMemoryOperator
        );

        // ++++++++++++++++++++++++++++++++

        let arg = self.get_1_stack_arg(I32)?;

        self.stack.push(StackValue::new(ty, self.operator_index));

        Ok(operator_variant_func(alignment_log2, offset, arg))
    }

    pub fn operator_memory_store(
        &mut self,
        ty: ValueType,
        operator_variant_func: fn(u32, u32, usize, usize) -> Operator,
    ) -> ParserResult<Operator> {
        let cursor = self.cursor;

        // TODO: Validation => alignment_log2 in [1, 2, 4]
        let alignment_log2 = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemoryOperator,
            MalformedAlignmentInMemoryOperator
        );

        // TODO: Validation
        let offset = get_value!(
            self.varuint32(),
            cursor,
            IncompleteMemoryOperator,
            MalformedOffsetInMemoryOperator
        );

        let (base, value) = self.get_2_stack_args(&[I32, ty])?;

        Ok(operator_variant_func(alignment_log2, offset, base, value))
    }

    pub fn operator_memory_grow(&mut self) -> ParserResult<Operator> {
        let cursor = self.cursor;

        let delta = self.get_1_stack_arg(I32)?;

        let _reserved = get_value!(
            self.varuint1(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(I32, self.operator_index));

        Ok(Operator::MemoryGrow(delta))
    }

    pub fn operator_memory_size(&mut self) -> ParserResult<Operator> {
        let cursor = self.cursor;

        let _reserved = get_value!(
            self.varuint1(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        self.stack.push(StackValue::new(I32, self.operator_index));

        Ok(Operator::MemorySize)
    }
}
