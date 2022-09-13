use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::AccessibleValue;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::basic::functions::signature::FuncSignature;
use crate::eight::literals::basic::functions::{generate_func, parse_function_call};
use crate::eight::literals::basic::string::StringVal;
use crate::eight::literals::Literal;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::values::{Value, ValueTypes};
use log::debug;
use std::process::exit;
use crate::eight::values::namespaces::{NamespaceValue, NamespaceValueTypes};


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
        "csv" => match CSV::create_class_func_call(&name, args) {
            Ok(cf) => {
                return Some(SecondaryExpression::CallResult(cf));
            }
            Err(err_str) => {
                panic!("{}", context.get_panic_smessage(err_str));
            }
        },
        "json" => {
            // TODO Impl this
            println!("matched json, exiting!");
            exit(0);
        }
        o => {
            panic!(
                "{}",
                context.get_panic_smessage(format!(
                    "Invalid member name '{:?}' for namespace 'Database', member not found",
                    o
                ))
            );
        }
    }

    /*if thr_ch == "csv" {

        } else if four_ch == "json" {
            match JSON::run_class_func(context, &name, args) {
                Some(SecondaryExpression::valarativeExpression(dc)) => {
                    return Some(dc);
                }
                x => {
                    panic!("Class function returned non-valarativeExpression '{:?}' on line {} snippet '{}...'",
                           x,
                           context.get_line_no(),
                           context.get_snippet()
                    );
                }
            }
        } else {
            panic!("No module found on line {} snippet '{}...'",
                   context.get_line_no(),
                   context.get_snippet()
            );
        }
    } else {
        panic!("EOF Reached while looking for database class func");
    }
    */
}

#[derive(Debug)]
pub struct CSV {
    filename: String,
}

impl CSV {
    pub fn new(filename: String) -> Self {
        debug!("----TODO Making a CSV here filename:'{:?}'", filename);
        CSV { filename }
    }
}

impl AccessibleValue for CSV {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self { todo!() }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self { todo!() }
}

impl ParsableLiteral for CSV {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        panic!("Should never be here! CSV is not parsable");
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        debug!(
            "CSV Attempting to parse func '{}' with args '{:?}'",
            name, args
        );
        return if name == "csv" {
            let sig = FuncSignature::new_from_value(
                vec![ValueTypes::StringType],
                ValueTypes::NamespaceValType(NamespaceValueTypes::DatabaseCSVType),
            );

            fn builtin<'d>(interpreter: &'d mut EightInterpreter) -> MemRef {
                let filn: &StringVal = interpreter.get_val_typed_from_frame::<StringVal>("$0");
                let val = Value::NamespaceVal(NamespaceValue::DatabaseCSV(CSV::new(filn.value.to_string())));

                interpreter.add_val_to_stack(val)
            }

            generate_func(String::from(name), args, sig, builtin)
        } else {
            Err(format!(
                "Function '{}' not found in the CSV ClassType",
                name
            ))
        };
    }
}

#[derive(Debug)]
pub struct JSON {
    pub(crate) filename: String,
}

impl AccessibleValue for JSON {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self {
        todo!()
    }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self {
        todo!()
    }
}

impl ParsableLiteral for JSON {
    fn parse<'a, 'c>(context: &mut Context) -> Option<Literal> {
        todo!()
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        todo!()
    }
}
