use crate::eight::common::AccessibleValue;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::expressions::secondary::{RunnableSecondaryExpression, SecondaryExpression, TypedSecondaryExpression};
use crate::eight::values::{Value, ValueTypes};
use log::{debug, trace};


#[derive(Debug)]
pub struct VariableTokenRef {
    pub(crate) varname: String,
    pub(crate) typ: ValueTypes,
}

impl AccessibleValue for VariableTokenRef {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self { todo!() }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self { todo!() }
}

impl TypedSecondaryExpression for VariableTokenRef {
    fn get_type(&self) -> Result<ValueTypes, String> {
        Ok(self.typ.clone())
    }
    fn get_references(&self) -> Vec<RelationEntry> {
        vec![RelationEntry::new(
            vec![],
            vec![self.varname.clone()],
            vec![],
        )]
    }
}

impl RunnableSecondaryExpression for VariableTokenRef {
    fn run_secondary_expr(mut self, interpreter: &mut EightInterpreter) -> MemRef {
        interpreter.get_val_from_frame(self.varname.as_str())
    }
}

impl VariableTokenRef {
    pub fn parse<'a>(context: &mut Context) -> Option<SecondaryExpression> {
        return match context.get() {
            Symbols::LiteralSymb(v) => {
                debug!("found literal '{}'", v);
                for vv in context.get_varnames() {
                    trace!("Checking '{}' == '{}'", vv, v);
                    if vv == v {
                        context.increment();
                        return Some(SecondaryExpression::Reference(VariableTokenRef {
                            varname: v.clone(),
                            typ: context.get_vartype(v.as_str()),
                        }));
                    }
                }
                None
            }
            o => {
                debug!("Found other symb '{:?}'", o);
                None
            }
        };
    }
}
