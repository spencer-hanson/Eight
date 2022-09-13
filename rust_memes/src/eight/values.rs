use crate::eight::common::AccessibleValue;
use crate::eight::expressions::primary::noop::NoOp;
use crate::eight::literals::basic::functions::Func;
use crate::eight::literals::basic::functions::signature::FuncSignature;
use crate::eight::literals::Literal;
use namespaces::database::{CSV, JSON};
use namespaces::model::Model;
use crate::eight::literals::basic::list::List;
use crate::eight::values::namespaces::{NamespaceValue, NamespaceValueTypes};

pub mod namespaces;

pub trait BasicValue {
    fn eq(&self, other: &Value) -> bool;
}

#[derive(Debug)]
pub enum Value {
    Literal(Literal),
    NamespaceVal(NamespaceValue),
    ListVal(List),
    Function(Func),
    None(NoOp),
}

impl BasicValue for Value {
    fn eq(&self, other: &Value) -> bool {
        match self {
            Value::Literal(l) => {
                l.eq(other)
            },
            o => {
                panic!("Runtime exception: unable to equate {:?} and {:?}", self, o);
            }
        }
    }
}

impl Value {
    pub fn gen_noop() -> Self {
        Value::None(NoOp {})
    }

    pub fn get_val<T: AccessibleValue>(&self) -> &T {
        T::implicit_cast_to(self)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ValueTypes {
    BooleanType,
    StringType,
    NumberType,
    NamespaceValType(NamespaceValueTypes),
    ListType(Box<ValueTypes>),
    FuncType(Box<FuncSignature>),
    NoneType,
}

impl ValueTypes {
    pub fn convert_to_type(val: &Value) -> ValueTypes {
        return match val {
            Value::Literal(Literal::String(_)) => ValueTypes::StringType,
            Value::Literal(Literal::Number(_)) => ValueTypes::NumberType,
            Value::Literal(Literal::Boolean(_)) => ValueTypes::BooleanType,
            Value::NamespaceVal(v) => ValueTypes::NamespaceValType(NamespaceValue::convert_to_type(v)),
            Value::Function(f) => ValueTypes::FuncType(Box::from(f.sig.clone())),
            Value::None(_) => ValueTypes::NoneType,
            Value::ListVal(v) => ValueTypes::ListType(Box::from(v.typ.clone()))
        };
    }
}
