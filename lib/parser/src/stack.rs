use crate::ValueType::{self, *};
use std::fmt::{self, Formatter};

///
#[derive(Clone, PartialEq)]
pub struct StackValue {
    pub(crate) value_type: ValueType,
    pub(crate) operator_ref: usize,
}

impl StackValue {
    pub fn new(value_type: ValueType, operator_ref: usize) -> Self {
        StackValue {
            value_type,
            operator_ref,
        }
    }
}

impl Into<ValueType> for StackValue {
    fn into(self) -> ValueType {
        self.value_type
    }
}

impl fmt::Debug for StackValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.value_type)
            .field(&self.operator_ref)
            .finish()
    }
}

///
#[derive(Clone, PartialEq)]
pub struct Stack {
    pub(crate) pointer: usize,
    pub(crate) stack: Vec<StackValue>,
}

impl Stack {
    ///
    pub fn new() -> Self {
        Self {
            pointer: 0,
            stack: vec![StackValue::new(I64, 0); 30], // Stack has initial size of 30. That should be enough for most function arguments
        }
    }

    ///
    pub fn pop(&mut self) -> Option<&StackValue> {
        if self.pointer > 0 {
            // Decrement stack pointer
            self.pointer -= 1;

            // Return popped value
            return Some(&self.stack[self.pointer]);
        }

        None
    }

    ///
    pub fn push(&mut self, value: StackValue) {
        // Check if capacity has been reached.
        if self.pointer >= self.stack.capacity() {
            // Increase the stack size by 15
            self.stack.resize(self.pointer + 15, StackValue::new(I64, 0)
            );
        }

        // Set the stack value
        self.stack[self.pointer] = value;

        // Incerment stack pointer
        self.pointer += 1;
    }

    ///
    pub fn size(&self) -> usize {
        self.pointer
    }

    ///
    pub fn check_types(&self, types: &[ValueType]) -> bool {
        let types_len = types.len();

        // Take the first `types_len` values on the stack.
        let types_iter = self.stack.iter().take(types_len);

        // Convert values to types
        let types_iter: Vec<ValueType> = types_iter.cloned().map(|x| x.into()).collect();

        types_iter == types
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_list()
            .entries(&self.stack[..self.pointer].to_vec())
            .finish()
    }
}

