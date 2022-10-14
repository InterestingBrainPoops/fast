#[derive(Debug, Clone)]
pub enum Token {
    OpenBracket(usize),
    CloseBracket(usize),
    Add(u8),
    Subtract(u8),
    Shift(i32),
    Clear,
    Print,
    Get,
}
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { tokens: vec![] }
    }

    pub fn parse(&mut self, path: String) {
        let mut data = String::new();
        let f = File::open(path).expect("Unable to open file");
        let mut br = BufReader::new(f);
        br.read_to_string(&mut data).expect("Unable to read string");
        let mut open_stack = Vec::with_capacity(50);
        let mut cursor = 0;
        let data = data.chars().collect::<Vec<char>>();
        while cursor < data.len() {
            let current = data[cursor];
            match current {
                '[' => {
                    if data.get(cursor + 1) == Some(&'-') && data.get(cursor + 2) == Some(&']') {
                        self.tokens.push(Token::Clear);
                        cursor += 3;
                        continue;
                    }
                    self.tokens.push(Token::OpenBracket(self.tokens.len()));
                    open_stack.push(self.tokens.len() - 1);
                    cursor += 1;
                }
                ']' => {
                    // dbg!(open_stack.clone());
                    let matching = open_stack.pop().unwrap();
                    self.tokens[matching] = Token::OpenBracket(self.tokens.len());
                    self.tokens.push(Token::CloseBracket(matching));
                    cursor += 1;
                }
                '+' => {
                    let mut accum = 1;
                    cursor += 1;
                    while cursor < data.len() && data[cursor] == '+' {
                        cursor += 1;
                        accum += 1;
                    }
                    self.tokens.push(Token::Add(accum));
                }
                '-' => {
                    let mut accum = 1;
                    cursor += 1;
                    while cursor < data.len() && data[cursor] == '-' {
                        cursor += 1;
                        accum += 1;
                    }
                    self.tokens.push(Token::Subtract(accum));
                }
                '>' => {
                    let mut accum = 1;
                    cursor += 1;
                    while cursor < data.len() && data[cursor] == '>' {
                        cursor += 1;
                        accum += 1;
                    }
                    self.tokens.push(Token::Shift(accum));
                }
                '<' => {
                    let mut accum = 1;
                    cursor += 1;
                    while cursor < data.len() && data[cursor] == '<' {
                        cursor += 1;
                        accum += 1;
                    }
                    self.tokens.push(Token::Shift(-accum));
                    match (
                        self.tokens[self.tokens.len() - 2],
                        self.tokens[self.tokens.len() - 1],
                    ) {}
                }
                '.' => {
                    cursor += 1;
                    self.tokens.push(Token::Print)
                }
                ',' => {
                    cursor += 1;
                    self.tokens.push(Token::Get)
                }
                _ => {
                    cursor += 1;
                }
            }
        }
    }
}
