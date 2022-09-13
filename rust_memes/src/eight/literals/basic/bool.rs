use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::common::AccessibleValue;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::{Literal};
use log::debug;
use crate::eight::values::{BasicValue, Value};

#[derive(Debug)]
pub struct BoolVal {
    pub(crate) value: bool,
}

impl BasicValue for BoolVal {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Literal(l) => {
                match l {
                    Literal::Boolean(b) => {
                        self.value == b.value
                    },
                    o => {
                        panic!("Runtime exception cannot equate {:?} and {:?}", self, o);
                    }
                }
            },
            o => {
                panic!("Runtime exception cannot equate {:?} and {:?}", self, o);
            }
        }
    }
}

impl AccessibleValue for BoolVal {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self { todo!() }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self { todo!() }
}

impl ParsableLiteral for BoolVal {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        return match context.get() {
            Symbols::LiteralSymb(s) => {
                if s == "true" {
                    context.increment();
                    debug!("Found 'true' bool");
                    Some(Literal::Boolean(BoolVal { value: true }))
                } else if s == "false" {
                    context.increment();
                    debug!("Found 'false' bool");
                    Some(Literal::Boolean(BoolVal { value: false }))
                } else {
                    debug!("Invalid literal for bool '{}'", s);
                    None
                }
            }
            o => {
                debug!("Invalid symbol for bool '{:?}'", o);
                None
            }
        };
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        todo!()
    }
}
