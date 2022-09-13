use crate::eight::common::parsing::context::Context;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::values::ValueTypes;
use crate::eight::expressions::secondary::TypedSecondaryExpression;


#[derive(Eq, PartialEq, Debug, Clone)]
pub struct FuncSignature {
    arglist: Vec<ValueTypes>,
    output: ValueTypes,
}

impl FuncSignature {
    pub fn empty() -> Self {
        FuncSignature {
            arglist: vec![],
            output: ValueTypes::NoneType,
        }
    }
    pub fn clone(&self) -> Self {
        FuncSignature {
            arglist: self.arglist.clone(),
            output: self.output.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.arglist = source.arglist.clone();
        self.output = source.output.clone();
    }

    pub fn get_output_type(&self) -> ValueTypes {
        self.output.clone()
    }

    pub fn new(input: Vec<ValueTypes>, output: ValueTypes) -> Self {
        FuncSignature {
            arglist: input,
            output,
        }
    }

    pub fn new_from_value(data: Vec<ValueTypes>, output: ValueTypes) -> Self {
        let mut v = Vec::new();
        for d in data {
            v.push(d);
        }

        FuncSignature { arglist: v, output }
    }

    pub fn match_output(&self, context: &mut Context, output: ValueTypes) {
        // Match output
        if output != self.output {
            panic!(
                "{}",
                context.get_panic_smessage(format!(
                    "Function call output type '{:?}' doesn't match expected '{:?}'",
                    output, self.output
                ))
            );
        }
        println!("Matched output '{:?}' == '{:?}'", output, self.output);
    }

    pub fn match_signature(&self, args: &FuncArgs) -> Result<(), String> {
        // Match signature or error
        let arglist = args.get_arglist();

        // Match arg length
        if arglist.len() != self.arglist.len() {
            return Err(format!(
                "Invalid function call, does not match function signature"
            ));
        }

        // Match arguments
        let mut i = 0;
        for arg in arglist {
            let sig_arg = self.arglist.get(i).unwrap();
            let argtyp = arg.get_type();
            match argtyp {
                Ok(m) => {
                    let mm = &m == sig_arg;
                    println!("matched '{:?}' == '{:?}' -> {:?}", &m, sig_arg, mm);
                    if mm {
                        i += 1;
                        continue;
                    } else {
                        return Err(format!("Invalid function call, argument {} does not match signature. Given '{:?}' Expected '{:?}'",
                                           i,
                                           m,
                                           sig_arg
                        ));
                    }
                }
                Err(s) => {
                    return Err(format!("Invalid type in arg! Err: '{}'", s));
                }
            }
        }

        Ok(())
    }
}
