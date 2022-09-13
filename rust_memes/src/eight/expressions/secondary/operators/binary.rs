use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::ParsableOperator;
use crate::eight::expressions::secondary::operators::{compare_lhs_rhs_type, split_lhs_rhs};
use crate::eight::expressions::secondary::{RunnableSecondaryExpression, SecondaryExpression, TransitionExpression, TypedSecondaryExpression};
use crate::eight::common::running::interpreter::EightInterpreter;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::literals::basic::bool::BoolVal;
use crate::eight::literals::basic::number::NumberVal;
use crate::eight::literals::basic::string::StringVal;
use crate::eight::literals::Literal;
use crate::eight::values::{Value, ValueTypes};
use crate::eight::values::BasicValue;


#[derive(Debug)]
pub enum BinaryOperators {
    Add,
    Exponent,
    Multiply,
    Divide,
    Modulo,
    Sub,
    EqualityCheck,
}

#[derive(Debug)]
pub struct BinaryOperator {
    pub(crate) rhs: SecondaryExpression,
    pub(crate) lhs: SecondaryExpression,
    pub(crate) op: BinaryOperators,
}

impl BinaryOperator {
    pub fn new(lhs: SecondaryExpression, rhs: SecondaryExpression, op: BinaryOperators) -> Self {
        BinaryOperator {
            rhs,
            lhs,
            op
        }
    }
}

impl RunnableSecondaryExpression for Box<BinaryOperator> {
    fn run_secondary_expr(mut self, interpreter: &mut EightInterpreter) -> MemRef {
        let m1 = interpreter.step(self.lhs);
        let m2 = interpreter.step(self.rhs);

        let mut l1 = interpreter.get_val(&m1);
        let mut r1 = interpreter.get_val(&m2);

        let val = match self.op {
            BinaryOperators::EqualityCheck => {
                Value::Literal(Literal::Boolean(BoolVal{value: l1.eq(r1)}))
            },
            o=> {
                let mut l = l1.get_val::<NumberVal>();
                let mut r = r1.get_val::<NumberVal>();


                match o {
                    BinaryOperators::Add => {
                        Value::Literal(Literal::Number(NumberVal { value: l.value + r.value }))
                    }
                    BinaryOperators::Exponent => {
                        if r.value < 0 {
                            panic!("Runtime Exception: Cannot raise {} to a value less than zero! val: {}", l.value, r.value);
                        }
                        Value::Literal(Literal::Number(NumberVal { value: l.value.pow(r.value as u32) }))
                    }
                    BinaryOperators::Multiply => {
                        Value::Literal(Literal::Number(NumberVal { value: l.value * r.value }))
                    }
                    BinaryOperators::Divide => {
                        Value::Literal(Literal::Number(NumberVal { value: l.value / r.value }))
                    }
                    BinaryOperators::Modulo => {
                        Value::Literal(Literal::Number(NumberVal { value: l.value % r.value }))
                    }
                    BinaryOperators::Sub => {
                        Value::Literal(Literal::Number(NumberVal { value: l.value - r.value }))
                    },
                    _ => {
                        panic!("Shouldn't be here, unknown operator found?");
                    }
                }
            }
        };

        interpreter.pop_stack_vals(m1);
        interpreter.pop_stack_vals(m2);
        return interpreter.add_val_to_stack(val);
    }
}

impl TypedSecondaryExpression for Box<BinaryOperator> {
    fn get_type(&self) -> Result<ValueTypes, String> {
        compare_lhs_rhs_type(self.lhs.get_type(), self.rhs.get_type(), &self.op)
    }

    fn get_references(&self) -> Vec<RelationEntry> {
        SecondaryExpression::combine_references(&self.rhs, &self.lhs)
    }
}

impl ParsableOperator for BinaryOperator {
    fn parse(context: &mut Context, expr_list: &mut Vec<TransitionExpression>, idx: usize, op: BinaryOperators) {
        let (lhs, rhs, iidx) = split_lhs_rhs(context, expr_list, idx);

        let lhst_res: Result<ValueTypes, String> = lhs.get_type();
        let rhst_res: Result<ValueTypes, String> = rhs.get_type();

        let lhs_typ: ValueTypes;
        let rhs_typ: ValueTypes;

        match lhst_res {
            Ok(t) => lhs_typ = t,
            Err(e) => {
                panic!("{}", context.get_panic_smessage(format!("Error in typecheck parsing '{}'", e)));
            }
        }

        match rhst_res {
            Ok(t) => rhs_typ = t,
            Err(e) => {
                panic!("{}", context.get_panic_smessage(format!("Error in typecheck parsing '{}'", e)));
            }
        }

        match &op {
            BinaryOperators::EqualityCheck => {
                if rhs_typ != lhs_typ {
                    context.back_to_linestart(); // Go back to line start for snippet
                    panic!("{}", context.get_panic_smessage(format!("Operator '==' only supports Number typed operands! Found {:?} and {:?}", lhs_typ, rhs_typ)));
                }
            },
            o => {
                match lhs_typ {
                    ValueTypes::NumberType => {
                        match rhs_typ {
                            ValueTypes::NumberType => {},
                            oo => {
                                context.back_to_linestart(); // Go back to line start for snippet
                                panic!("{}", context.get_panic_smessage(format!("Operator '{:?}' only supports Number typed operands! Found {:?} and {:?}", op, o, oo)));
                            }
                        }
                    },
                    oo => {
                        context.back_to_linestart(); // Go back to line start for snippet
                        panic!("{}", context.get_panic_smessage(format!("Operator {:?} only supports Number typed operands! Found {:?} and {:?}", op, o, oo)));
                    }
                }

            }
        }

        expr_list.insert(
            iidx,
            TransitionExpression::TransExpr(
                SecondaryExpression::BinaryOperation(Box::from(BinaryOperator::new(lhs, rhs, op)))
            ),
        );
    }
}

pub fn sym_to_binop(sym: &Symbols, context: &Context) -> BinaryOperators {
    return match sym {
        Symbols::Exponent => { BinaryOperators::Exponent }
        Symbols::Multiply => { BinaryOperators::Multiply }
        Symbols::Divide => { BinaryOperators::Divide }
        Symbols::Modulus => { BinaryOperators::Modulo }
        Symbols::Add => { BinaryOperators::Add }
        Symbols::Sub => { BinaryOperators::Sub }
        Symbols::EqualityCheck => { BinaryOperators::EqualityCheck }
        other => {
            panic!("{}", context.get_panic_smessage(format!("Invalid symbol '{:?}' expected operator", other)));
        }
    };
}

pub fn binop_to_sym(binop: &BinaryOperators) -> Symbols {
    return match binop {
        BinaryOperators::Add => { Symbols::Add }
        BinaryOperators::Exponent => { Symbols::Exponent }
        BinaryOperators::Multiply => { Symbols::Multiply }
        BinaryOperators::Divide => { Symbols::Divide }
        BinaryOperators::Modulo => { Symbols::Modulus }
        BinaryOperators::Sub => { Symbols::Sub }
        BinaryOperators::EqualityCheck => { Symbols::EqualityCheck }
    };
}
