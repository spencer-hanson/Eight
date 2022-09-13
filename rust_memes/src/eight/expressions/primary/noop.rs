use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::AccessibleValue;
use crate::eight::expressions::primary::ExpressionRelations;
use crate::eight::expressions::secondary::TypedSecondaryExpression;
use crate::eight::values::{Value, ValueTypes};

#[derive(Debug)]
pub struct NoOp {}

impl TypedSecondaryExpression for NoOp {
    fn get_type(&self) -> Result<ValueTypes, String> {
        Ok(ValueTypes::NoneType)
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        vec![]
    }
}

impl ExpressionRelations for NoOp {
    fn get_expr_references(&self) -> Vec<RelationEntry> {
        vec![]
    }
}

impl AccessibleValue for NoOp {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self { todo!() }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self { todo!() }
}
