use log::{debug, trace};

use basic::bool::BoolVal;
use basic::number::NumberVal;
use basic::string::StringVal;

use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use enum_dispatch::enum_dispatch;
use crate::eight::expressions::secondary::TypedSecondaryExpression;
use crate::eight::values::{BasicValue, Value, ValueTypes};

pub mod basic;


#[derive(Debug)]
pub enum Literal {
    Boolean(BoolVal),
    String(StringVal),
    Number(NumberVal),
}
impl BasicValue for Literal {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Literal::Boolean(b) => {
                b.eq(other)
            },
            Literal::String(s) => {
                s.eq(other)
            },
            Literal::Number(n) => {
                n.eq(other)
            }
        }
    }
}

impl TypedSecondaryExpression for Literal {
    fn get_type(&self) -> Result<ValueTypes, String> {
        return Ok(match self {
            Literal::Boolean(_) => ValueTypes::BooleanType,
            Literal::String(_) => ValueTypes::StringType,
            Literal::Number(_) => ValueTypes::NumberType,
        });
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        vec![]
    }
}

pub fn parse_literal<'a>(context: &mut Context) -> Option<Literal> {
    // Parse bool
    debug!("Attempting Bool Parse");
    match BoolVal::parse(context) {
        Some(x) => {
            return Some(x);
        }
        None => {
            debug!("Bool parse fail");
        }
    }

    // Parse string
    debug!("Attempting String parse");
    match StringVal::parse(context) {
        Some(x) => {
            return Some(x);
        }
        None => {
            debug!("String parse fail");
        }
    }

    // Parse number
    debug!("Attempting Number parse");
    match NumberVal::parse(context) {
        Some(x) => {
            return Some(x);
        }
        None => {
            debug!("Number parse fail");
        }
    }

    None
}
