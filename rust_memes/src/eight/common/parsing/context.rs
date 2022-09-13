use crate::eight::common::tokenizing::symbols::{SymbolType, Symbols};
use log::{debug, trace};
use std::cmp::{max, min};
use std::collections::HashMap;
use crate::eight::values::ValueTypes;

#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) raw_code: String,
    n: &'a usize, // todo remove temp val to keep lifetime annotations?
    index: usize,
    symbols: Vec<Symbols>,
    pub(crate) vartable: HashMap<String, ValueTypes>,
}

impl<'a> Context<'a> {
    pub fn get_vartype(&self, name: &str) -> ValueTypes {
        self.vartable.get(name).unwrap().clone()
    }

    pub fn print_symbols_current(&self) {
        let mut prev = 0;
        if self.index < 2 {
            prev = 0;
        } else {
            prev = max(self.index - 2, 0);
        }

        let end = min(self.symbols.len(), self.index + 4);
        for i in prev..end {
            if i == self.index {
                trace!("cur:{:?}, ", self.symbols[i]);
            } else {
                trace!("{}:{:?}, ", i, self.symbols[i]);
            }
        }
        trace!("...\n");
    }

    pub fn print_vartable(&self) {
        for (k, v) in self.vartable.iter() {
            trace!("Var: {:?} -> {:?}", k, v);
        }
    }

    pub fn new(raw_code: String, symbols: Vec<Symbols>, index: usize) -> Context<'a> {
        return Context {
            raw_code,
            n: &5, // TODO Remove temp lifetime placeholder?
            index,
            symbols,
            vartable: HashMap::new(),
        };
    }

    pub fn put_var(&mut self, s: String, typ: ValueTypes) {
        self.vartable.insert(s, typ);
    }

    pub fn get_varnames(&self) -> Vec<String> {
        let mut v = Vec::new();
        let k = self.vartable.keys();
        for key in k {
            v.push(String::from(key));
        }
        return v;
    }

    pub fn get_raw(&self) -> Symbols {
        if self.index >= self.symbols.len() {
            panic!("Reached EOF in get_raw!");
        }
        return self.symbols[self.index].clone();
    }

    pub fn back_to_linestart(&mut self) {
        while self.index > 0 {
            let cur_sym = &self.symbols[self.index].clone();
            if cur_sym == &Symbols::NewLine || cur_sym == &Symbols::WindowsNewLine {
                break;
            } else {
                self.index -= 1;
            }
        }
        self.increment(); // Increment just past the newline to be on the beginning of the next line
    }

    // pub fn back_to_nonwhitespace(&mut self) {
    //     let whitesp_syms = Symbols::get_symbols_by_type(SymbolType::Whitespace);
    //     self.decrement(); // Initially move back one space
    //
    //     // Continue to move back until a non-whitespace symbol is found
    //     while self.index > 0 {
    //         let cur_sym = &self.symbols[self.index].clone();
    //         for sym in &whitesp_syms {
    //             if cur_sym == sym {
    //                 self.decrement();
    //                 break;
    //             }
    //         }
    //         return;
    //     }
    // }

    pub fn get_index(&self) -> usize {
        return self.index;
    }

    pub fn jump(&mut self, idx: usize) {
        self.index = idx;
    }

    pub fn get_safe_multiple(&mut self, len: usize) -> Vec<Result<Symbols, String>> {
        // Update 9-13-22 probably fixed? but leaving comment for now
        // todo!()
        // Make sure you remember that after each increment there is whitespace between symbols,
        // and with multiple symbols there will be excess needed to account for
        // get_safe_multiple(2) -> cur:wsp wsp A wsp wsp B
        // would need to pretend to increment to 'A' then do the same pretend increment to 'B'
        // WILL BREAK increment TODO figure this out
        // could have separate symbol stream for non-whitespace, and map indexes of symbols to indexes in the orig
        // symbol stream, then use orig symbol stream to determine line number

        let mut gotten_count = 0;
        let mut last_found_idx = 0;
        let whitesp_syms = Symbols::get_symbols_by_type(SymbolType::Whitespace);
        let mut gotten_symbs = Vec::new();
        let mut whitespace_count = 0;

        let start_line = self.get_line_no();
        let snipp = self.get_snippet();

        while gotten_count < len {
            if self.index + whitespace_count + gotten_count >= self.symbols.len() {
                gotten_symbs.push(Err(format!(
                    "Reached EOF while looking for non-whitespace token! Started on line {} snippet: '{}'",
                    start_line, snipp
                )));
                return gotten_symbs;
            }

            while self.index + whitespace_count + gotten_count < self.symbols.len() && gotten_count < len {
                let cur_sym = &self.symbols[self.index + whitespace_count + gotten_count].clone();
                let mut found_whitespace = false;

                for sym in &whitesp_syms {
                    if cur_sym == sym {
                        // println!("Cur symb {:?} matched whitespace", cur_sym);
                        found_whitespace = true;
                        break;
                    }
                }

                if found_whitespace {
                    // println!("Incrementing over whitespace");
                    whitespace_count += 1;
                } else {
                    // println!("Returning {:?}", cur_sym);
                    gotten_symbs.push(Ok(cur_sym.clone()));
                    gotten_count += 1;
                }
            }
        }

        return gotten_symbs;
    }

    pub fn get_safe(&mut self) -> Result<Symbols, String> {
        self.get_safe_multiple(1).remove(0)
    }

    pub fn get_multiple(&mut self, len: usize) -> Vec<Symbols> {
        let mut v = Vec::new();
        for val in self.get_safe_multiple(len) {
            match val {
                Ok(s) => v.push(s),
                Err(s) => {
                    panic!("{}", self.get_panic_smessage(s))
                }
            }
        }
        return v;
    }

    pub fn get(&mut self) -> Symbols {
        self.get_multiple(1).remove(0)
    }

    pub fn has_next_nonwhitespace(&self) -> bool {
        return if self.has_next() { true } else { false };
    }

    pub fn has_next(&self) -> bool {
        if self.index + 1 > self.symbols.len() {
            return false;
        }
        return true;
    }

    pub fn increment_raw(&mut self) {
        self.index += 1;
    }

    pub fn increment(&mut self) {
        self.increment_raw();
        let whitesp_syms = Symbols::get_symbols_by_type(SymbolType::Whitespace);
        loop {
            if self.index == self.symbols.len() {
                return; // EOF reached
            }
            for sym in &whitesp_syms {
                if &self.symbols[self.index].clone() == sym {
                    self.increment_raw();
                    break;
                }
            }
            return;
        }
    }

    pub fn get_panic_smessage(&self, msg: String) -> String {
        return self.get_panic_message(msg.as_str());
    }

    pub fn get_panic_message(&self, msg: &str) -> String {
        trace!("---------PANIC---------");
        trace!("--VARTABLE AT PANIC--");
        self.print_vartable();

        trace!("--SYMBOLS AT PANIC--");
        self.print_symbols_current();

        let mut message = String::new();
        message.push_str("\n--\n");
        message.push_str(format!("Parsing failed: {:?}\n", msg).as_str());
        message.push_str(format!("On line: {:?}\n", self.get_line_no()).as_str());
        message.push_str(format!("Snippet: \n`{}`", self.get_snippet()).as_str());
        message.push_str("\n--\n");
        return message;
    }

    pub fn get_line_no(&self) -> i32 {
        let mut line_count = 0;
        let whitespace_syms =
            Symbols::sort_by_longest(Symbols::get_symbols_by_type(SymbolType::Newline));

        for idx in 0..self.index {
            for sym_idx in 0..whitespace_syms.len() {
                if self.symbols[idx] == whitespace_syms[sym_idx] {
                    line_count += 1;
                    break;
                }
            }
        }
        return line_count + 1; // line count starts at 0
    }

    pub fn get_snippet(&self) -> String {
        let mut symbs = Vec::new();
        for idx in 0..min(self.symbols.len() - self.index, 5) {
            symbs.push(self.symbols[self.index + idx].clone());
        }
        let mut st = String::new();

        for symb in symbs {
            st.push_str(symb.to_str().as_str());
        }
        return st;
    }

}
