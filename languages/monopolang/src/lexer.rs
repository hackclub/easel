#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Grouping
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,

    // Logical operators
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,
    True,
    False,

    // Keywords
    And,
    Or,
    If,
    Else,
    Then,
    End,
    While,
    Range,
    From,
    To,
    By,
    Do,
    Procedure,
    Call,
    Set,
    Print,

    // Economy keywords
    Gamble,
    Sell,
    Buy,
    Loan,
    Repay,
    Work,

    // Special
    At,
    Dollar,
    Arrow,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    line: u32,
    column: u32,
    start: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            line: 1,
            column: 0,
            start: 0,
            current: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });

        return self.tokens.clone();
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            ' ' | '\r' | '\t' => (),
            '\n' => {
                self.line += 1;
                self.column = 0; // Reset column to 0 because we increment it each iteration
            }
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '[' => self.add_token(TokenType::LeftBracket),
            ']' => self.add_token(TokenType::RightBracket),
            '@' => self.add_token(TokenType::At),
            '$' => self.add_token(TokenType::Dollar),
            '+' => self.add_token(TokenType::Plus),
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenType::Arrow)
                } else {
                    self.add_token(TokenType::Minus)
                }
            }
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '\0' => (),
            _ => self.error(&format!("Unexpected character: {}", c)),
        }
    }

    fn identifier(&mut self) {
        while self.is_identifier() {
            self.advance();
        }

        let kind = match self.source[self.start..self.current].to_string().as_str() {
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "then" => TokenType::Then,
            "end" => TokenType::End,
            "while" => TokenType::While,
            "range" => TokenType::Range,
            "from" => TokenType::From,
            "to" => TokenType::To,
            "by" => TokenType::By,
            "do" => TokenType::Do,
            "proc" => TokenType::Procedure,
            "call" => TokenType::Call,
            "set" => TokenType::Set,
            "print" => TokenType::Print,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "gamble" => TokenType::Gamble,
            "buy" => TokenType::Buy,
            "sell" => TokenType::Sell,
            "loan" => TokenType::Loan,
            "repay" => TokenType::Repay,
            "work" => TokenType::Work,
            _ => TokenType::Identifier,
        };

        self.add_token(kind);
    }

    fn string(&mut self) {
        self.start += 1; // Skip the opening quote

        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error("Unterminated string");
        }
        self.add_token(TokenType::String);
        self.advance(); // Consume the closing quote
    }

    fn number(&mut self) {
        while self.is_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit() {
            self.advance();
            while self.is_digit() {
                self.advance();
            }
        }
        self.add_token(TokenType::Number);
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current as usize - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    fn add_token(&mut self, kind: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        let length = text.len() as u32;

        self.tokens.push(Token {
            kind,
            lexeme: text,
            line: self.line,
            column: self.column - length + 1, // Subtract length to get the start of the token
        });
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self) -> bool {
        self.peek().is_digit(10)
    }

    fn is_alpha(&self) -> bool {
        self.peek().is_alphabetic()
    }

    fn is_identifier(&self) -> bool {
        self.peek().is_alphabetic() || self.peek() == '_'
    }

    fn error(&self, message: &str) -> ! {
        panic!("Error at <{}:{}>: {}", self.line, self.column, message);
    }
}
