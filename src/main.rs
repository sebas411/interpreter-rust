use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

static mut HAS_ERROR: bool = false;

fn is_digit(c: char) -> bool {
    let ascii_c = c as u8;
    if ascii_c >= 48 && ascii_c <= 57 {
        return true;
    }
    false
}

fn error(line: usize, message: &str) {
    report(line as i32, "", message);
}

fn report(line: i32, where_is: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_is, message);
    unsafe {HAS_ERROR = true;}
}

#[derive(Clone, Debug)]
struct Token {
    token_type: String,
    lexeme: String,
    literal: String,
    line: i32
}

impl Token {
    fn new(token_type: &str, lexeme: &str, literal: &str, line: usize) -> Self {
        Self {
            token_type: token_type.into(),
            lexeme: lexeme.into(),
            literal: literal.into(),
            line: line as i32,

        }
    }

    fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    total_size: usize,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Self { source: source.into(), tokens: vec![], start: 0, current: 0, line: 1, total_size: source.chars().count() }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.total_size
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.total_size {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {self.line += 1}
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }
        self.advance();

        let value = self.source.chars().skip(self.start + 1).take(self.current - self.start - 2).collect::<String>();
        self.add_token_literal("STRING", &value);
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let my_num_string = self.source.chars().skip(self.start).take(self.current - self.start).collect::<String>();
        let my_num = my_num_string.parse::<f64>().unwrap();
        let add_decimal;
        if my_num == (my_num as i64) as f64 {
            add_decimal = ".0";
        } else {
            add_decimal = "";
        }
        self.add_token_literal("NUMBER", &format!("{}{}", my_num, add_decimal)); 
    }

    fn add_token(&mut self, token_type: &str) {
        self.add_token_literal(token_type, "null");
    }

    fn add_token_literal(&mut self, token_type: &str, literal: &str) {
        let text = self.source.chars().skip(self.start).take(self.current - self.start).collect::<String>();
        self.tokens.push(Token::new(token_type, &text, literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token("LEFT_PAREN"),
            ')' => self.add_token("RIGHT_PAREN"),
            '{' => self.add_token("LEFT_BRACE"),
            '}' => self.add_token("RIGHT_BRACE"),
            ',' => self.add_token("COMMA"),
            '.' => self.add_token("DOT"),
            '-' => self.add_token("MINUS"),
            '+' => self.add_token("PLUS"),
            ';' => self.add_token("SEMICOLON"),
            '*' => self.add_token("STAR"),
            '!' => {
                let token_type = if self.match_next('=') {"BANG_EQUAL"} else {"BANG"};
                self.add_token(token_type);
            },
            '=' => {
                let token_type = if self.match_next('=') {"EQUAL_EQUAL"} else {"EQUAL"};
                self.add_token(token_type);
            },
            '<' => {
                let token_type = if self.match_next('=') {"LESS_EQUAL"} else {"LESS"};
                self.add_token(token_type);
            },
            '>' => {
                let token_type = if self.match_next('=') {"GREATER_EQUAL"} else {"GREATER"};
                self.add_token(token_type);
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token("SLASH");
                }
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            other_char => {
                if is_digit(other_char) {
                    self.number();
                } else {
                    error(self.line, &format!("Unexpected character: {}", other_char));
                }
            },
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c.clone()
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new("EOF", "", "null", self.line));
        self.tokens.clone()
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token.to_string());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            run(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    unsafe {if HAS_ERROR {
        process::exit(65);
    }}
}
