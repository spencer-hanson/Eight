use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::util::slice_str;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::common::AccessibleValue;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::{Literal};
use log::debug;
use crate::eight::values::{BasicValue, Value};

#[derive(Debug)]
pub struct NumberVal {
    pub(crate) value: i32,
}

impl NumberVal {
    pub fn copy(&self) -> Self {
        NumberVal {
            value: self.value.clone()
        }
    }
}

impl BasicValue for NumberVal {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Literal(l) => {
                match l {
                    Literal::Number(b) => {
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

impl AccessibleValue for NumberVal {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self {
        match val {
            Value::Literal(Literal::Number(n)) => {
                n
            },
            o => {
                //TODO Runtime exceptions
                panic!("Runtime exception: unable to implicitly cast {:?} to a number value", o);
            }
        }
    }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self { todo!() }
}

impl ParsableLiteral for NumberVal {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        return match context.get() {
            Symbols::LiteralSymb(snippet) => {
                for idx in 0..snippet.len() {
                    let one_char = slice_str(&snippet, idx, idx + 1);

                    if one_char == "0"
                        || one_char == "1"
                        || one_char == "2"
                        || one_char == "3"
                        || one_char == "4"
                        || one_char == "5"
                        || one_char == "6"
                        || one_char == "7"
                        || one_char == "8"
                        || one_char == "9"
                    {
                    } else {
                        // println!("Found non-numeric character in literal '{}'!", one_char);
                        return None;
                    }
                }

                let num = &snippet.parse::<i32>();
                return match num {
                    Ok(x) => {
                        debug!("Found num '{}'", x);
                        context.increment();
                        Some(Literal::Number(NumberVal { value: *x }))
                    }
                    _ => None,
                };
            }
            _ => None,
        };
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        todo!()
    }
}
