use crate::eight::common::AccessibleValue;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableExpression;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::primary::{Expression, ExpressionRelations, RunnableExpression};
use crate::eight::expressions::secondary::{RunnableSecondaryExpression, SecondaryExpression};
use crate::eight::literals::basic::functions::parse_function_call;
use crate::eight::literals::basic::string::StringVal;
use crate::eight::expressions::secondary::TypedSecondaryExpression;
use crate::eight::literals::Literal;
use crate::eight::values::{Value, ValueTypes};


#[derive(Debug)]
pub struct Format {
    exprs: Vec<SecondaryExpression>,
}

pub fn get_fmt_relations(exprs: &Vec<SecondaryExpression>) -> Vec<RelationEntry> {
    let mut refs: Vec<Box<RelationEntry>> = Vec::new();
    for expr in exprs {
        let mut t: Vec<RelationEntry> = expr.get_references();
        for rel in t {
            refs.push(Box::from(rel));
        }
    }
    return vec![RelationEntry::new(vec![], vec![], refs)];
}

impl RunnableSecondaryExpression for Format {
    fn run_secondary_expr(mut self, interpreter: &mut EightInterpreter) -> MemRef {
        //TODO String interpolation here?
        let mut out_str = String::new();
        let mut exprs = Vec::new();
        std::mem::swap(&mut self.exprs, &mut exprs);
        for exp in exprs {
            let val = interpreter.step(exp);
            out_str.push_str(
                StringVal::explicit_cast_to(interpreter.get_val(&val)).value.as_str()
            );
            interpreter.pop_stack_vals(val);
        }
        interpreter.add_val_to_stack(Value::Literal(Literal::String(StringVal { value: format!("{}", out_str) })))
    }
}

impl TypedSecondaryExpression for Box<Format> {
    fn get_type(&self) -> Result<ValueTypes, String> {
        Ok(ValueTypes::StringType)
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        return get_fmt_relations(&self.exprs);
    }
}


impl Format {
    //pub fn parse<'a>(context: &mut Context) -> Option<SecondaryExpression>
    pub fn parse<'a, 'c>(context: &'c mut Context) -> Option<SecondaryExpression> {
        let idx = context.get_index();

        let (name, args) = match parse_function_call(context) {
            Some(n) => n,
            None => {
                return None;
            }
        };

        if name.eq("fmt") {
            return Some(
                SecondaryExpression::FormatExpr(Box::from(
                    Format { exprs: args.to_vec() }
                )));
        }
        context.jump(idx); // TODO?
        None
    }
}