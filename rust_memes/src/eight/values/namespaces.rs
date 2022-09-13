use crate::eight::common::parsing::context::Context;
use crate::eight::common::tokenizing::symbols::Symbols;
use log::debug;
use std::collections::HashMap;
use crate::eight::expressions::secondary::SecondaryExpression;
use lazy_static::lazy_static;
use crate::eight::values::namespaces::database::{CSV, JSON};
use crate::eight::values::namespaces::model::Model;
use crate::eight::values::namespaces::NamespaceValueTypes::{DatabaseCSVType, DatabaseJSONType};
use crate::eight::values::namespaces::stdlib::threads::ThreadPool;

pub mod database;
pub mod model;
pub mod algorithms;
pub mod stdlib;

type NamespaceFunc = fn(&mut Context) -> Option<SecondaryExpression>;

#[derive(Debug)]
pub enum NamespaceValue{
    DatabaseJSON(JSON),
    DatabaseCSV(CSV),
    Model(Model),
    ThreadPool(ThreadPool)
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum NamespaceValueTypes {
    DatabaseJSONType,
    DatabaseCSVType,
    ModelType,
    ThreadPoolType
}

impl NamespaceValue {
    pub fn convert_to_type(val: &NamespaceValue) -> NamespaceValueTypes {
        return match val {
            NamespaceValue::DatabaseJSON(_) => NamespaceValueTypes::DatabaseJSONType,
            NamespaceValue::DatabaseCSV(_) => NamespaceValueTypes::DatabaseCSVType,
            NamespaceValue::Model(_) => NamespaceValueTypes::ModelType,
            NamespaceValue::ThreadPool(_) => NamespaceValueTypes::ThreadPoolType
        }
    }
}

pub fn parse_namespace<'a>(context: &mut Context) -> Option<SecondaryExpression> {
    let mut namespaces: HashMap<&str, NamespaceFunc> = HashMap::new();

    namespaces.insert("Database", database::parse_class_func);
    namespaces.insert("Model", model::parse_class_func);
    namespaces.insert("ThreadPool", stdlib::threads::parse_class_func);

    return match context.get() {
        Symbols::LiteralSymb(cchunk) => {
            for (namespace, _) in namespaces.iter() {
                debug!("Checking namespace {:?}", namespace);

                if cchunk.eq(namespace) {
                    context.increment(); // increment past literal

                    match context.get() {
                        Symbols::ClassAccessor => {
                            context.increment();

                            let lookup = namespaces.get(namespace).unwrap();
                            return lookup(context);
                        }
                        Symbols::ValueAccessor => {
                            panic!("{}", context.get_panic_message("Wrong accessor operator '.' for classtype access, use '::' instead."));
                        }
                        o => {
                            panic!(
                                "{}",
                                context.get_panic_smessage(format!(
                                    "Expected '::' found '{:?}'",
                                    o.to_str()
                                ))
                            );
                        }
                    }
                }
            }
            debug!("Value '{}' didn't match any known namespaced var", cchunk);
            None
        }
        _ => None,
    };
}
