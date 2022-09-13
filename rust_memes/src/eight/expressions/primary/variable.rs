use crate::eight::common::is_varname_valid;
use crate::eight::common::parsing::ast::RelationEntry;
use crate::eight::common::parsing::context::Context;
use crate::eight::common::parsing::{ParsableExpression};
use crate::eight::common::tokenizing::symbols::Symbols;
use crate::eight::expressions::secondary::{parse_secondary_expression, TypedSecondaryExpression};
use crate::eight::common::running::interpreter::{EightInterpreter};
use crate::eight::expressions::primary::{Expression, ExpressionRelations, RunnableExpression};
use crate::eight::expressions::secondary::SecondaryExpression;
use crate::eight::values::ValueTypes;
use log::{debug, trace};


#[derive(Debug)]
pub struct Variable {
    pub(crate) name: String,
    pub value: SecondaryExpression,
}

impl Variable {
    pub fn get_var_type(&self) -> Result<ValueTypes, String> {
        self.value.get_type()
    }
}

impl RunnableExpression for Variable {
    fn run_expr(self, interpreter: &mut EightInterpreter) {
        let v = interpreter.step(self.value);
        let val = interpreter.take_val(v);

        interpreter.add_val_to_frame(
            self.name.clone(),
            val
        );

    }
}

impl ExpressionRelations for Box<Variable> {
    fn get_expr_references(&self) -> Vec<RelationEntry> {
        let mut rr = Vec::new();
        for r in self.value.get_references() {
            rr.push(Box::from(r));
        }

        vec![RelationEntry::new(vec![self.name.clone()], vec![], rr)]
    }
}

impl ParsableExpression for Variable {
    fn parse<'a, 'c>(context: &'c mut Context) -> Option<Expression> {
        debug!("Attempting VariableToken parsing");
        // 'let <name> <op> <val>'

        // Check for let keyword
        match context.get() {
            Symbols::Let => (),
            _ => {
                trace!("No let symbol found in VariableToken");
                return None; // No 'let' keyword detected
            }
        }
        context.increment(); // past 'let'

        // TODO Search for operators here (+=.-=, etc)?

        // Don't increment past delimiter so we can match the equality operator

        match context.get() {
            Symbols::LiteralSymb(name) => {
                // Check variable name
                if !is_varname_valid(name.as_str()) {
                    panic!(
                        "{}",
                        context.get_panic_smessage(format!("Invalid varname '{}'", name.as_str()))
                    );
                }
                context.increment();
                trace!("Found var '{}'", name);

                match context.get() {
                    Symbols::Equal => {
                        trace!("Attempting expr parse");
                        context.increment(); // Increment past equals
                        let next_expr = parse_secondary_expression(context);
                        let end_expr = context.get();

                        // End expr check
                        match end_expr {
                            Symbols::Semicolon => {
                                context.increment();
                                match next_expr.get_type() {
                                    Ok(typ) => {
                                        context.put_var(String::from(&name), typ);
                                    }
                                    Err(s) => {
                                        panic!(
                                            "{}",
                                            context.get_panic_smessage(format!(
                                                "Error getting type of expression, '{}'",
                                                s
                                            ))
                                        );
                                    }
                                }
                                return Some(Expression::VariableTokenExpr(Box::new(Variable {
                                    name,
                                    value: next_expr,
                                })));
                            }
                            _ => {
                                panic!(
                                    "{}",
                                    context.get_panic_message("Expected ';' at end of expression")
                                );
                            }
                        }
                    }
                    f => {
                        panic!("{}", context.get_panic_smessage(format!("Expected assignment operator after 'let' keyword, found '{:?}' instead", f)));
                    }
                }
            }
            _ => {
                panic!(
                    "{}",
                    context.get_panic_message("Expected string literal after let token!")
                );
            }
        }
    }
}
