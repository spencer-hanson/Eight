use crate::eight::common::parsing::context::Context;
use crate::eight::common::tokenizing::symbols::{SymbolType, Symbols};

pub fn slice_str(s: &str, start: usize, endi: usize) -> &str {
    if endi == usize::MAX {
        return &s[start..];
    }

    let end: usize = endi as usize;
    let l = s.len();
    if start > l {
        return "";
    } else if end > l {
        return &s[start..];
    }

    return &s[start..end];
}

pub fn consume_until_symboltype(context: &mut Context, symtyp: SymbolType) {
    let syms = Symbols::get_symbols_by_type(symtyp);


    if !context.has_next() {
        panic!(
            "{}",
            context.get_panic_smessage(format!(
                "Reached EOF Looking for SymbolType '{:?}'",
                symtyp
            ))
        );
    }

    loop {
        for sym in &syms {
            if &context.get_raw() == sym {
                context.increment(); // Increment past matching symbol
                return;
            }
        }
        if !context.has_next() {
            panic!(
                "{}",
                context.get_panic_smessage(format!(
                    "Reached EOF Looking for SymbolType '{:?}'",
                    symtyp
                ))
            );
        }
        context.increment();
    }
}

pub fn consume_until_symbol(context: &mut Context, sym: Symbols) {
    loop {
        if context.get() == sym {
            context.increment(); // Increment past matching symbol
            break;
        } else {
            context.increment();
        }
    }
}
