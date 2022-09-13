use crate::eight::Expression;
// use crate::eight::RunnableExpression;

use crate::eight::common::running::interpreter::{EightInterpreter};
use log::debug;

pub fn run(mut exprs: Vec<Vec<Expression>>) {
    debug!("-------RUNNING LOCALLY START--------");

    let mut interpreter = EightInterpreter::new();

    for expr in exprs {
        interpreter.run(expr);
    }
    debug!("-----RUNNING LOCALLY COMPLETE------");
}
