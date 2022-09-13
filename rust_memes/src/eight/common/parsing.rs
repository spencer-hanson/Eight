use crate::eight::common::parsing::context::Context;
use crate::eight::expressions::primary::Expression;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::expressions::secondary::{organize_secondary_expression_list, partial_parse_secondary_expression, SecondaryExpression, TransitionExpression};
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::Literal;
use crate::eight::expressions::secondary::operators::binary::{BinaryOperator, BinaryOperators};

pub mod ast;
pub mod context;
pub mod util;

pub trait ParsableExpression {
    fn parse<'a, 'c>(context: &'c mut Context) -> Option<Expression>;
}

pub trait ParsableLiteral {
    fn parse<'a>(context: &mut Context) -> Option<Literal>;

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String>;
}

pub trait ParsableOperator {
    // Take the expression list and simplify it given the idx of the operator
    fn parse(context: &mut Context, expr_list: &mut Vec<TransitionExpression>, idx: usize, op: BinaryOperators);
}
