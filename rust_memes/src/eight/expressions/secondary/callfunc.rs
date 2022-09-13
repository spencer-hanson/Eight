use std::collections::HashMap;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::AccessibleValue;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::basic::functions::Func;
use log::{debug, trace};
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::secondary::{RunnableSecondaryExpression, TypedSecondaryExpression};
use crate::eight::literals::basic::functions::content::FuncContent;
use crate::eight::values::{Value, ValueTypes};

#[derive(Debug)]
pub struct CallFunc {
    func: Func,
    pub(crate) args: FuncArgs,
}

impl CallFunc {
    pub fn get_output_type(&self) -> ValueTypes {
        self.func.sig.get_output_type()
    }

    pub fn new(f: Func, args: FuncArgs) -> Self {
        CallFunc { func: f, args }
    }
}

impl TypedSecondaryExpression for CallFunc {
    fn get_type(&self) -> Result<ValueTypes, String> {
        Ok(self.get_output_type())
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        let mut r = RelationEntry::new(vec![], vec![], vec![]);
        for arg in self.args.get_arglist() {
            for refr in arg.get_references() {
                r.entries.push(Box::from(refr));
            }
        }

        debug!("Callfunc ENTRS {:?}", r);
        return vec![r];
    }
}

impl RunnableSecondaryExpression for CallFunc {
    fn run_secondary_expr(mut self, interpreter: &mut EightInterpreter) -> MemRef {
        let mut f: Func = std::mem::replace(
            &mut self.func,
            Func::empty(),
        );

        let mut args: FuncArgs = std::mem::replace(
            &mut self.args,
            FuncArgs::empty(),
        );

        return match f.content {
            FuncContent::Func(exprs) => {
                // TODO impl user funcs
                panic!("TODO user defined functions not implemented yet");
            },
            FuncContent::BuiltInFunc(func) => {
                trace!("Running built in func");
                let mut fn_vars: Vec<(String, MemRef)> = Vec::new();
                let mut fn_args: Vec<(String, Value)> = Vec::new();

                for idx in 0..args.arglist.len() {
                    let mut s = args.grab_arg(idx);
                    let d = interpreter.step( s);
                    trace!("Parsing arg #{} -> '{:?}'", idx, d);
                    let name = format!("${}", idx);
                    fn_vars.push((name.clone(), d.copy()));

                    let val = interpreter.take_val(d);
                    fn_args.push((name, val));
                }
                trace!("Creating new frame for func, adding args");
                interpreter.new_frame();
                for (n, v) in fn_args {
                    interpreter.add_val_to_frame(
                        n,
                        v
                    );
                }
                trace!("Running func");
                let o = func(interpreter);
                trace!("Grabbing args passed by value");
                let mut mod_args: Vec<(MemRef, Value)> = Vec::new();

                for (nam, mem) in fn_vars {
                    let m = interpreter.get_val_from_frame(nam.as_str());
                    let v = interpreter.take_val(m);
                    mod_args.push((mem, v));
                }
                trace!("Popping func frame");
                interpreter.pop_or_clear_frame();

                trace!("Adding back arg values");
                for (mem, val) in mod_args {
                    interpreter.add_mem_to_frame(mem, val);
                }

                return o;
            }
        };
    }
}
