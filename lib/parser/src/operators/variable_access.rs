use crate::{
    ir::{Operator, Local, Section, get_global_by_index},
    parser::{Parser, ParserResult},
    kinds::ErrorKind,
    errors::ParserError,
    stack::StackValue,
    ValueType::{self, *},
};
use hashbrown::HashMap;

// Extends Parser implementation
impl<'a> Parser<'a> {
    ///
    pub fn get_local_details(&mut self, locals: Option<&[Local]>) -> ParserResult<(ValueType, u32)> {
        let cursor = self.cursor;

        // The local index
        let local_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        let index = local_index as usize;

        let locals = locals.unwrap();

        // Validate that local exists
        if index >= locals.len() {
            return Err(ParserError {
                kind: ErrorKind::LocalDoesNotExist,
                cursor,
            })
        }

        // Get the type of the local
        let local_type = locals[index].local_type.clone();

        Ok((local_type, local_index))
    }

    ///
    pub fn operator_local_get(&mut self, locals: Option<&[Local]>) -> ParserResult<Operator> {
        let (local_type, local_index) = self.get_local_details(locals)?;

        self.stack
            .push(StackValue::new(local_type, self.operator_index));

        Ok(Operator::LocalGet(local_index))
    }

    ///
    pub fn operator_local_set(&mut self, locals: Option<&[Local]>) -> ParserResult<Operator> {
        let (local_type, local_index) = self.get_local_details(locals)?;

        // Get value from the stack
        let arg = self.get_1_stack_arg(local_type.clone())?;

        Ok(Operator::LocalSet(local_index, arg))
    }

    ///
    pub fn operator_local_tee(&mut self, locals: Option<&[Local]>) -> ParserResult<Operator> {
        let (local_type, local_index) = self.get_local_details(locals)?;

        // Get value from the stack
        let arg = self.get_1_stack_arg(local_type.clone())?;

        self.stack
            .push(StackValue::new(local_type, self.operator_index));

        Ok(Operator::LocalTee(local_index, arg))
    }

    ///
    pub fn get_global_details(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<(ValueType, u32)> {
        let cursor = self.cursor;

        // The global index
        let global_index = get_value!(
            self.varuint32(),
            cursor,
            IncompleteExpression,
            MalformedOpcodeInExpression
        );

        let global = get_global_by_index(global_index, sections);

        // Validate that global exists
        if global.is_none() {
            return Err(ParserError {
                kind: ErrorKind::GlobalDoesNotExist,
                cursor,
            })
        }

        let global = global.unwrap();

        // Get the type of the local
        let global_type = global.content_type;

        Ok((global_type, global_index))
    }

    ///
    pub fn operator_global_get(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Operator> {
        let (global_type, global_index) = self.get_global_details(sections)?;

        self.stack
            .push(StackValue::new(global_type, self.operator_index));

        Ok(Operator::GlobalGet(global_index))
    }

    ///
    pub fn operator_global_set(&mut self, sections: &HashMap<u8, Section>) -> ParserResult<Operator> {
        let (global_type, global_index) = self.get_global_details(sections)?;

        // Get value from the stack
        let arg = self.get_1_stack_arg(global_type.clone())?;

        Ok(Operator::GlobalSet(global_index, arg))
    }
}
