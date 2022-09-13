use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::expressions::primary::ExpressionRelations;
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::expressions::secondary::TypedSecondaryExpression;


#[derive(Debug)]
pub struct ReturnExpr {
    expr: SecondaryExpression,
}


impl<'a> ExpressionRelations for ReturnExpr {
    fn get_expr_references(&self) -> Vec<RelationEntry> {
        return self.expr.get_references();
    }
}
