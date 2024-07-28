use crate::error::Positioned;
use macros::*;
use strum::EnumIs;

pub struct Lexer {
    index: usize,
    program: Vec<char>,
    tokens: Vec<Positioned<Token>>,
}

impl Lexer {
    pub fn new(program: String) -> Self {
        let program = program.chars().collect();
        Lexer { index: 0, program, tokens: Vec::new() }
    }

    pub fn parse(&mut self) -> Vec<Positioned<Token>> {
        self.tokens.clear();

        while let Some(next_char) = self.next() {
            match next_char {
                '+' => match_tokens!(self, Plus, '+' => PlusPlus),
                '-' => match_tokens!(self, Minus, '-' => MinusMinus),
                '/' => self.push(Token::Slash),
                '%' => self.push(Token::Percent),

                ')' => self.push(Token::CloseParen),
                '(' => self.push(Token::OpenParen),

                '}' => self.push(Token::CloseBrace),
                '{' => self.push(Token::OpenBrace),
                '$' => {
                    let start = self.index;
                    self.next();
                    let mut string = String::new();
                    while self.peek() != Some('}') {
                        string.push(self.next_char());
                    }
                    self.next();
                    self.push_long(Token::Block(string), start)
                },

                ']' => self.push(Token::CloseBracket),
                '[' => self.push(Token::OpenBracket),

                '.' => self.push(Token::Period),
                '`' => self.push(Token::Grave),
                ',' => self.push(Token::Comma),
                '=' => self.push(Token::Equals),
                '!' => self.push(Token::Not),
                '?' => self.push(Token::Question),

                '|' => match_two!(self, '|', Or),

                '<' => match_tokens!(self, Greater, '=' => GreaterThan),
                '>' => match_tokens!(self, Lesser, '=' => LesserThan),

                ';' => {
                    while self.peek() != Some('\n') {
                        if self.next() == Some(';') {
                            break;
                        }
                    }
                }

                '^' => {
                    if self.peek().is_alphabetic() {
                        let start = self.index;
                        let word = self.next_word();
                        self.push_long(Token::Pointer(word, PointerAction::Jump), start);
                        continue;
                    }

                    self.push(Token::Carat);
                }

                '*' => {
                    if self.peek().is_alphabetic() {
                        let start = self.index;
                        let word = self.next_word();
                        self.push_long(Token::Pointer(word, PointerAction::Push), start);
                        continue;
                    }

                    self.push(Token::Asterisk);
                }

                '&' => {
                    if self.peek().is_alphabetic() {
                        let start = self.index;
                        let word = self.next_word();
                        self.push_long(Token::Pointer(word, PointerAction::Create), start);
                        continue;
                    }

                    if self.matches('&') {
                        self.next();
                        self.push(Token::And);
                    } else {
                        self.push(Token::Ampersand);
                    }
                }

                '0'..='9' => {
                    let start = self.index;
                    let mut raw_number = String::from(next_char);

                    while self.peek().is_numeric() {
                        raw_number.push(self.next().unwrap());
                    }

                    let number = raw_number.parse().unwrap();
                    self.push_long(Token::Literal(number), start);
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.index;
                    let mut word = self.next_word();
                    word.insert(0, next_char);
                    let token = self.match_keyword(&word).unwrap_or(Token::Word(word));
                    self.push_long(token, start)
                }

                '"' => {
                    let start = self.index;
                    let mut string = String::new();
                    while self.peek() != Some('"') {
                        string.push(self.next_char());
                    }
                    self.next();
                    self.push_long(Token::String(string), start)
                }

                '\'' => {
                    let start = self.index;
                    let char = self.next_char();
                    self.next();
                    self.push_long(Token::Literal(char as i64), start);
                }
                _ => {}
            };
        }

        self.tokens.clone()
    }

    pub fn next_char(&mut self) -> char {
        match self.next().unwrap() {
            '\\' => {
                match self.next().unwrap() {
                    '0' => '\0',
                    't' => '\t',
                    'n' => '\n',
                    'r' => '\r',
                    'e' => '\x1b',
                    c => c,
                }
            }
            c => c,
        }
    }

    pub fn match_keyword(&mut self, word: &str) -> Option<Token> {
        match word.to_lowercase().as_str() {
            "if" => Some(Token::Keyword(Keyword::If)),
            "else" => Some(Token::Keyword(Keyword::Else)),
            "while" => Some(Token::Keyword(Keyword::While)),
            "fn" => Some(Token::Keyword(Keyword::Function)),
            "return" => Some(Token::Keyword(Keyword::Return)),
            "break" => Some(Token::Keyword(Keyword::Break)),
            "continue" => Some(Token::Keyword(Keyword::Continue)),
            _ => None,
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(Positioned { inner: token, range: self.index..self.index })
    }

    pub fn push_long(&mut self, token: Token, start: usize) {
        self.tokens.push(Positioned { inner: token, range: start..self.index })
    }

    pub fn push_two(&mut self, token: Token) {
        self.tokens.push(Positioned { inner: token, range: self.index..self.index + 1 })
    }

    pub fn next(&mut self) -> Option<char> {
        self.index += 1;
        match self.program.len() >= self.index {
            true => Some(self.program[self.index - 1]),
            false => None,
        }
    }

    pub fn peek(&self) -> Option<char> {
        match self.program.len() > self.index {
            true => Some(self.program[self.index]),
            false => None,
        }
    }

    pub fn matches(&self, c: char) -> bool {
        match self.peek() {
            Some(peeked) => peeked == c,
            None => false,
        }
    }

    pub fn next_word(&mut self) -> String {
        let mut word = String::new();

        while self.peek().map(|c| c.is_alphabetic() || c == '_').unwrap_or(false) {
            word.push(self.next().unwrap());
        }

        word
    }
}

trait OptionChar {
    fn is_numeric(&self) -> bool;
    fn is_alphabetic(&self) -> bool;
}

impl OptionChar for Option<char> {
    fn is_numeric(&self) -> bool {
        match self {
            Some(c) => c.is_numeric(),
            None => false,
        }
    }

    fn is_alphabetic(&self) -> bool {
        match self {
            Some(c) => c.is_alphabetic(),
            None => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    If,
    Else,
    While,
    Function,
    Return,
    Break,
    Continue,
}

#[derive(Debug, Clone, EnumIs, PartialEq)]
pub enum Token {
    Literal(i64),
    Word(String),
    Keyword(Keyword),
    Block(String),

    OpenParen,
    CloseParen,

    OpenBrace,
    CloseBrace,

    OpenBracket,
    CloseBracket,

    Percent,
    Carat,

    And,
    Or,

    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Asterisk,
    Slash,
    Ampersand,

    Equals,
    Greater,
    GreaterThan,
    Lesser,
    LesserThan,
    Not,
    Question,

    Period,
    Comma,
    Grave,
    String(String),

    Pointer(String, PointerAction),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PointerAction {
    Jump,
    Create,
    Push,
}

pub mod macros {
    macro_rules! match_tokens {
        ($s:ident, $base_token:ident, $($extra_char:literal => $extra_token:ident),*) => {
            {
                let mut base = true;
                $(
                    if $s.matches($extra_char) {
                        $s.push_two(Token::$extra_token);
                        $s.next();
                        base = false;
                    }
                )*
                if base {
                    $s.push(Token::$base_token);
                }
            }
        }
    }

    macro_rules! match_two {
        ($s:ident, $add_char:expr, $token:ident) => {{
            if $s.matches($add_char) {
                $s.push_two(Token::$token)
            }
        }};
    }

    pub(crate) use match_tokens;
    pub(crate) use match_two;
}
