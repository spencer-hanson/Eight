use crate::eight::common::{AccessibleValue, is_varname_valid};
use crate::eight::common::parsing::context::Context;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::primary::noop::NoOp;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::basic::functions::content::FuncContent;
use crate::eight::literals::basic::functions::signature::FuncSignature;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::secondary::{parse_secondary_expression, SecondaryExpression};
use crate::eight::values::Value;

use std::fmt::Debug;
use log::{debug, trace};

pub mod args;
pub mod content;
pub mod signature;


#[derive(Debug)]
pub struct Func {
    pub(crate) name: String,
    pub(crate) sig: FuncSignature,
    pub(crate) content: FuncContent,
}

impl Func {
    pub fn new(name: String, sig: FuncSignature, cont: FuncContent) -> Self {
        Func {
            name,
            sig,
            content: cont,
        }
    }

    pub fn empty() -> Self {
        Func {
            name: String::new(),
            sig: FuncSignature::empty(),
            content: FuncContent::BuiltInFunc(|i: &mut EightInterpreter| MemRef::empty()),
        }
    }

    pub fn call(&mut self, interpreter: &mut EightInterpreter, mut args: FuncArgs) -> MemRef {
        todo!()
    }
}

impl AccessibleValue for Func {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self {
        todo!()
    }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self {
        todo!()
    }
}

pub fn parse_function_call_args<'a>(context: &mut Context) -> FuncArgs {
    let mut args = Vec::new();

    loop {
        let secexpr: SecondaryExpression = parse_secondary_expression(context);
        args.push(secexpr);

        match context.get() {
            Symbols::Comma => {
                context.increment();
                continue;
            }
            Symbols::ParenClose => {
                context.increment();
                break;
            }
            s => {
                panic!(
                    "{}",
                    context.get_panic_smessage(format!(
                        "Unknown identifier '{:?}' when parsing func args",
                        s.to_str()
                    ))
                );
            }
        }
    }
    return FuncArgs { arglist: args };
}

pub fn parse_function_call<'a>(context: &mut Context) -> Option<(String, FuncArgs)> {
    return match context.get() {
        Symbols::LiteralSymb(func_name) => {
            match context.get() {
                Symbols::ParenOpen => {
                    if !is_varname_valid(func_name.as_str()) {
                        panic!(
                            "{}",
                            context.get_panic_smessage(format!(
                                "Invalid function name '{}' in parsing function call",
                                func_name.as_str()
                            ))
                        );
                    }
                    context.increment();
                    // Past func name literal
                    trace!(
                        "Found literal that could possibly be a function call '{:?}'",
                        func_name
                    );

                    context.increment();
                    Some((func_name, parse_function_call_args(context)))
                }
                _ => {
                    trace!("No '(' found for func");
                    None
                }
            }
        }
        _ => None,
    };
}

pub fn generate_func<'a>(name: String, mut args: FuncArgs, sig: FuncSignature,
                         builtin: fn(&mut EightInterpreter) -> MemRef) -> Result<CallFunc, String> {
    // Make sure args passed in match the func sig
    match sig.match_signature(&args) {
        Ok(_) => (),
        Err(s) => return Err(s),
    }

    let bt = FuncContent::BuiltInFunc(builtin);

    let ff = Func::new(String::from(name), sig.clone(), bt);

    return Ok(CallFunc::new(ff, args));
}
