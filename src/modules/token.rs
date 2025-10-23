#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: String,
    pub lexeme: String,
    pub literal: String,
    _line: i32
}

impl Token {
    pub fn new(token_type: &str, lexeme: &str, literal: &str, line: usize) -> Self {
        Self {
            token_type: token_type.into(),
            lexeme: lexeme.into(),
            literal: literal.into(),
            _line: line as i32,

        }
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
