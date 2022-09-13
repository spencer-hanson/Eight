use common::parsing::context::Context;
use values::namespaces::database::{CSV, JSON};
use values::namespaces::model::Model;
use enum_dispatch::enum_dispatch;
use expressions::primary::variable::Variable;

use crate::eight::common::parsing::ast::RelationEntry;
use expressions::primary::noop::NoOp;
use literals::basic::bool::BoolVal;
use literals::basic::functions::signature::FuncSignature;
use literals::basic::functions::Func;
use literals::basic::number::NumberVal;
use literals::basic::string::StringVal;
use literals::basic::variableref::VariableTokenRef;

use crate::eight::common::parsing::util::{consume_until_symbol, consume_until_symboltype};
use crate::eight::common::parsing::ParsableExpression;
use crate::eight::common::tokenizing::lexing;
use crate::eight::common::tokenizing::symbols::{SymbolType, Symbols};
use crate::eight::literals::{Literal};
use expressions::primary::print::Print;
use expressions::primary::returnexpr::ReturnExpr;
use expressions::secondary::callfunc::CallFunc;
use log::{debug, info, trace};
use crate::eight::common::AccessibleValue;
use common::running::interpreter::EightInterpreter;
use crate::eight::expressions::primary::Expression;
use crate::eight::expressions::secondary::operators::binary::BinaryOperator;

pub mod common;
pub mod values;
pub mod expressions;
pub mod literals;
pub mod runners;

pub fn parse_expr(context: &mut Context) -> Option<Expression> {
    match context.get_safe() {
        Ok(symbol) => {
            debug!("Found symbol: {:?}", symbol);

            //Multi-line Comment parsing
            if symbol == Symbols::MultiCommentStart {
                context.increment();
                consume_until_symbol(context, Symbols::MultiCommentEnd);
                debug!("Multi comment found");
                return None;
            }

            // Single-line Comment parsing
            if symbol == Symbols::SingleComment {
                context.increment();
                consume_until_symboltype(context, SymbolType::Newline);
                debug!("Single comment found");
                return None;
            }

            // Variable Expression parsing
            match Variable::parse(context) {
                Some(Expression::VariableTokenExpr(vt)) => {
                    return Some(Expression::VariableTokenExpr(vt));
                }
                _ => {
                    trace!("Didn't find VariableToken");
                }, // Line didn't match
            }

            match Print::parse(context) {
                Some(p) => {
                    return Some(p);
                }
                _ => (),
            }
            panic!("{}", context.get_panic_message("Unknown code fragment"));
        }
        Err(e) => {
            debug!("Ran into EOF! Done?");
            return Some(Expression::EndParsingExpr(NoOp {}));
        }
    }
}

pub fn parse_exprs(context: &mut Context) -> Vec<Expression> {
    debug!("----------PARSING START----------");
    let mut exprs = Vec::new();

    loop {
        let expr = parse_expr(context);
        trace!("Looping");
        match expr {
            Some(Expression::EndParsingExpr(_)) => {
                debug!("Reached EOF");
                break;
            }
            Some(x) => match x {
                Expression::VariableTokenExpr(expr) => {
                    let val_typ = expr.get_var_type();
                    match val_typ {
                        Ok(typ) => {
                            context.vartable.insert(String::from(&expr.name), typ);
                            exprs.push(Expression::VariableTokenExpr(expr));
                        }
                        Err(e) => {
                            // context index might be off by one right here, removing decrement()
                            panic!("{}", context.get_panic_smessage(e));
                        }
                    }
                }
                ex => {
                    exprs.push(ex);
                }
            },
            None => (),
        }

        debug!("Exprs parsed {:?}", exprs);
        context.print_vartable()
    }
    debug!("----------PARSING END----------");
    return exprs;
}

pub fn start_parse<'a>(code: String) -> Vec<Expression> {
    let symbols = lexing::parse(
        format!("{}\n", code.clone()) // Add newline to end of file to make parsing go smoother
    );

    let mut context = Context::new(code.clone(), symbols, 0);

    let exprs = parse_exprs(&mut context);
    return exprs;
}
