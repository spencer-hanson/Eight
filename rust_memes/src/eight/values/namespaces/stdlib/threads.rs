use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::basic::functions::{generate_func, parse_function_call};
use crate::eight::literals::basic::functions::signature::FuncSignature;
use crate::eight::literals::basic::number::NumberVal;
use crate::eight::literals::Literal;
use crate::eight::values::namespaces::{NamespaceValue, NamespaceValueTypes};
use crate::eight::values::{Value, ValueTypes};

pub fn parse_class_func(context: &mut Context) -> Option<SecondaryExpression> {
    let (name, args) = match parse_function_call(context) {
        Some(n) => n,
        None => {
            let sym = context.get().to_str();
            panic!(
                "{}",
                context.get_panic_smessage(format!("Invalid namespace member {:?}", sym))
            );
        }
    };

    match name.as_str() {
        "new" => {
            match ThreadPool::create_class_func_call(&name, args) {
                Ok(cf) => return Some(SecondaryExpression::CallResult(cf)),
                Err(err_str) => {
                    panic!("{}", context.get_panic_smessage(err_str));
                }
            }
        },
        o => {
            panic!("{}", context.get_panic_smessage(format!(
                    "Invalid member name '{:?}' for namespace 'ThreadPool', member not found", o)));
        }
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    thread_count: u8
}

impl ParsableLiteral for ThreadPool {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        todo!()
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        return if name == "new" {
            let sig = FuncSignature::new_from_value(
                vec![ValueTypes::NumberType],
                ValueTypes::NamespaceValType(NamespaceValueTypes::ThreadPoolType)
            );

            fn builtin<'d>(interpreter: &'d mut EightInterpreter) -> MemRef {
                let thread_count: &NumberVal = interpreter.get_val_typed_from_frame::<NumberVal>("$0");
                let val = Value::NamespaceVal(NamespaceValue::ThreadPool(ThreadPool::new(thread_count.value)));
                interpreter.add_val_to_stack(val)
            }

            return generate_func(String::from(name), args, sig, builtin);
        } else {
            Err(format!("Function '{}' not found in the ThreadPool Classtype", name))
        }
    }
}

impl ThreadPool {
    pub fn new(count: i32) -> Self {
        ThreadPool {
            thread_count: count as u8
        }
    }
}