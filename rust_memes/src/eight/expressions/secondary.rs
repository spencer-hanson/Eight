pub mod callfunc;
pub mod operators;
pub mod parentheses;

use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableOperator;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::values::namespaces::parse_namespace;
use crate::eight::literals::basic::functions::parse_function_call;
use crate::eight::literals::basic::variableref::VariableTokenRef;
use crate::eight::literals::{Literal, parse_literal};
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::expressions::primary::noop::NoOp;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::expressions::secondary::operators::binary::{BinaryOperator, sym_to_binop};
use crate::eight::values::{Value, ValueTypes};
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::primary::format::Format;

use log::{debug, trace};
use enum_dispatch::enum_dispatch;
use crate::eight::expressions::secondary::parentheses::Parentheses;
use crate::eight::literals::basic::list::ListExpression;


#[derive(Debug)]
pub enum IntermediateExpression {
    Operator(Symbols),
    SecondaryExpr(SecondaryExpression),
}

#[derive(Debug)]
pub enum TransitionExpression {
    InterExpr(IntermediateExpression),
    TransExpr(SecondaryExpression),
}

#[enum_dispatch]
pub trait TypedSecondaryExpression {
    fn get_type(&self) -> Result<ValueTypes, String>;

    fn get_references(&self) -> Vec<RelationEntry>;
}

pub trait RunnableSecondaryExpression {
    fn run_secondary_expr(self, interpreter: &mut EightInterpreter) -> MemRef;
}

#[enum_dispatch(TypedSecondaryExpression)]
#[derive(Debug)]
pub enum SecondaryExpression {
    LiteralExpr(Literal),
    ListExpr(ListExpression),
    BinaryOperation(Box<BinaryOperator>),
    CallResult(CallFunc),
    Reference(VariableTokenRef),
    NoOpExpr(NoOp),
    FormatExpr(Box<Format>),
    ParenExpr(Box<Parentheses>)
    // TypecastExpr()
}

impl RunnableSecondaryExpression for SecondaryExpression {
    fn run_secondary_expr(mut self, interpreter: &mut EightInterpreter) -> MemRef {
        match self {
            SecondaryExpression::BinaryOperation(mut a) => {
                a.run_secondary_expr(interpreter)
            }
            SecondaryExpression::Reference(mut r) => {
                r.run_secondary_expr(interpreter)
            }
            SecondaryExpression::CallResult(mut c) => {
                c.run_secondary_expr(interpreter)
            }
            SecondaryExpression::NoOpExpr(_) => {
                MemRef::empty()
            },
            SecondaryExpression::LiteralExpr(l) => {
                interpreter.add_val_to_stack(Value::Literal(l))
            },
            SecondaryExpression::FormatExpr(mut f) => {
                f.run_secondary_expr(interpreter)
            },
            SecondaryExpression::ParenExpr(p) => {
                p.run_secondary_expr(interpreter)
            },
            SecondaryExpression::ListExpr(l) => {
                l.run_secondary_expr(interpreter)
            }
        }
    }
}

impl SecondaryExpression {
    pub fn gen_noop() -> Self {
        SecondaryExpression::NoOpExpr(NoOp {})
    }

    pub fn combine_references(ex1: &SecondaryExpression, ex2: &SecondaryExpression) -> Vec<RelationEntry> {
        let mut r = ex1.get_references();
        let mut l = ex2.get_references();
        r.append(&mut l);
        return r;
    }
}

pub fn get_operator_order() -> Vec<Symbols> {
    let mut op_order: Vec<Symbols> = Vec::new();
    op_order.push(Symbols::Exponent);
    op_order.push(Symbols::Multiply);
    op_order.push(Symbols::Divide);
    op_order.push(Symbols::Divide);
    op_order.push(Symbols::Modulus);
    op_order.push(Symbols::Add);
    op_order.push(Symbols::Sub);
    op_order.push(Symbols::EqualityCheck);
    return op_order;
}

pub fn match_operator_symbol(context: &mut Context) -> Option<Symbols> {
    let sym = context.get();
    let ops = get_operator_order();
    for op in ops {
        if sym == op {
            return Some(sym);
        }
    }
    return None;
}

pub fn organize_secondary_expression_list<'a, 'b>(context: &mut Context<'a>, mut expr_list: Vec<IntermediateExpression>) -> SecondaryExpression {
    /*
     Take a list of Expressions (Value Expression) and Operators,
     apply Operation order rules and return a unified expression
    */
    let mut expression_list = Vec::new();
    let op_order = get_operator_order();

    // Transition to expression
    for expr in expr_list {
        match expr {
            IntermediateExpression::SecondaryExpr(sec) => {
                expression_list.push(TransitionExpression::TransExpr(sec));
            }
            o => {
                expression_list.push(TransitionExpression::InterExpr(o));
            }
        }
    }

    let expression_limit = 1000;
    let mut expr_count = 0;

    for op in op_order.iter() {
        loop {
            let mut found = false;
            let mut found_idx = 0;
            let mut found_op = None;
            let mut idx = 0;

            for expr in expression_list.iter() {
                match expr {
                    TransitionExpression::InterExpr(interexpr) => {
                        if expr_count >= expression_limit {
                            panic!("{}", context.get_panic_smessage(format!("Expression parsing limit reached, can't simplify expression, too large")));
                        }

                        match interexpr {
                            IntermediateExpression::Operator(interop) => {
                                if op == interop {
                                    found_op = Some(op);
                                    found_idx = idx;
                                    found = true;
                                    break;
                                } else {
                                    idx += 1;
                                    continue;
                                }
                            }
                            _ => {
                                expr_count += 1;
                                idx += 1;
                                continue;
                            }
                        }
                    }
                    oo => {
                        idx += 1;
                        continue;
                    }
                }
            }

            if found {
                BinaryOperator::parse(context, &mut expression_list, found_idx, sym_to_binop(&found_op.unwrap(), &context));
                continue;
            } else {
                break; // Break out of loop
            }
        }
    }

    if expression_list.len() != 1 {
        panic!(
            "{}",
            context.get_panic_smessage(format!(
                "Expressionlist not simplified! '{:?}'",
                expression_list
            ))
        );
    }

    let val = expression_list.pop();
    match val {
        Some(v) => {
            return match v {
                TransitionExpression::TransExpr(ex) => ex,
                TransitionExpression::InterExpr(ex) => {
                    return match ex {
                        IntermediateExpression::SecondaryExpr(s) => s,
                        _ => {
                            panic!(
                                "{}",
                                context.get_panic_smessage(format!(
                                    "Non-value intermediate expression found"
                                ))
                            );
                        }
                    };
                }
            };
        }
        None => {
            panic!(
                "{}",
                context.get_panic_smessage(format!("No expression left after simplifying!"))
            );
        }
    }
}

fn secondary_expr_to_intermediate(s: SecondaryExpression) -> IntermediateExpression {
    IntermediateExpression::SecondaryExpr(s)
}

pub fn partial_parse_secondary_expression<'a>(context: &mut Context) -> Option<Vec<IntermediateExpression>> {
    let mut secexpr: Option<SecondaryExpression> = None;
    let mut found = false;

    // TODO check pre-operator '!' here

    // Literal here
    debug!("Attempting Literal parse");
    match parse_literal(context) {
        Some(x) => {
            secexpr = Some(SecondaryExpression::LiteralExpr(x));
            found = true;
        }
        None => {
            debug!("Value parse fail");
        }
    }

    //Secondary expressions here

    // Parse list expression
    if !found {
        debug!("Attempting list expression parse");
        match ListExpression::parse(context) {
            Some(s) => {
                secexpr = Some(s);
                found = true;
            },
            None => {
                debug!("list expression parse fail");
            }
        }
    }

    // Parse fmt function call
    if !found {
        debug!("Attempting fmt call parse");
        match Format::parse(context) {
            Some(s) => {
                secexpr = Some(s);
                found = true;
            },
            None => {
                debug!("fmt call parse fail");
            }
        }
    }

    // Parse parentheses
    if !found {
        debug!("Attempting paren parse");
        match Parentheses::parse(context) {
            Some(p) => {
                secexpr = Some(p);
            },
            None => {
                debug!("Paren parse fail");
            }
        }
    }

    // Parse function call
    // if !found {
    //     debug!("Attempting Function call parse");
    //     match parse_function_call(context) {
    //         Some((name, args)) => {
    //             // TODO figure out if the literal 'name' exists in the current scope, and if the
    //             // args match the signature, then return a CallResult(FuncReference, args)
    //             todo!();
    //         }
    //         None => {
    //             debug!("Function call parse fail");
    //         }
    //     }
    // }

    // Parse namespaced var
    if !found {
        debug!("Attempting namespace parse");
        match parse_namespace(context) {
            Some(x) => {
                secexpr = Some(x);
                found = true;
            }
            None => {
                debug!("Namespace parse fail");
            }
        }
    }

    // TODO Pointer parsing here

    // Parse VariableTokenRef
    if !found {
        debug!("Attempting VarTokenRef parse");
        match VariableTokenRef::parse(context) {
            Some(x) => {
                secexpr = Some(x);
                found = true;
            }
            None => {
                debug!("VarTokenRef parse fail");
            }
        }
    }

    return match secexpr {
        Some(x) => match match_operator_symbol(context) {
            Some(op) => {
                trace!("Found operator {:?}", op);
                let mut v = Vec::new();
                v.push(secondary_expr_to_intermediate(x));
                v.push(IntermediateExpression::Operator(op.clone()));
                context.increment();

                match partial_parse_secondary_expression(context) {
                    Some(n) => {
                        for i in n.into_iter() {
                            let g = i;
                            v.push(g);
                        }
                        Some(v)
                    }
                    None => {
                        panic!(
                            "{}",
                            context.get_panic_smessage(format!(
                                "Expected expression after operator '{:?}'",
                                op.clone()
                            ))
                        );
                    }
                }
            }
            None => {
                trace!("Didn't find operator, just expr {:?}", &x);
                let mut v = Vec::new();
                v.push(secondary_expr_to_intermediate(x));
                Some(v)
            }
        },
        None => None,
    };
}

pub fn parse_secondary_expression<'a, 'b>(context: &'a mut Context) -> SecondaryExpression {
    let val = partial_parse_secondary_expression(context);
    match val {
        Some(mut x) => organize_secondary_expression_list(context, x),
        None => panic!(
            "{}",
            context.get_panic_message("Unknown secondary expression statement")
        ),
    }
}
