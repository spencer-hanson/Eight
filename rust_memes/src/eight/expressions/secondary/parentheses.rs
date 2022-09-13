use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::secondary::{parse_secondary_expression, RunnableSecondaryExpression, SecondaryExpression, TypedSecondaryExpression};
use crate::eight::values::ValueTypes;


#[derive(Debug)]
pub struct Parentheses {
    expr: SecondaryExpression
}

impl Parentheses {
    pub fn parse(context: &mut Context) -> Option<SecondaryExpression> {
        match context.get() {
            Symbols::ParenOpen => {
                let ln = context.get_line_no();
                context.increment();
                let s = parse_secondary_expression(context);
                match context.get() {
                    Symbols::ParenClose => {
                        context.increment();
                        return Some(SecondaryExpression::ParenExpr(Box::from(Parentheses {
                            expr: s
                        })));
                    },
                    _ => panic!("{}", context.get_panic_smessage(format!("Expected closing paren from '(' started on line {}", ln)))
                }
            },
            _ => None
        }
    }
}

impl TypedSecondaryExpression for Box<Parentheses> {
    fn get_type(&self) -> Result<ValueTypes, String> {
        self.expr.get_type()
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        self.expr.get_references()
    }
}

impl RunnableSecondaryExpression for Parentheses {
    fn run_secondary_expr(self, interpreter: &mut EightInterpreter) -> MemRef {
        self.expr.run_secondary_expr(interpreter)
    }
}
