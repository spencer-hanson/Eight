use std::borrow::BorrowMut;
use crate::eight::expressions::primary::Expression;
use crate::eight::expressions::secondary::{RunnableSecondaryExpression, SecondaryExpression};
use crate::eight::literals::Literal;
use crate::eight::values::Value;
use crate::eight::common::AccessibleValue;
use crate::eight::common::running::memory::frame::Frame;
use crate::eight::common::running::memory::memref::MemRef;
use log::{debug, trace};
use crate::eight::common::running::memory::stack::Stack;
use crate::eight::expressions::primary::noop::NoOp;


pub struct EightInterpreter {
    frames: Vec<Frame>,
    working_memory: Stack,
    empty: Value
}

impl EightInterpreter {
    pub fn new() -> Self {
        let mut frames = Vec::new();
        frames.push(Frame::new());

        EightInterpreter {
            frames,
            working_memory: Stack::new(),
            empty: Value::None(NoOp{})
        }
    }

    pub fn step(&mut self, mut expr: SecondaryExpression) -> MemRef {
        trace!("Stepping '{:?}'", expr);
        let o = expr.run_secondary_expr(self);
        trace!("Step Result -> {:?}", o);
        return o;
    }

    pub fn run(&mut self, mut exprs: Vec<Expression>) {
        let mut frame = Frame::new();
        let l = exprs.len();

        for _ in 0..l {
            let mut ex = exprs.remove(0);
            debug!("Expression being run: '{:?}'", ex);
            ex.run_expr(self);
        }
    }

    pub fn add_val_to_stack(&mut self, val: Value) -> MemRef {
        self.working_memory.insert(val)
    }

    pub fn take_val(&mut self, mem: MemRef) -> Value {
        if mem.is_stack_ref() {
            self.working_memory.take(mem)
        } else if mem.is_frame_ref() {
            self.frames[0].remove(mem.get_as_frame())
        } else {
            Value::None(NoOp{})
        }
    }

    pub fn pop_stack_vals(&mut self, mem: MemRef) {
        if mem.is_stack_ref() {
            self.working_memory.take(mem);
        }
    }

    pub fn add_mem_to_frame(&mut self, mem: MemRef, val: Value) {
        // Add a previously removed frame value back to the frame
        // ignore any stack values
        if mem.is_stack_ref() { return; }
        self.add_val_to_frame(mem.get_as_frame().to_string(), val);
    }

    pub fn new_frame(&mut self) {
        self.frames.insert(0, Frame::new());
    }

    pub fn pop_or_clear_frame(&mut self) {
        if self.frames.len() == 1 {
            self.frames.pop();
            self.frames.push(Frame::new());
        } else {
            self.frames.pop();
        }
    }

    pub fn add_val_to_frame(&mut self, name: String, val: Value) {
        self.frames[0].add_var(name, val);
    }

    pub fn get_val_typed_from_frame<T: AccessibleValue>(&self, name: &str) -> &T {
        self.frames[0].get_typed(name)
    }

    pub fn get_val_from_frame(&self, name: &str) -> MemRef {
        match self.frames[0].contains_var(name) {
            Some(m) => m,
            None => {
                // TODO Runtime exceptions
                panic!("Runtime exception, cannot get var '{:?}' from frame!", name);
            }
        }
    }

    pub fn get_val(&self, mem: &MemRef) -> &Value {
        if mem.is_stack_ref() {
            return self.working_memory.get_ref(mem);
        } else if mem.is_frame_ref() {
            return self.frames[0].get(mem.get_as_frame());
        } else {
            &self.empty
        }
    }

    pub fn get_mut_val_from_frame(&mut self, name: &str) -> &mut Value {
        let m = self.get_val_from_frame(name);
        let s = m.get_as_frame();
        self.frames[0].get_mut(name)
    }
}
