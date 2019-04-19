use crate::{
    ir::Operator,
    parser::{Parser, ParserResult},
    stack::StackValue,
    ValueType::{self, *},
};
use wasmo_utils::verbose;

// Extends Parser implementation
impl<'a> Parser<'a> {
    pub fn operator_numeric_2_args(
        &mut self,
        param_ty: ValueType,
        return_ty: ValueType,
        operator_variant_func: fn(usize, usize) -> Operator,
    ) -> ParserResult<Operator> {
        let (lhs, rhs) = self.get_2_stack_args(&[param_ty.clone(), param_ty])?;

        self.stack.push(StackValue::new(return_ty, self.operator_index));

        Ok(operator_variant_func(lhs, rhs))
    }

    pub fn operator_numeric_1_arg(
        &mut self,
        param_ty: ValueType,
        return_ty: ValueType,
        operator_variant_func: fn(usize) -> Operator,
    ) -> ParserResult<Operator> {
        let arg = self.get_1_stack_arg(param_ty)?;

        self.stack.push(StackValue::new(return_ty, self.operator_index));

        Ok(operator_variant_func(arg))
    }
}
