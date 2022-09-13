use crate::eight::common::AccessibleValue;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableExpression;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::primary::{Expression, ExpressionRelations, RunnableExpression};
use crate::eight::expressions::primary::format::get_fmt_relations;
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::literals::basic::functions::parse_function_call;
use crate::eight::literals::basic::string::StringVal;
use crate::eight::values::Value;
use crate::eight::expressions::secondary::TypedSecondaryExpression;


#[derive(Debug)]
pub struct Print {
    exprs: Vec<SecondaryExpression>
}

impl RunnableExpression for Print {
    fn run_expr(mut self, interpreter: &mut EightInterpreter) {
        //TODO String interpolation here?
        let mut out_str = String::new();
        let mut exprs = Vec::new();
        std::mem::swap(&mut self.exprs, &mut exprs);
        for exp in exprs {
            let val = interpreter.step(exp);
            out_str.push_str(
                StringVal::implicit_cast_to(interpreter.get_val(&val)).value.as_str()
            );
            interpreter.pop_stack_vals(val);
        }
        println!("{}", out_str);
    }
}

impl ExpressionRelations for Box<Print> {
    fn get_expr_references(&self) -> Vec<RelationEntry> {
        return get_fmt_relations(&self.exprs);
    }
}

impl ParsableExpression for Print {
    fn parse<'a, 'c>(context: &'c mut Context) -> Option<Expression> {
        let idx = context.get_index();

        let (name, args) = match parse_function_call(context) {
            Some(n) => n,
            None => {
                return None;
            }
        };
        println!("IAMHERE");
        context.print_symbols_current();

        if name.eq("println") || name.eq("print") { // TODO differenate between print and println
            match context.get() {
                Symbols::Semicolon => {
                    context.increment();
                    return Some(Expression::PrintExpr(Box::from(Print {
                        exprs: args.to_vec(),
                    })));
                },
                o => {
                    panic!("{}", context.get_panic_smessage(format!("Expected ';' at end of print expression, got '{:?}'", o)));
                }
            }
        }
        context.jump(idx); // jump back to before parsing function call, wrong function found TODO?
        return None;
    }
}
