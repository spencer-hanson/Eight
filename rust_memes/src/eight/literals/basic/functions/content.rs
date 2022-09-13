use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::primary::Expression;
use crate::eight::values::Value;
use std::fmt::{Debug, Formatter};


pub enum FuncContent {
    BuiltInFunc(fn(&mut EightInterpreter) -> MemRef),
    Func(Vec<Expression>),
}

impl Debug for FuncContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = "FuncContent(";
        return match self {
            FuncContent::Func(v) => {
                write!(f, "{}{:?}", s, v)
            }
            FuncContent::BuiltInFunc(_) => {
                write!(f, "{}BuiltInFunc)", s)
            }
        };
    }
}
