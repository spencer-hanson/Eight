use crate::eight::common::parsing::util::slice_str;
use crate::eight::common::tokenizing::symbols::Symbols;
use log::{debug, info, trace};

pub fn check_symbol(string: &str, symbol: Symbols) -> Option<Symbols> {
    let sym = symbol.to_str();
    let sym_len = sym.len();

    let sliced = slice_str(string, 0, sym_len);

    return if sliced == sym { Some(symbol) } else { None };
}

pub fn get_possible_symbols(s: &str, idx: usize, symbols: &Vec<Symbols>) -> Vec<Symbols> {
    let mut possible_syms = Vec::new();

    for sym in symbols {
        let snip = slice_str(s, idx, usize::MAX);
        match check_symbol(snip, sym.clone()) {
            Some(s) => {
                possible_syms.push(s);
            }
            None => (),
        }
    }
    return possible_syms;
}

pub fn parse(s: String) -> Vec<Symbols> {
    info!("----------LEXING START----------");
    trace!("Starting lexing parsing on '{:?}'", s);
    info!("Found {} chars", s.len());

    let symbols = Symbols::get_all();
    let mut parsed = Vec::new();
    let mut outer_done = false;
    let mut idx: usize = 0;
    while !outer_done {
        if idx >= s.len() {
            break;
        }
        let mut possible_syms = get_possible_symbols(s.as_str(), idx, &symbols);
        if possible_syms.len() == 0 {
            let mut done = false;
            let mut collect_offset = 0;
            while !done {
                if idx + collect_offset == s.len() {
                    done = true;
                    outer_done = true;
                    collect_offset -= 1;
                }

                let next = get_possible_symbols(s.as_str(), idx + collect_offset, &symbols);
                if next.len() == 0 {
                    collect_offset += 1;
                    continue;
                } else {
                    done = true;
                    parsed.push(Symbols::LiteralSymb(String::from(slice_str(
                        s.as_str(),
                        idx,
                        idx + collect_offset,
                    ))));
                    idx += collect_offset;
                }
            }
        } else {
            trace!("Possible symbols: {:?}", possible_syms);
            let mut longest_symbol_len = 0;
            let mut sym_idx = 0;
            for i in 0..possible_syms.len() {
                let sym_len = possible_syms[i].to_str().len();

                if sym_len > longest_symbol_len {
                    longest_symbol_len = sym_len;
                    sym_idx = i;
                }
            }
            idx += longest_symbol_len;
            let chosen_sym = possible_syms.remove(sym_idx);
            match chosen_sym {
                Symbols::Quote => {
                    let mut quote_offset = 0;
                    loop {
                        if idx + quote_offset >= s.len() {
                            panic!("Reached EOF while parsing a quote! Idx: '{}'", idx);
                            // TODO Error messaging during lexing
                        }
                        let sym_str = slice_str(s.as_str(), idx + quote_offset, usize::MAX);
                        // println!("Checking sym_str '{:?}'", sym_str);
                        match check_symbol(sym_str, Symbols::Quote) {
                            Some(_) => {
                                trace!("Should break");
                                break;
                            }
                            None => (),
                        }
                        quote_offset += 1;
                    }
                    trace!("Broke from loop");
                    parsed.push(Symbols::StringLiteral(String::from(slice_str(
                        s.as_str(),
                        idx,
                        idx + quote_offset,
                    ))));
                    idx += quote_offset + 1;
                }
                s => parsed.push(s),
            }
            trace!("Parsed {:?}", parsed);
        }
    }
    info!("Lexed {} symbols", parsed.len());
    info!("----------LEXING END----------");
    parsed
}
