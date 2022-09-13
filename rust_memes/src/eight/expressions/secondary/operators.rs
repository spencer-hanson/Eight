pub mod binary;

use crate::eight::common::parsing::context::Context;
use crate::eight::expressions::secondary::{SecondaryExpression, TransitionExpression};
use core::cmp;
use crate::eight::expressions::secondary::operators::binary::BinaryOperators;
use crate::eight::values::ValueTypes;

#[derive(Debug)]
pub enum AccessorOperator {
    ClassAccessor, // ::
    ValueAccessor, // .
}

pub fn split_lhs_rhs<'a>(context: &mut Context, expr_list: &mut Vec<TransitionExpression>, idx: usize) -> (SecondaryExpression, SecondaryExpression, usize) {
    // println!("Removing rhs, idx: {}", idx);
    let rhs = unwrap_trans_expr(context, expr_list.remove(idx + 1));
    // println!("rhs: {:?}", rhs);
    let c = expr_list.remove(idx);
    // println!("cent: {:?}", c);
    let lhs = unwrap_trans_expr(context, expr_list.remove(idx - 1));
    // println!("lhs: {:?}", lhs);

    let iidx = cmp::max(cmp::min(idx - 1, expr_list.len()), 0);
    return (lhs, rhs, iidx);
}


fn unwrap_trans_expr<'a, 'b>(context: &'a mut Context, expr: TransitionExpression) -> SecondaryExpression {
    match expr {
        TransitionExpression::TransExpr(e) => {
            return e;
        }
        TransitionExpression::InterExpr(ex) => {
            match ex {
                e => {
                    panic!(
                        "{}",
                        context.get_panic_smessage(format!(
                            "Invalid Expression, didn't find value in expr, found {:?}",
                            e
                        ))
                    );
                }
            }
        }
    }
}

pub fn compare_lhs_rhs_type(r_lhs: Result<ValueTypes, String>, r_rhs: Result<ValueTypes, String>, expr_type: &BinaryOperators) -> Result<ValueTypes, String> {
    match r_lhs {
        Ok(lhs) => match r_rhs {
            Ok(rhs) => {
                if lhs == rhs {
                    return Ok(lhs);
                } else {
                    Err(format!(
                        "incompatible types {:?} {:?} for operation {:?}",
                        lhs, rhs, expr_type
                    ))
                }
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
