use std::collections::HashMap;
use crate::eight::common::AccessibleValue;
use crate::eight::common::running::memory::memref::MemRef;
use crate::eight::values::Value;

#[derive(Debug)]
pub struct VarTable {
    pub lookup: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Frame {
    pub vartable: VarTable,
}

impl Frame {
    pub fn new() -> Frame {
        return Frame {
            vartable: VarTable::new(),
        };
    }

    pub fn contains_var(&self, name: &str) -> Option<MemRef> {
        return if self.vartable.lookup.contains_key(name) {
            Some(MemRef::frame(name.to_string()))
        } else {
            None
        }
    }

    pub fn get_typed<T: AccessibleValue>(&self, name: &str) -> &T {
        T::implicit_cast_to(self.get(&name))
    }

    pub fn get_mut(&mut self, name: &str) -> &mut Value {
        self.lookup_var(name);
        self.vartable.lookup.get_mut(name).unwrap()
    }

    pub fn get(&self, name: &str) -> &Value {
        self.lookup_var(name);
        return self.vartable.lookup.get(name).unwrap();
    }

    pub fn remove(&mut self, name: &str) -> Value {
        self.vartable.lookup.remove(name).unwrap()
    }

    fn lookup_var(&self, name: &str) {
        if self.vartable.lookup.contains_key(name) {
            return;
        } else {
            println!("Current vars in frame: {:?}", self.vartable);
            panic!("Unknown var '{}' lookup failed!", name);
        }
    }

    pub fn add_var(&mut self, name: String, value: Value) -> bool {
        return self.vartable.add_var(name, value);
    }
}

impl VarTable {
    pub fn new() -> VarTable {
        return VarTable {
            lookup: HashMap::new(),
        };
    }

    pub fn get_varnames(&self) -> Vec<String> {
        let mut v = Vec::new();
        let k = self.lookup.keys();
        for key in k {
            v.push(String::from(key));
        }
        return v;
    }

    pub fn add_var(&mut self, name: String, value: Value) -> bool {
        return match self.lookup.get(&name) {
            Some(_) => false,
            None => {
                self.lookup.insert(name, value);
                true
            }
        };
    }
}
