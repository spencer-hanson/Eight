use std::collections::HashMap;
use crate::eight::common::running::memory::memref::{MemData, MemRef};
use crate::eight::expressions::primary::noop::NoOp;
use crate::eight::values::Value;
use log::warn;


#[derive(Debug)]
pub struct Stack {
    data: HashMap<i32, Value>,
    empty: Value,
    free_spaces: [bool; 1000] // TODO more / configurable free spaces?
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: HashMap::new(),
            empty: Value::None(NoOp {}),
            free_spaces: [true; 1000] // TODO increase / configurable spaces?
        }
    }

    pub fn get_lowest_free_index(&self) -> i32 {
        for idx in 0..self.free_spaces.len() {
            if self.free_spaces[idx] {
                return idx as i32;
            }
        }
        // TODO Runtime exceptions
        panic!("Unable to find free space in working stack memory!");
    }

    fn alloc_slot(&mut self, val: Value) -> i32 {
        let idx = self.get_lowest_free_index();
        self.free_spaces[idx as usize] = false;
        self.data.insert(idx, val);
        return idx;
    }

    fn dealloc_slot(&mut self, idx: i32) -> Value {
        if self.free_spaces[idx as usize] {
            warn!("Attempted dealloc of empty slot {:?}", idx);
        }
        let v = self.data.remove(&idx);
        self.free_spaces[idx as usize] = true;
        return v.unwrap();
    }


    pub fn insert(&mut self, val: Value) -> MemRef {
        MemRef::stack(self.alloc_slot(val))
    }

    pub fn take(&mut self, mem: MemRef) -> Value {
        let idx = mem.get_as_stack();
        self.dealloc_slot(*idx)
    }

    pub fn get_ref(&self, mem: &MemRef) -> &Value {
        let idx = mem.get_as_stack();
        match self.data.get(idx) {
            Some(v) => v,
            None => {
                // TODO Runtime exceptions
                panic!("Invalid stack memory access! Stack idx {:?} doesn't exist", idx);
            }
        }
    }
}
