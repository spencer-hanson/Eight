pub mod compiling;
pub mod logging;
pub mod parsing;
pub mod running;
pub mod tokenizing;

use lazy_static::lazy_static;
use crate::eight::values::Value;
use regex::Regex;

pub trait AccessibleValue {
    // Values accessible by
    // '::' class accessor
    // '.' instance accessor
    fn implicit_cast_to<'a>(val: &'a Value) -> &'a Self;

    fn explicit_cast_to<'a>(val: &'a Value) -> Self;
}

pub fn is_varname_valid(varname: &str) -> bool {
    //TODO Put rules for var names here
    // Also checks function names, since they follow the same rules
    lazy_static! {
        static ref VARNAME_PAT: Regex = Regex::new("^[a-zA-Z]+\\w*$").unwrap();
    }
    return VARNAME_PAT.is_match(varname);
}
