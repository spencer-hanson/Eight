use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::secondary::{parse_secondary_expression, RunnableSecondaryExpression, SecondaryExpression, TypedSecondaryExpression};
use crate::eight::values::{Value, ValueTypes};

#[derive(Debug)]
pub struct List {
    pub(crate) data: Vec<Value>,
    pub(crate) typ: ValueTypes,
}

#[derive(Debug)]
pub struct ListExpression {
    pub(crate) exprs: Vec<SecondaryExpression>,
    pub(crate) typ: ValueTypes,
}


impl ListExpression {
    fn parse_element(ln: i32, context: &mut Context) -> (Option<SecondaryExpression>, Option<ValueTypes>, bool) {
        // Returns (SecondaryExpression, ValueType, is_next_expr: bool)
        context.increment();
        match context.get() {
            Symbols::BracketClose => {
                // Empty list
                return (None, None, false);
            },
            _ => ()
        }
        let s = parse_secondary_expression(context);
        let typ: Result<ValueTypes, String> = s.get_type();
        let v_typ: ValueTypes;

        match typ {
            Ok(v) => {
                v_typ = v;
            },
            Err(s) => {
                panic!("{}", context.get_panic_smessage(format!("Type error during list starting with '[' on line {} Error: {}", ln, s)))
            }
        }

        match context.get() {
            Symbols::BracketClose => {
                return (Some(s), Some(v_typ), false);
            },
            Symbols::Comma => {
                return (Some(s), Some(v_typ), true);
            },
            _ => panic!("{}", context.get_panic_smessage(format!("Expected closing bracket or comma from '[' started on line {}", ln)))
        }
    }

    pub fn parse(context: &mut Context) -> Option<SecondaryExpression> {
        match context.get() {
            Symbols::BracketOpen => {
                let ln = context.get_line_no();

                let expr_and_type = ListExpression::parse_element(ln, context);
                let expr = expr_and_type.0;
                if !expr_and_type.2 {
                    return Some(SecondaryExpression::ListExpr(ListExpression::empty()));
                }

                let mut exprs: Vec<SecondaryExpression> = Vec::new();

                exprs.push(expr.unwrap_or_else(|| {
                    panic!("{}", context.get_panic_smessage(format!("Expected value in list, none found from '[' starting on line {}", ln)));
                }));
                let orig_typ = expr_and_type.1.unwrap();

                loop {
                    match ListExpression::parse_element(ln, context) {
                        (Some(sexpr), Some(sexpr_typ), cont) => {
                            if orig_typ != sexpr_typ {
                                panic!("Currently only homogeneous-typed lists are supported"); // TODO mutli-typed list?
                            }
                            exprs.push(sexpr);

                            if !cont {
                                context.increment(); // Increment past closing ']'
                                return Some(SecondaryExpression::ListExpr(ListExpression::new(exprs, orig_typ)));
                            }
                        },
                        p => panic!("{}", context.get_panic_smessage(format!("Error parsing list: unexpected pattern returned from List::parse_element! {:?}", p)))
                    }
                }
            },
            _ => None
        }
    }

    pub fn new(exprs: Vec<SecondaryExpression>, typ: ValueTypes) -> Self {
        ListExpression {
            exprs,
            typ
        }
    }

    pub fn empty() -> Self {
        ListExpression::new(vec![], ValueTypes::NoneType)
    }
}

impl TypedSecondaryExpression for ListExpression {
    fn get_type(&self) -> Result<ValueTypes, String> {
        Ok(ValueTypes::ListType(Box::from(self.typ.clone())))
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        todo!()
    }
}

impl RunnableSecondaryExpression for ListExpression {
    fn run_secondary_expr(self, interpreter: &mut EightInterpreter) -> MemRef {
        todo!()
    }
}