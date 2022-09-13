use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableLiteral;
use crate::eight::common::AccessibleValue;
use crate::eight::values::namespaces::algorithms::Algorithm;
use crate::eight::expressions::secondary::callfunc::CallFunc;
use crate::eight::literals::basic::functions::args::FuncArgs;
use crate::eight::literals::Literal;
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::values::Value;

pub fn parse_class_func(context: &mut Context) -> Option<SecondaryExpression> {
    return None;
    // let name = consume_until_delimiter(code, &mut *index, vec!["::", "("]);
    // if name == "new" {
    //     return Model::parse(index, code);
    // }
    // panic!("{}", context.get_panic_message(""Unknown namespace Database::'{}'", name));
}

#[derive(Debug)]
pub struct Model {
    name: String,
    algorithm: Algorithm,
}

impl AccessibleValue for Model {
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self {
        todo!()
    }

    fn explicit_cast_to<'a>(val: &'a Value) -> Self {
        todo!()
    }
}

impl ParsableLiteral for Model {
    fn parse<'a>(context: &mut Context) -> Option<Literal> {
        todo!()
    }

    fn create_class_func_call<'a>(name: &str, args: FuncArgs) -> Result<CallFunc, String> {
        todo!()
    }
}
