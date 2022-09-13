use std::ops::Add;

#[derive(Debug)]
pub struct CodeWhitespace {
    cur_index: usize,
    snippets: Vec<CodeChunk>,
}

#[derive(Debug)]
pub struct CodeChunk {
    pub(crate) str_data: String,
    is_whitespace: bool,
    newlines: i8,
}

pub fn check_is_whitespace(ch: &str) -> bool {
    return if ch.len() == 1 {
        if ch == " " || ch == "\n" || ch == "\r" || ch == "\t" || ch == "" {
            true
        } else {
            false
        }
    } else {
        for i in 0..ch.len() {
            if !check_is_whitespace(&ch[i..i + 1]) {
                return false;
            }
        }
        true
    };
}

impl<'a> CodeWhitespace {
    pub fn new(snippet: &str) -> CodeWhitespace {
        let mut s: Vec<CodeChunk> = Vec::new();
        let mut index: usize = 0;
        let mut is_whitespace = false;

        let mut start_index = index;

        while index < snippet.len() {
            let ch: &str = &snippet[index..index + 1];
            if check_is_whitespace(ch) {
                if !is_whitespace {
                    is_whitespace = true;

                    let sn = &snippet[start_index..index];
                    let mut ws = false;
                    if sn == "" {
                        ws = true;
                    }
                    s.push(CodeChunk::new(String::from(sn), ws));
                    start_index = index;
                }
            } else {
                if is_whitespace {
                    is_whitespace = false;
                    s.push(CodeChunk::new(
                        String::from(&snippet[start_index..index]),
                        true,
                    ));
                    start_index = index;
                }
            }
            index += 1;
        }
        // Handle EOF
        let ch: &str = &snippet[start_index..index];
        s.push(CodeChunk::new(String::from(ch), check_is_whitespace(ch)));

        return CodeWhitespace {
            cur_index: 0,
            snippets: s,
        };
    }

    pub fn print_all(&self) {
        for snip in &self.snippets {
            print!("{}", snip.str_data);
        }
    }

    pub fn print_from_current(&self) {
        for i in self.cur_index..self.cur_index + 4 {
            println!("Chunk '{:?}'", self.snippets[i]);
        }
    }

    pub fn print_no_whitespace(&self) {
        for snip in &self.snippets {
            if !snip.is_whitespace {
                println!("'{}'", snip.str_data);
            }
        }
    }

    pub fn print(&self) {
        for snip in &self.snippets {
            println!("CodeChunk{{'{}', '{}'}}", snip.str_data, snip.is_whitespace);
        }
    }

    pub fn newline_count(self) -> i8 {
        let mut count = 0;
        for snip in self.snippets.iter() {
            count += snip.newlines;
        }
        return count + 1; // Add one since lines start at 0
    }

    pub fn get_current_index(&self) -> usize {
        return self.cur_index;
    }

    pub fn get_line_no_from_index(&self, index: usize) -> i8 {
        let mut count = 0;

        for i in 0..index {
            count += self.snippets[i].newlines;
        }
        return count + 1;
    }

    pub fn get_current_line_no(&self) -> i8 {
        return self.get_line_no_from_index(self.cur_index);
    }

    pub fn indexes_to_raw(&self, start: usize, end: usize) -> String {
        let mut e = end;

        let mut s = String::new();

        if start > self.snippets.len() || end < start {
            return String::from("");
        }

        if end > self.snippets.len() {
            e = self.snippets.len();
        }
        for i in start..e {
            s = s.add(self.snippets[i].str_data.as_str());
        }
        return s;
    }

    pub fn get(&mut self) -> String {
        loop {
            if !self.snippets[self.cur_index].is_whitespace {
                return String::from(&self.snippets[self.cur_index].str_data);
            } else {
                println!("Skipping '{}'", self.snippets[self.cur_index].str_data);
                self.increment();
                if self.cur_index > self.snippets.len() {
                    // TODO Add error handling
                    panic!("Unable to continue parsing, reached EOF! Started looking for tokens on line {}",
                           self.get_current_line_no()
                    );
                }
            }
        }
    }

    pub fn peek_multiple(&mut self, times: i8) -> bool {
        // Ensure the next 'times' tokens are non-whitespace
        for i in 0..times {
            if self.peek_by_idx(self.cur_index + i as usize) {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }

    fn peek_by_idx(&mut self, i: usize) -> bool {
        let mut idx = i;
        loop {
            return match self.snippets.get(idx) {
                Some(x) => {
                    if x.is_whitespace {
                        idx += 1;
                        continue;
                    } else {
                        true
                    }
                }
                None => false,
            };
        }
    }

    pub fn peek(&mut self) -> bool {
        return self.peek_by_idx(self.cur_index);
    }

    pub fn split_current(&mut self, f: &str, s: &str) {
        let mut first = f;
        let mut second = s;
        println!("split f '{}' s '{}'", f, s);

        if self.peek() {
            self.snippets.remove(self.cur_index);
            self.snippets.insert(
                self.cur_index,
                CodeChunk::new(
                    String::from(second), // Add second before on same index
                    check_is_whitespace(second),
                ),
            );
            self.snippets.insert(
                self.cur_index,
                CodeChunk::new(String::from(first), check_is_whitespace(first)),
            );
        }
    }

    pub fn increment(&mut self) {
        self.cur_index += 1;
    }

    pub fn decrement(&mut self) {
        //Used to roll back a line before showing an error message
        self.cur_index -= 1;
    }

    pub fn increment_to_newline(&mut self) {
        loop {
            match self.snippets.get(self.cur_index) {
                Some(cchunk) => {
                    if cchunk.newlines != 0 {
                        self.increment();
                        break;
                    } else {
                        self.increment();
                    }
                }
                None => break,
            }
        }
    }
}

impl CodeChunk {
    pub fn new(data: String, is_whitespace: bool) -> CodeChunk {
        let mut count = 0;
        for ch in data.chars() {
            if ch == '\n' {
                count += 1;
            }
        }

        return CodeChunk {
            str_data: data,
            is_whitespace,
            newlines: count,
        };
    }

    pub fn is_whitespace(&self) -> bool {
        return self.is_whitespace;
    }

    pub fn get_newlines(&self) -> i8 {
        return self.newlines;
    }

    pub fn copy(&self) -> CodeChunk {
        return CodeChunk {
            str_data: String::from(&self.str_data),
            is_whitespace: self.is_whitespace,
            newlines: self.newlines,
        };
    }
}
