use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Token {
    IDENT(String),
    OASSIGN,
    NUMBER(i64),
    STRING(String),
    KW_INT,
    DECIMAL(f64),
    EOF,
}

impl Token {
    pub fn new(buf: String) -> Self {
        let trim = buf.trim();
        if trim.starts_with("\"") {
            return Token::STRING(trim[1..buf.len() - 1].to_string());
        }
        match trim {
            "int" => return Token::KW_INT,
            "=" => return Token::OASSIGN,
            _ => {}
        }
        match i64::from_str_radix(trim, 10) {
            Ok(s) => return Token::NUMBER(s),
            Err(..) => match f64::from_str(trim) {
                Ok(f) => return Token::DECIMAL(f),
                Err(..) => return Token::IDENT(trim.to_owned()),
            },
        }
    }
}

#[derive(Debug)]
pub struct LexerResult {
    pub token: Token,
    pub line: usize,
    pub file: String,
    pub pos: usize,
}

impl LexerResult {
    pub fn new(tok: String, lex: &Lexer) -> Self {
        LexerResult {
            token: Token::new(tok),
            line: lex.line(),
            file: lex.file.clone(),
            pos: lex.pos() - 1,
        }
    }
}

pub struct Lexer<'a> {
    line: usize,
    pos: usize,
    index: usize,
    pub file: String,
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(file: &str, input: &'a [u8]) -> Self {
        Self {
            input,
            index: 0,
            pos: 1,
            file: file.to_owned(),
            line: 1,
        }
    }

    pub fn next(&mut self) -> LexerResult {
        let mut instr = false;
        let mut new_line = false;
        let mut buf = String::new();
        while self.index < self.input.len() {
            let c = self.input[self.index] as char;
            match c {
                ' ' | '\n' => {
                    if c == '\n' {
                        new_line = true;
                    }
                    if !instr {
                        if buf.len() != 0 {
                            self.pos += 1;
                            self.index += 1;
                            if new_line {
                                let lex = LexerResult::new(buf, self);
                                self.line += 1;
                                self.pos = 1;
                                return lex;
                            }
                            return LexerResult::new(buf, self);
                        }
                    }
                    buf.push(c);
                }
                '\'' | '\"' => {
                    if instr {
                        instr = false;
                        buf.push(c);
                        self.pos += 1;
                        self.index += 1;
                        return LexerResult::new(buf, self);
                    }
                    buf.push(c);
                    instr = true;
                }
                _ => buf.push(c),
            }
            self.pos += 1;
            self.index += 1;
            if new_line {
                new_line = false;
                self.line += 1;
                self.pos = 1;
            }
        }
        if instr {
            panic!(
                "Error: In File {} at line {} position {}: Unclosed string literal",
                self.file, self.line, self.pos
            );
        }

        LexerResult {
            token: Token::EOF,
            line: self.line,
            file: self.file.clone(),
            pos: self.pos,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}
