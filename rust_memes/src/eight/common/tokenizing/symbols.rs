use std::collections::HashMap;
use strum_macros::EnumString;

// https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
macro_rules! symbols {
    ($(($name:ident, $ch:literal, $($cat:expr),+)),+) => {
        as_item! {
            #[derive(Debug, PartialEq, EnumString, Clone)]
            pub enum Symbols {
                LiteralSymb(String),
                StringLiteral(String),
                $(
                    #[strum(serialize = $ch)]
                    $name
                ),+
            }
        }

        impl Symbols {
            pub fn get_all_strs() -> Vec<&'static str> {
                vec![$($ch),+]
            }

            pub fn get_all() -> Vec<Self> {
                vec![$(Self::$name),+]
            }

            pub fn get_symbol_hashmap() -> HashMap<SymbolType, Vec<Self>> {
                let mut h:HashMap<SymbolType, Vec<Self>> = HashMap::new();
                $($(
                for idx in 0..$cat.len() {
                    let el = &$cat[idx];

                    if h.contains_key(&el) {
                        let en = h.remove_entry(&el);
                        match en {
                            Some((typ, mut ops)) => {
                                ops.push(Self::$name);
                                h.insert($cat[idx], ops);
                            },
                            None => {
                                h.insert($cat[idx], vec![Self::$name]);
                            }
                        }
                    } else {
                        h.insert($cat[idx], vec![Self::$name]);
                    }

                }

                )+)+

                return h;
            }

            pub fn get_symbols_by_type(typ: SymbolType) -> Vec<Self> {
                return Self::get_symbol_hashmap().get(&typ).unwrap().to_vec();
            }

            pub fn len(&self) -> usize {
                return self.to_str().len();
            }

            pub fn sort_by_longest(mut symbs: Vec<Symbols>) -> Vec<Symbols> {
                let mut s = Vec::new();
                let mut processed = 0;
                let mut sym_len = symbs.len();
                while processed < sym_len {

                    let mut cur_biggest = 0;
                    let mut cur_idx = 0;
                    let mut found_idx = 0;

                    for symb in &symbs {
                        let symb_len = symb.len();
                        if symb_len > cur_biggest {
                            cur_biggest = symb.len();
                            found_idx = cur_idx;
                        }
                        cur_idx += 1;
                    }
                    let sym = symbs.remove(found_idx);
                    s.push(sym);
                    processed += 1;
                }
                s
            }

            pub fn to_str(&self) -> String {
                match self {
                    Self::LiteralSymb(s) => {
                        return s.clone();
                    },
                    Self::StringLiteral(s) => {
                      return s.clone();
                    },
                    $(
                    Self::$name => {
                        return String::from($ch);
                    }
                    ),+
                }
            }

            pub fn to_strs(typ: SymbolType) -> Vec<String> {
                let mut v = Vec::new();
                let sv = Self::get_symbols_by_type(typ);
                for el in sv {
                    v.push(el.to_str());
                }
                return v;
            }

        }
    };
}

macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum SymbolType {
    Operator,
    Assign,
    Whitespace,
    Newline,
    Accessor,
    Keyword,
    Bracket,
    Delimiter,
    Comment,
    StringQuote,
}
symbols![
    // Operators
    (Add, "+", [SymbolType::Operator]),
    (Sub, "-", [SymbolType::Operator]),
    (Exponent, "**", [SymbolType::Operator]),
    (Multiply, "*", [SymbolType::Operator]),
    (Divide, "/", [SymbolType::Operator]),
    (Modulus, "%", [SymbolType::Operator]),
    // Assignment Operators
    (Equal, "=", [SymbolType::Assign]),
    // Boolean Operators
    (EqualityCheck, "==", [SymbolType::Operator]),
    // Accessors
    (ClassAccessor, "::", [SymbolType::Accessor]),
    (ValueAccessor, ".", [SymbolType::Accessor]),
    // Var create
    (Let, "let", [SymbolType::Keyword]),
    // End Expression Delimiter
    (Semicolon, ";", [SymbolType::Delimiter]),
    (Comma, ",", [SymbolType::Delimiter]),
    // Brackets
    (ParenOpen, "(", [SymbolType::Bracket]),
    (ParenClose, ")", [SymbolType::Bracket]),
    (BracketOpen, "[", [SymbolType::Bracket]),
    (BracketClose, "]", [SymbolType::Bracket]),
    // String quoting
    (Quote, "\"", [SymbolType::StringQuote]),
    // Whitespace
    (NewLine, "\n", [SymbolType::Whitespace, SymbolType::Newline]),
    (LineReturn, "\r", [SymbolType::Whitespace, SymbolType::Newline]),
    (Space, " ", [SymbolType::Whitespace]),
    // Whitespace - Newlines
    (WindowsNewLine, "\r\n", [SymbolType::Whitespace, SymbolType::Newline]),
    // Comments
    (SingleComment, "//", [SymbolType::Comment]),
    (MultiCommentStart, "/*", [SymbolType::Comment]),
    (MultiCommentEnd, "*/", [SymbolType::Comment])
];
