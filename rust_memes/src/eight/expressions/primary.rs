use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::expressions::primary::noop::NoOp;
use crate::eight::expressions::primary::print::Print;
use crate::eight::expressions::primary::returnexpr::ReturnExpr;
use crate::eight::expressions::primary::variable::Variable;
use enum_dispatch::enum_dispatch;
use crate::eight::expressions::primary::format::Format;

pub mod noop;
pub mod print;
pub mod returnexpr;
pub mod variable;
pub mod format;


#[enum_dispatch]
pub trait ExpressionRelations {
    fn get_expr_references(&self) -> Vec<RelationEntry>;
}

pub trait RunnableExpression {
    fn run_expr(self, interpreter: &mut EightInterpreter);
}

#[enum_dispatch(ExpressionRelations)]
#[derive(Debug)]
pub enum Expression {
    VariableTokenExpr(Box<Variable>),
    ReturnExpr(ReturnExpr),
    PrintExpr(Box<Print>),
    EndParsingExpr(NoOp)
}

impl Expression {
    // Remember if you copy the trait func sig to remove the &mut -> mut to consume the expression
    pub(crate) fn run_expr(mut self, interpreter: &mut EightInterpreter) {
        match self {
            Expression::VariableTokenExpr(mut v) => { v.run_expr(interpreter) }
            Expression::PrintExpr(mut p) => { p.run_expr(interpreter) }
            _ => {
                // Add other expressions here
                todo!()
            }
        };
    }
}
