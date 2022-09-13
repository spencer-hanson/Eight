use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::common::AccessibleValue;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::{Literal};
use crate::eight::values::{BasicValue, Value};


#[derive(Debug)]
pub struct StringVal {
    pub(crate) value: String,
}


impl StringVal {
    pub fn copy(&self) -> Self {
        StringVal {
            value: self.value.clone()
        }
    }
}

impl BasicValue for StringVal {
    fn eq(&self, other: &Value) -> bool {
        match other {
            Value::Literal(l) => {
                match l {
                    Literal::String(b) => {
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

impl AccessibleValue for StringVal {
    fn implicit_cast_to(val: &Value) -> &Self {
        return match val {
            Value::Literal(Literal::String(s)) => s,
            _ => panic!("Runtime Exception: Cannot cast {:?} to String!", val),
        };
    }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self {
        return match val {
            Value::Literal(l) => {
                match l {
                    Literal::String(s) => {
                        StringVal{ value: s.value.clone() }
                    },
                    Literal::Number(n) => {
                        StringVal{ value: n.value.clone().to_string() }
                    },
                    Literal::Boolean(b) => {
                        StringVal{ value: b.value.clone().to_string() }
                    }
                }
            },
            o => {
                StringVal{ value: format!("{:?}", o)}
            }
        }
    }
}

impl ParsableLiteral for StringVal {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        return match context.get() {
            Symbols::StringLiteral(s) => {
                context.increment();
                Some(Literal::String(StringVal { value: s }))
            }
            _ => None,
        };
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        todo!()
    }
}
