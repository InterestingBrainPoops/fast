use crate::parser::Token;

pub struct Runner {
    tokens: Vec<Token>,
    memory: [u8; 30_000],
    mem_pointer: usize,
    instr_pointer: usize,
}

impl Runner {
    pub fn new(tokens: Vec<Token>) -> Runner {
        Runner {
            tokens,
            memory: [0; 30_000],
            mem_pointer: 0,
            instr_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        while self.instr_pointer < self.tokens.len() {
            match self.tokens[self.instr_pointer] {
                Token::Clear => {
                    self.memory[self.mem_pointer] = 0;
                    self.instr_pointer += 1;
                }
                Token::OpenBracket(x) => {
                    if self.memory[self.mem_pointer] == 0 {
                        self.instr_pointer = x + 1;
                    } else {
                        self.instr_pointer += 1;
                    }
                }
                Token::CloseBracket(x) => {
                    if self.memory[self.mem_pointer] != 0 {
                        self.instr_pointer = x + 1;
                    } else {
                        self.instr_pointer += 1;
                    }
                }
                Token::Add(x) => {
                    self.instr_pointer += 1;
                    self.memory[self.mem_pointer] += x;
                }
                Token::Subtract(x) => {
                    self.instr_pointer += 1;
                    self.memory[self.mem_pointer] -= x;
                }
                Token::Shift(x) => {
                    self.instr_pointer += 1;
                    self.mem_pointer += x as usize;
                }
                Token::Print => {
                    self.instr_pointer += 1;
                    print!("{}", self.memory[self.mem_pointer] as char);
                }
                Token::Get => todo!(),
            }
        }
    }
}
